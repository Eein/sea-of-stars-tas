//! The [`Action`] trait — identity, appraisal hooks, and the default
//! step-driving behaviour shared by every action.

use data::prelude::PlayerPartyCharacter;
use joystick::common::JoystickBtnInterface;

use super::menu::{self, cancel_press, confirm_press, tap_press};
use super::step::{ActionCtx, ActionStep, DriveResult, StepOutcome};
use crate::combat::damage;
use crate::control::SosAction;
use crate::memory::combat_manager::{
    CombatDamageType, CombatEnemy, CombatManagerData, CombatMove, CombatPlayer,
};

/// How the timed input is landed during [`ActionStep::Attacking`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimingType {
    /// No timed input.
    None,
    /// A single crisp tap when the window opens.
    OneHit,
    /// Hold, then release when the window fires. Charge skills own their QTE:
    /// they override [`Action::execute_attacking`] with their own state machine
    /// (see `sunball.rs`) and report it live via [`Action::qte_in_flight`].
    Charge,
    /// One tap per hit across a multi-hit animation.
    MultiHit,
}

/// What an action targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetType {
    None,
    Player,
    Enemy,
    Aoe,
    All,
}

/// The resource an action spends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillResource {
    None,
    Mana,
    ComboPoints,
    UltimateGuage,
}

/// Which top-level battle command the action lives under.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleCommand {
    Attack,
    Skill,
    Combo,
    Item,
}

impl BattleCommand {
    /// The command's index on the battle-command ring, matching
    /// `battle_command_index` in memory (Attack=0, Skill=1, Combo=2, Item=3).
    pub fn ring_index(self) -> i64 {
        match self {
            BattleCommand::Attack => 0,
            BattleCommand::Skill => 1,
            BattleCommand::Combo => 2,
            BattleCommand::Item => 3,
        }
    }

    /// The game's `BattleCommandDefinition` class name for this command,
    /// matched against the live command ring and a fighter's
    /// `disabled_commands` set. (The skill command's class is
    /// `SelectSpecialMoveBattleCommand` — there is no `SkillBattleCommand`.)
    pub fn class_name(self) -> &'static str {
        match self {
            BattleCommand::Attack => "BasicAttackBattleCommand",
            BattleCommand::Skill => "SelectSpecialMoveBattleCommand",
            BattleCommand::Combo => "ComboBattleCommand",
            BattleCommand::Item => "ItemsBattleCommand",
        }
    }
}

/// A combat action. One `impl` per skill; identity + usability + damage power
/// the appraiser, and the `execute_*` steps drive the executor.
#[allow(unused_variables)]
pub trait Action {
    // --- Identity ---

    /// The character who performs the action.
    fn character(&self) -> PlayerPartyCharacter;
    /// The move's `combatMoveId` (matches `CombatMove.move_id`). Borrows `self`
    /// so runtime-discovered moves (combos) can return a stored name.
    fn internal_name(&self) -> &str;
    fn battle_command(&self) -> BattleCommand {
        BattleCommand::Skill
    }
    fn target_type(&self) -> TargetType {
        TargetType::Enemy
    }
    fn timing_type(&self) -> TimingType {
        // Most skills land a single timed hit; only a few charge (Sunball) or
        // hit multiple times (Moonerang). Those override this.
        TimingType::OneHit
    }
    fn resource(&self) -> SkillResource {
        SkillResource::Mana
    }
    fn cost(&self) -> u32 {
        0
    }
    fn damage_types(&self) -> Vec<CombatDamageType> {
        Vec::new()
    }
    /// Whether this action's own timed input — a QTE it drives during its
    /// [`Attacking`](ActionStep::Attacking) step — is live on screen. The
    /// executor uses this to (re-)latch the action straight into execution
    /// when its QTE appears without having been committed through the menus.
    /// Actions with such a QTE (charge skills) override this.
    fn qte_in_flight(&self, cmd: &CombatManagerData) -> bool {
        false
    }

    // --- Appraisal (utility AI) ---

    /// Whether the action can be used this turn: its character is alive and
    /// on-screen, the move is loaded (unlocked), and the resource is affordable.
    fn is_usable(&self, cmd: &CombatManagerData) -> bool {
        let Some(player) = self.player(cmd) else {
            return false;
        };
        if player.dead || !player.enabled {
            return false;
        }
        if !self.move_available(cmd) {
            return false;
        }
        match self.resource() {
            SkillResource::None => true,
            SkillResource::Mana => player.current_mp >= self.mp_cost(cmd),
            SkillResource::ComboPoints => cmd.combo_points >= self.cost(),
            SkillResource::UltimateGuage => cmd.ultimate_progress >= 1.0,
        }
    }

    /// The move's live MP cost, read from its `skillPointCost` in the move
    /// definition, falling back to the module's declared [`cost`](Self::cost)
    /// when the move isn't present in memory.
    fn mp_cost(&self, cmd: &CombatManagerData) -> u32 {
        self.find_move(cmd)
            .and_then(|m| m.skill_point_cost)
            .unwrap_or_else(|| self.cost())
    }

    /// Estimated damage against `enemy` at the highest damage roll — the
    /// optimistic value the appraiser ranks by. Actions override
    /// [`estimate_damage_at`](Self::estimate_damage_at), not this.
    fn estimate_damage(
        &self,
        cmd: &CombatManagerData,
        player: &CombatPlayer,
        enemy: &CombatEnemy,
    ) -> f32 {
        let (_, max_roll) = cmd.damage_roll_bounds();
        self.estimate_damage_at(cmd, player, enemy, max_roll)
    }

    /// Estimated damage against `enemy` with `roll` as the random damage term.
    /// The appraiser evaluates both roll bounds: the max for the damage value,
    /// the min to gate kill bonuses on kills that land at *any* roll (the TAS
    /// can't steer the roll). Default is a multiple of the basic attack;
    /// damaging skills override this with their real formula.
    fn estimate_damage_at(
        &self,
        cmd: &CombatManagerData,
        player: &CombatPlayer,
        enemy: &CombatEnemy,
        roll: f32,
    ) -> f32 {
        let (base, timed) = damage::basic_attack_damage(player, enemy, roll);
        ((base + timed) * 1.5).floor()
    }

    // --- Execution steps (drive one frame of the action's step machine) ---
    //
    // The executor dispatches the action's current [`ActionStep`] to the
    // matching method, which reads live memory, presses at most one input, and
    // returns a [`StepOutcome`] telling the executor to stay, advance, or drop
    // the action. The defaults implement the standard menu-driving behaviour
    // (delegating the mechanics to the shared `menu` drivers and routing on
    // their [`DriveResult`]), so a normal action — basic attack, most skills,
    // combos — overrides nothing. An ability with its own mechanics (Sunball's
    // charge) overrides the step it owns and keeps that logic, and any state it
    // needs, in its own file; the methods take `&mut self` for exactly that.

    /// [`Boosting`](ActionStep::Boosting): absorb Live Mana until the attacker
    /// holds the charges the appraisal counted on. On the command ring, hold
    /// Boost and tap Confirm — the game merges 5 small mana into one charge per
    /// tap (up to 3) — verifying each landed charge against the live
    /// `mana_charge_count` instead of firing blind. Releases Boost and moves to
    /// the command ring when the target is met, the ground pool runs dry, or
    /// the tap budget is spent (never wedge the turn on a boost).
    fn execute_boosting(&mut self, ctx: &mut ActionCtx) -> StepOutcome {
        /// Delay after Boost is held (and between taps) so the absorb overlay
        /// is up and each Confirm lands as an absorb, not a menu press.
        const ABSORB_SETTLE: f64 = 0.3;
        /// Confirm taps to spend absorbing before giving up: 3 charges plus
        /// retries for taps the game ignored.
        const MAX_ABSORB_TAPS: u32 = 8;

        let have = self.player(ctx.cmd).map_or(0, |p| p.mana_charge_count);
        let pool_left = ctx.cmd.live_mana.can_yield_charge();
        if have >= ctx.want_mana_charges || !pool_left || ctx.scratch.taps >= MAX_ABSORB_TAPS {
            if have < ctx.want_mana_charges {
                // Fail-open: proceed unboosted rather than wedge, but say so —
                // the appraisal's damage assumed these charges, so the hit
                // will land below the reported estimate.
                log::warn!(
                    "boosting gave up at {have}/{} charges (pool: {} small, {} big, {} taps) — attacking unboosted below the estimate",
                    ctx.want_mana_charges,
                    ctx.cmd.live_mana.small,
                    ctx.cmd.live_mana.big,
                    ctx.scratch.taps,
                );
            }
            ctx.gamepad.release(&SosAction::Boost);
            ctx.scratch.timer = 0.0;
            ctx.scratch.taps = 0;
            return StepOutcome::Advance(ActionStep::SelectingCommand);
        }
        ctx.gamepad.press(&SosAction::Boost);
        if !ctx.btn.done() {
            ctx.btn.update(ctx.gamepad, ctx.dt);
        } else if ctx.scratch.timer >= ABSORB_SETTLE {
            // One absorb tap, then wait out the settle again before the next.
            *ctx.btn = tap_press(SosAction::Confirm);
            ctx.scratch.taps += 1;
            ctx.scratch.timer = 0.0;
        }
        StepOutcome::Stay
    }

    /// [`SelectingCommand`](ActionStep::SelectingCommand): navigate the
    /// battle-command ring to this action's command and confirm it. Skills and
    /// combos open a submenu ([`SelectingAbility`](ActionStep::SelectingAbility));
    /// everything else goes straight to target select.
    fn execute_selecting_command(&mut self, ctx: &mut ActionCtx) -> StepOutcome {
        if !ctx.btn.done() {
            ctx.btn.update(ctx.gamepad, ctx.dt);
            return StepOutcome::Stay;
        }
        if ctx.in_submenu() {
            // We're in an ability submenu, not the ring — back out first.
            *ctx.btn = cancel_press();
            ctx.scratch.timer = 0.0;
            return StepOutcome::Stay;
        }
        // The ring's live entries decide the slot: removed commands (e.g. the
        // Elder Mist trials stripping Skill/Combo) shift the remaining
        // indices, so the fixed mapping only serves as a fallback when the
        // ring isn't readable.
        let want_command = ctx
            .cmd
            .battle_command_ring
            .iter()
            .position(|(name, _)| name == self.battle_command().class_name())
            .map(|i| i as i64)
            .unwrap_or_else(|| self.battle_command().ring_index());
        if ctx.cmd.battle_command_index == Some(want_command) {
            *ctx.btn = confirm_press();
            ctx.scratch.timer = 0.0;
            return match self.battle_command() {
                BattleCommand::Skill | BattleCommand::Combo => {
                    StepOutcome::Advance(ActionStep::SelectingAbility)
                }
                _ => StepOutcome::Advance(ActionStep::SelectingTarget),
            };
        }
        // Step the command cursor toward the desired command.
        *ctx.btn = tap_press(SosAction::MenuDown);
        StepOutcome::Stay
    }

    /// [`SelectingAbility`](ActionStep::SelectingAbility): navigate the ability
    /// submenu (combo on the battle selector, skill on the skill selector) to
    /// this action's move and confirm it. Backs out to
    /// [`SelectingCommand`](ActionStep::SelectingCommand) if the move can't be
    /// found/cast, advances to target select once committed.
    fn execute_selecting_ability(&mut self, ctx: &mut ActionCtx) -> StepOutcome {
        let drive = match self.battle_command() {
            BattleCommand::Combo => menu::drive_combo_submenu(self.internal_name(), ctx),
            _ => menu::drive_skill_submenu(self.internal_name(), ctx),
        };
        match drive {
            DriveResult::Ok => StepOutcome::Advance(ActionStep::SelectingTarget),
            DriveResult::Wait => StepOutcome::Stay,
            DriveResult::Error => StepOutcome::Advance(ActionStep::SelectingCommand),
        }
    }

    /// [`SelectingTarget`](ActionStep::SelectingTarget): move the enemy cursor
    /// onto the action's target and confirm (the shared cursor driver bails to
    /// the default target if the wanted one can't be reached). Recovers to the
    /// menu we're actually in if the previous confirm didn't take.
    fn execute_selecting_target(&mut self, ctx: &mut ActionCtx) -> StepOutcome {
        if !ctx.btn.done() {
            ctx.btn.update(ctx.gamepad, ctx.dt);
            return StepOutcome::Stay;
        }
        if ctx.in_submenu() {
            // Still in an ability submenu — handle it there.
            ctx.scratch.timer = 0.0;
            return StepOutcome::Advance(ActionStep::SelectingAbility);
        }
        if ctx.cmd.battle_command_has_focus {
            // Still on the command ring — the confirm didn't take; retry.
            ctx.scratch.timer = 0.0;
            return StepOutcome::Advance(ActionStep::SelectingCommand);
        }
        match menu::drive_target_cursor(ctx) {
            DriveResult::Ok => {
                *ctx.btn = confirm_press();
                ctx.scratch.timer = 0.0;
                StepOutcome::Advance(ActionStep::ConfirmingTarget)
            }
            DriveResult::Wait | DriveResult::Error => StepOutcome::Stay,
        }
    }

    /// [`ConfirmingTarget`](ActionStep::ConfirmingTarget): drive the
    /// target-confirm press; once it lands, arm the timed-hit edge detector and
    /// enter [`Attacking`](ActionStep::Attacking).
    fn execute_confirming_target(&mut self, ctx: &mut ActionCtx) -> StepOutcome {
        if ctx.btn.update(ctx.gamepad, ctx.dt) {
            ctx.gamepad.release(&SosAction::Confirm);
            ctx.scratch.timer = 0.0;
            ctx.scratch.last_timed_ready = false;
            return StepOutcome::Advance(ActionStep::Attacking);
        }
        StepOutcome::Stay
    }

    /// [`Attacking`](ActionStep::Attacking): land the action's timed input for
    /// the whole animation, per its [`timing_type`](Self::timing_type). Resolves
    /// ([`Done`](StepOutcome::Done)) when a menu returns (the action ended); a
    /// stuck window triggers a mash.
    fn execute_attacking(&mut self, ctx: &mut ActionCtx) -> StepOutcome {
        // A menu returning means the action resolved — release and let the
        // executor pick the next action.
        if ctx.menus_returned() {
            ctx.gamepad.release(&SosAction::Confirm);
            ctx.scratch.timer = 0.0;
            return StepOutcome::Done;
        }
        let timed_ready = ctx.timed_ready();
        match self.timing_type() {
            TimingType::OneHit | TimingType::MultiHit => {
                // Tap Confirm on each rising edge of the window.
                ctx.gamepad.release(&SosAction::Confirm);
                if timed_ready && !ctx.scratch.last_timed_ready {
                    *ctx.btn = confirm_press();
                }
                ctx.btn.update(ctx.gamepad, ctx.dt);
                if timed_ready {
                    ctx.scratch.timer = 0.0; // window activity = progress
                } else {
                    ctx.mash_if_stuck();
                }
            }
            // No input; just wait for the action to resolve. (A Charge skill
            // never reaches this default — it overrides the whole step.)
            TimingType::None | TimingType::Charge => {
                ctx.gamepad.release(&SosAction::Confirm);
                ctx.mash_if_stuck();
            }
        }
        ctx.scratch.last_timed_ready = timed_ready;
        StepOutcome::Stay
    }

    // --- Helpers ---

    /// Damage estimate for a magic special move at `roll`: the real decompiled
    /// formula when the move's `specialMovePower` is readable from memory, the
    /// rough magic heuristic otherwise (move not loaded yet — roll-independent,
    /// so its kills always count as guaranteed). The executor lands its cast
    /// QTEs — Sunball charges to max, timed hits connect — so the estimate
    /// assumes a full charge (`1.0`). The caster's current Live Mana charges
    /// feed the boost term.
    fn special_move_estimate(
        &self,
        cmd: &CombatManagerData,
        player: &CombatPlayer,
        enemy: &CombatEnemy,
        damage_type: CombatDamageType,
        roll: f32,
    ) -> f32 {
        let combat_move = self.find_move(cmd);
        match combat_move.and_then(|m| m.special_move_power.map(|p| (m, p))) {
            Some((combat_move, power)) => {
                let boost = damage::special_move_boost(player, combat_move);
                damage::special_move_damage(player, enemy, damage_type, power, boost, 1.0, roll)
            }
            None => damage::magic_damage_estimate(player, enemy, damage_type),
        }
    }

    /// The live `CombatPlayer` for this action's character, if present.
    fn player<'a>(&self, cmd: &'a CombatManagerData) -> Option<&'a CombatPlayer> {
        cmd.players
            .items
            .iter()
            .find(|p| p.character == self.character())
    }

    /// Whether this action's move is available for its character. `loaded`
    /// (membership in the fighter's per-fight loaded move lists) is the
    /// primary signal; a base skill (`unlockable == 0`) also counts, in case
    /// the loaded lists lag the fight's start. Only registered actions reach
    /// this, so an early-game unlearned base skill can't leak in through an
    /// unregistered move.
    fn move_available(&self, cmd: &CombatManagerData) -> bool {
        self.find_move(cmd)
            .is_some_and(|m| m.unlocked && !m.disabled && (m.loaded || m.unlockable == Some(0)))
    }

    /// This action's move definition in its character's live move list, if
    /// present.
    fn find_move<'a>(&self, cmd: &'a CombatManagerData) -> Option<&'a CombatMove> {
        cmd.moves
            .iter()
            .filter(|cm| cm.character == self.character())
            .flat_map(|cm| &cm.moves)
            .find(|m| m.move_id.as_deref() == Some(self.internal_name()))
    }
}
