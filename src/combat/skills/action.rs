//! The [`Action`] trait — identity, appraisal hooks, and the default
//! step-driving behaviour shared by every action.

use data::prelude::PlayerPartyCharacter;
use joystick::common::JoystickBtnInterface;

use super::menu::{self, MAX_TARGET_TAPS, TARGET_DIRS, cancel_press, confirm_press, tap_press};
use super::step::{ActionCtx, ActionStep, StepOutcome};
use crate::combat::damage;
use crate::control::SosAction;
use crate::memory::combat_manager::{
    CombatDamageType, CombatEnemy, CombatManagerData, CombatMove, CombatPlayer,
};

/// If a post-commit wait sees no timed-window activity for this long, the
/// confirms likely desynced — mash to force the turn along.
const STUCK_TIMEOUT: f64 = 6.0;
/// Delay after committing a charge action before we start holding Confirm: the
/// caster leaps to center screen before the charge phase opens, so holding too
/// early gets the input eaten and the charge fails. The jump is slow, so this
/// must cover it — a short wait fails when the caster has to travel.
const CHARGE_SETTLE: f64 = 1.5;
/// How long to hold Confirm to build the charge before releasing to fire. A
/// charge isn't a timed hit — there's no `timed_attack_ready` window.
const CHARGE_HOLD: f64 = 1.75;
/// Grace after the charge releases before the mash safety net kicks in, to let
/// the cast animation resolve on its own.
const CHARGE_RESOLVE_GRACE: f64 = 4.0;

/// How the timed input is landed during [`ActionStep::Attacking`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimingType {
    /// No timed input.
    None,
    /// A single crisp tap when the window opens.
    OneHit,
    /// Hold, then release when the window fires.
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

    /// The game's `*BattleCommand` class name for this command, matched against a
    /// fighter's `disabled_commands` set to honour tutorial restrictions.
    pub fn class_name(self) -> &'static str {
        match self {
            BattleCommand::Attack => "BasicAttackBattleCommand",
            BattleCommand::Skill => "SkillBattleCommand",
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

    /// Estimated damage against `enemy`. Default is a multiple of the basic
    /// attack; damaging skills override this with their real formula.
    fn estimate_damage(
        &self,
        cmd: &CombatManagerData,
        player: &CombatPlayer,
        enemy: &CombatEnemy,
    ) -> f32 {
        let (_, max_roll) = cmd.damage_roll_bounds();
        let (base, timed) = damage::basic_attack_damage(player, enemy, max_roll);
        ((base + timed) * 1.5).floor()
    }

    // --- Execution steps (drive one frame of the action's step machine) ---
    //
    // The executor dispatches the action's current [`ActionStep`] to the
    // matching method, which reads live memory, presses at most one input, and
    // returns a [`StepOutcome`] telling the executor to stay, advance, or drop
    // the action. The defaults implement the standard menu-driving behaviour, so
    // a normal action (basic attack, most skills, combos) overrides nothing; only
    // an odd timing (a charge) is expressed through [`timing_type`](Self::timing_type),
    // which the default [`execute_attacking`](Self::execute_attacking) already
    // dispatches on.

    /// [`SelectingCommand`](ActionStep::SelectingCommand): navigate the
    /// battle-command ring to this action's command and confirm it. Skills and
    /// combos open a submenu ([`SelectingAbility`](ActionStep::SelectingAbility));
    /// everything else goes straight to target select.
    fn execute_selecting_command(&self, ctx: &mut ActionCtx) -> StepOutcome {
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
        let want_command = self.battle_command().ring_index();
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
    fn execute_selecting_ability(&self, ctx: &mut ActionCtx) -> StepOutcome {
        match self.battle_command() {
            BattleCommand::Combo => menu::drive_combo_submenu(self.internal_name(), ctx),
            _ => menu::drive_skill_submenu(self.internal_name(), ctx),
        }
    }

    /// [`SelectingTarget`](ActionStep::SelectingTarget): move the enemy cursor
    /// onto the action's target and confirm, bailing to the default target after
    /// [`MAX_TARGET_TAPS`]. Recovers to the menu we're actually in if the
    /// previous confirm didn't take.
    fn execute_selecting_target(&self, ctx: &mut ActionCtx) -> StepOutcome {
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
        let cursor_target = ctx.cmd.selected_attack_target_guid.clone();
        let on_target = match (ctx.want_target, &cursor_target) {
            (Some(want), Some(have)) => want == have,
            (None, _) => true,
            _ => false,
        };
        if on_target || ctx.scratch.taps >= MAX_TARGET_TAPS {
            *ctx.btn = confirm_press();
            ctx.scratch.timer = 0.0;
            return StepOutcome::Advance(ActionStep::ConfirmingTarget);
        }
        // If the previous tap didn't move the cursor, that axis is exhausted
        // (edge / wrong direction) — rotate to the next.
        if cursor_target == ctx.scratch.last_cursor {
            ctx.scratch.cursor_dir = (ctx.scratch.cursor_dir + 1) % TARGET_DIRS.len();
        }
        ctx.scratch.last_cursor = cursor_target;
        *ctx.btn = tap_press(TARGET_DIRS[ctx.scratch.cursor_dir]);
        ctx.scratch.taps += 1;
        StepOutcome::Stay
    }

    /// [`ConfirmingTarget`](ActionStep::ConfirmingTarget): drive the
    /// target-confirm press; once it lands, arm the timed-hit edge detector and
    /// enter [`Attacking`](ActionStep::Attacking).
    fn execute_confirming_target(&self, ctx: &mut ActionCtx) -> StepOutcome {
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
    fn execute_attacking(&self, ctx: &mut ActionCtx) -> StepOutcome {
        // A menu returning means the action resolved — release (dropping any held
        // charge so it can't auto-confirm the menu) and let the executor pick
        // the next action.
        if ctx.cmd.battle_command_has_focus || ctx.in_submenu() {
            ctx.gamepad.release(&SosAction::Confirm);
            ctx.scratch.timer = 0.0;
            return StepOutcome::Done;
        }
        let timed_ready = ctx.timed_ready();
        match self.timing_type() {
            TimingType::Charge => {
                // Settle → hold to build the charge → release to fire. No timed
                // window is involved; it's driven off the step timer alone.
                let hold_end = CHARGE_SETTLE + CHARGE_HOLD;
                if ctx.scratch.timer >= CHARGE_SETTLE && ctx.scratch.timer < hold_end {
                    ctx.gamepad.press(&SosAction::Confirm);
                } else {
                    ctx.gamepad.release(&SosAction::Confirm);
                }
                // Safety net: if the cast still hasn't resolved a while after
                // releasing, mash to force the turn along.
                if ctx.scratch.timer >= hold_end + CHARGE_RESOLVE_GRACE {
                    ctx.mash();
                }
            }
            TimingType::OneHit | TimingType::MultiHit => {
                // Tap Confirm on each rising edge of the window.
                ctx.gamepad.release(&SosAction::Confirm);
                if timed_ready && !ctx.scratch.last_timed_ready {
                    *ctx.btn = confirm_press();
                }
                ctx.btn.update(ctx.gamepad, ctx.dt);
                if timed_ready {
                    ctx.scratch.timer = 0.0; // window activity = progress
                } else if ctx.scratch.timer >= STUCK_TIMEOUT {
                    ctx.mash();
                }
            }
            TimingType::None => {
                // No input; just wait for the action to resolve.
                ctx.gamepad.release(&SosAction::Confirm);
                if ctx.scratch.timer >= STUCK_TIMEOUT {
                    ctx.mash();
                }
            }
        }
        ctx.scratch.last_timed_ready = timed_ready;
        StepOutcome::Stay
    }

    // --- Helpers ---

    /// The live `CombatPlayer` for this action's character, if present.
    fn player<'a>(&self, cmd: &'a CombatManagerData) -> Option<&'a CombatPlayer> {
        cmd.players
            .items
            .iter()
            .find(|p| p.character == self.character())
    }

    /// Whether this action's move is available for its character. `loaded` (a
    /// live `combatMoveComponent`) is unreliable for skills — a castable skill
    /// like CrescentArc can read `loaded=0` — so, as for combos, a base skill
    /// (`unlockable == 0`) also counts. Only registered actions reach this, so an
    /// early-game unlearned base skill can't leak in through an unregistered move.
    fn move_available(&self, cmd: &CombatManagerData) -> bool {
        self.find_move(cmd)
            .is_some_and(|m| m.loaded || m.unlockable == Some(0))
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
