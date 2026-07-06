//! Combat actions — one module (an `Action` impl) per skill.
//!
//! An `Action` is the unit the utility AI appraises (`is_usable` +
//! `calculate_value`) and the executor drives step-by-step (the `execute_*`
//! methods, one per [`ActionStep`]). Basic attacks and combos are modelled by
//! the appraiser's `CombatAction` enum today; skills — which each want custom
//! timing, damage formulas, and types — live here as trait impls so a simple
//! skill is a few lines and a weird one overrides only what it needs.

use data::prelude::PlayerPartyCharacter;
use joystick::common::JoystickBtnInterface;
use joystick::prelude::GenericJoystick;

use crate::combat::damage;
use crate::control::SosAction;
use crate::memory::combat_manager::{
    CombatDamageType, CombatEnemy, CombatManagerData, CombatPlayer,
};
use crate::seq::button::ButtonPress;

mod basic_attack;
mod combo;
mod crescent_arc;
mod sunball;

pub use basic_attack::BasicAttack;
pub use combo::Combo;

/// The steps an action is driven through, mirroring the executor's `TurnFsm`.
/// The executor calls the matching `Action::execute_*` for the current step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionStep {
    /// Navigating the battle-command ring (Attack/Skill/Combo/Item).
    SelectingCommand,
    /// Boosting the command with extra mana (optional).
    Boost,
    /// Confirming the highlighted command.
    ConfirmCommand,
    /// Navigating the skill/combo submenu to the desired ability.
    SelectingSkill,
    /// Confirming the highlighted ability.
    ConfirmSkill,
    /// Moving the enemy cursor onto the target.
    SelectingEnemySequence,
    /// Confirming the target and committing.
    ConfirmEnemySequence,
    /// Landing the timed input during the animation.
    TimingSequence,
    /// The action has resolved.
    ActionComplete,
}

/// How the timed input is landed during [`ActionStep::TimingSequence`].
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
}

/// The result of driving one [`ActionStep`]: stay on this step, advance to
/// another, or the whole action has resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome {
    /// Remain on the current step next frame.
    Stay,
    /// Move to `next` step (the controller resets nothing else).
    Advance(ActionStep),
    /// The action finished; the controller drops it and picks the next.
    Done,
}

/// Everything an [`Action`]'s `execute_*` step needs to drive one frame: the live
/// combat snapshot, the active player's gamepad, the in-flight button press, and
/// the per-action scratch owned by the controller and threaded through for the
/// action's lifetime.
pub struct ActionCtx<'a> {
    pub cmd: &'a CombatManagerData,
    pub gamepad: &'a mut GenericJoystick,
    pub btn: &'a mut ButtonPress,
    pub dt: f64,
    /// The enemy `unique_id` the action wants to hit (`None` = accept the default
    /// cursor target).
    pub want_target: Option<&'a str>,
    /// Time spent on the current step (settle delays + anti-hang watchdogs).
    pub state_timer: &'a mut f64,
    /// Directional/confirm taps spent this step, so we can bail instead of looping.
    pub target_taps: &'a mut u32,
    /// Index into [`TARGET_DIRS`] for the current cursor-step direction.
    pub target_dir: &'a mut usize,
    /// Cursor target at the last tap, to detect when a direction stops moving it.
    pub last_cursor: &'a mut Option<String>,
    /// `timed_attack_ready` last frame, so we act only on the rising edge.
    pub last_timed_ready: &'a mut bool,
}

impl ActionCtx<'_> {
    /// Whether an ability submenu (combo or skill) is currently open.
    fn in_submenu(&self) -> bool {
        self.cmd.highlighted_combo_id.is_some() || self.cmd.skill_command_has_focus
    }

    /// Any acting player's timed-hit window is open this frame.
    fn timed_ready(&self) -> bool {
        self.cmd.players.items.iter().any(|p| p.timed_attack_ready)
    }

    /// Safety-net mash: spam Confirm on the active pad to force a stuck turn along.
    fn mash(&mut self) {
        if self.btn.done() {
            *self.btn = mash_press();
        }
        self.btn.update(self.gamepad, self.dt);
    }
}

/// Cursor taps to spend chasing the chosen target/ability before bailing to
/// whatever the cursor lands on (mirrors the Python bot's bail-out).
const MAX_TARGET_TAPS: u32 = 16;
/// Grace after confirming a Skill/Combo command before an unopened submenu is
/// treated as "closed" — the submenu takes a couple frames to appear, and
/// bouncing early wedges us on the command ring.
const SUBMENU_SETTLE: f64 = 0.30;
/// If a post-commit wait sees no timed-window activity for this long, the
/// confirms likely desynced — mash to force the turn along.
const STUCK_TIMEOUT: f64 = 6.0;
/// Delay after committing a charge action before we start holding Confirm: the
/// caster leaps to center screen before the charge phase opens, so holding too
/// early gets the input eaten and the charge never builds. Kept short — the
/// charge screen opens quickly, so a long wait just stalls the hold.
const CHARGE_SETTLE: f64 = 0.35;
/// How long to hold Confirm to build the charge before releasing to fire. A
/// charge isn't a timed hit — there's no `timed_attack_ready` window.
const CHARGE_HOLD: f64 = 1.75;
/// Grace after the charge releases before the mash safety net kicks in, to let
/// the cast animation resolve on its own.
const CHARGE_RESOLVE_GRACE: f64 = 4.0;
/// Directions to step the enemy cursor. Enemies can be laid out 2D, so we cycle
/// through these when a direction stops moving the cursor.
const TARGET_DIRS: [SosAction; 4] = [
    SosAction::MenuRight,
    SosAction::MenuDown,
    SosAction::MenuLeft,
    SosAction::MenuUp,
];

/// A menu Confirm with a short settle gap, to step the command ring / target select.
pub(crate) fn confirm_press() -> ButtonPress {
    ButtonPress {
        action: SosAction::Confirm,
        press_time: 0.04,
        release_time: 0.10,
        ..Default::default()
    }
}

/// A Cancel (B) tap to back out of a menu (e.g. an uncastable submenu).
pub(crate) fn cancel_press() -> ButtonPress {
    ButtonPress {
        action: SosAction::Cancel,
        press_time: 0.04,
        release_time: 0.12,
        ..Default::default()
    }
}

/// A directional tap with a short settle gap, used to step the command/enemy cursor.
pub(crate) fn tap_press(action: SosAction) -> ButtonPress {
    ButtonPress {
        action,
        press_time: 0.04,
        release_time: 0.12,
        ..Default::default()
    }
}

/// A single Confirm tap used for mashing.
pub(crate) fn mash_press() -> ButtonPress {
    ButtonPress {
        action: SosAction::Confirm,
        press_time: 0.1,
        release_time: 0.2,
        ..Default::default()
    }
}

/// Drive the combo submenu (which reuses the *battle* selector — battle focus
/// stays true, skill focus doesn't) to `want_ability`, confirm it, and advance
/// to target select. Backs out to [`SelectingCommand`](ActionStep::SelectingCommand)
/// if the combo can't be found or isn't castable.
fn drive_combo_submenu(want_ability: &str, ctx: &mut ActionCtx) -> StepOutcome {
    if !ctx.btn.done() {
        ctx.btn.update(ctx.gamepad, ctx.dt);
        return StepOutcome::Stay;
    }
    let highlighted = ctx.cmd.highlighted_combo_id.clone();
    let in_submenu = highlighted.is_some();
    let on_combo = highlighted.as_deref() == Some(want_ability);
    if !ctx.cmd.battle_command_has_focus {
        // Combo confirmed — we're in target select now.
        *ctx.state_timer = 0.0;
        *ctx.target_taps = 0;
        return StepOutcome::Advance(ActionStep::SelectingEnemySequence);
    }
    if !in_submenu && *ctx.state_timer < SUBMENU_SETTLE {
        // Submenu still opening after the confirm — wait for it.
        return StepOutcome::Stay;
    }
    if !in_submenu
        || *ctx.target_taps >= MAX_TARGET_TAPS
        || (on_combo && !ctx.cmd.highlighted_combo_castable)
    {
        // Shouldn't be here, can't find it, or it isn't castable — back out.
        *ctx.state_timer = 0.0;
        *ctx.target_taps = 0;
        return StepOutcome::Advance(ActionStep::SelectingCommand);
    }
    if on_combo {
        *ctx.btn = confirm_press();
        *ctx.state_timer = 0.0;
        return StepOutcome::Stay;
    }
    *ctx.btn = tap_press(SosAction::MenuDown);
    *ctx.target_taps += 1;
    StepOutcome::Stay
}

/// Drive the skill submenu (which lives on the *skill* selector — skill focus
/// true, battle focus drops) to `want_ability`, confirm it, and advance to target
/// select. Backs out to [`SelectingCommand`](ActionStep::SelectingCommand) if the
/// submenu doesn't open or the skill can't be found/cast.
fn drive_skill_submenu(want_ability: &str, ctx: &mut ActionCtx) -> StepOutcome {
    if !ctx.btn.done() {
        ctx.btn.update(ctx.gamepad, ctx.dt);
        return StepOutcome::Stay;
    }
    let in_submenu = ctx.cmd.skill_command_has_focus;
    let highlighted = ctx.cmd.highlighted_skill_id.clone();
    let on_skill = highlighted.as_deref() == Some(want_ability);
    if !in_submenu && !ctx.cmd.battle_command_has_focus {
        // Off the ring with no skill submenu → the skill was confirmed; target
        // select now.
        *ctx.state_timer = 0.0;
        *ctx.target_taps = 0;
        return StepOutcome::Advance(ActionStep::SelectingEnemySequence);
    }
    if !in_submenu && *ctx.state_timer < SUBMENU_SETTLE {
        // Submenu still opening after the confirm — wait for it.
        return StepOutcome::Stay;
    }
    if !in_submenu
        || *ctx.target_taps >= MAX_TARGET_TAPS
        || (on_skill && !ctx.cmd.highlighted_skill_castable)
    {
        // Settled and still on the ring (submenu didn't open), can't find it, or
        // it isn't castable — back out via command select.
        *ctx.state_timer = 0.0;
        *ctx.target_taps = 0;
        return StepOutcome::Advance(ActionStep::SelectingCommand);
    }
    if on_skill {
        *ctx.btn = confirm_press();
        *ctx.state_timer = 0.0;
        return StepOutcome::Stay;
    }
    *ctx.btn = tap_press(SosAction::MenuDown);
    *ctx.target_taps += 1;
    StepOutcome::Stay
}

/// A combat action. One `impl` per skill; identity + usability + value power the
/// appraiser now, and the `execute_*` steps will drive the executor (slice 2).
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
        if !self.move_loaded(cmd) {
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
        cmd.moves
            .iter()
            .filter(|cm| cm.character == self.character())
            .flat_map(|cm| &cm.moves)
            .find(|m| m.move_id.as_deref() == Some(self.internal_name()))
            .and_then(|m| m.skill_point_cost)
            .unwrap_or_else(|| self.cost())
    }

    /// Estimated damage against `enemy`. Default is a multiple of the basic
    /// attack; damaging skills override this with their real formula.
    fn estimate_damage(&self, player: &CombatPlayer, enemy: &CombatEnemy) -> f32 {
        let (base, timed) = damage::basic_attack_damage(player, enemy, damage::MAX_ROLL);
        ((base + timed) * 1.5).floor()
    }

    // --- Execution steps (drive one frame of the action's micro-FSM) ---
    //
    // The controller dispatches the action's current [`ActionStep`] to the
    // matching method, which reads live memory, presses at most one input, and
    // returns a [`StepOutcome`] telling the controller to stay, advance, or drop
    // the action. The defaults implement the standard menu-driving behaviour, so
    // a normal action (basic attack, most skills, combos) overrides nothing; only
    // an odd timing (a charge) is expressed through [`timing_type`](Self::timing_type),
    // which the default [`execute_timing_sequence`](Self::execute_timing_sequence)
    // already dispatches on.

    /// Navigate the battle-command ring to this action's command and confirm it.
    /// Skills/combos open a submenu ([`SelectingSkill`](ActionStep::SelectingSkill));
    /// everything else goes straight to target select.
    fn execute_selecting_command(&self, ctx: &mut ActionCtx) -> StepOutcome {
        if !ctx.btn.done() {
            ctx.btn.update(ctx.gamepad, ctx.dt);
            return StepOutcome::Stay;
        }
        if ctx.in_submenu() {
            // We're in an ability submenu, not the ring — back out first.
            *ctx.btn = cancel_press();
            *ctx.state_timer = 0.0;
            return StepOutcome::Stay;
        }
        let want_command = self.battle_command().ring_index();
        if ctx.cmd.battle_command_index == Some(want_command) {
            *ctx.btn = confirm_press();
            *ctx.state_timer = 0.0;
            return match self.battle_command() {
                BattleCommand::Skill | BattleCommand::Combo => {
                    StepOutcome::Advance(ActionStep::SelectingSkill)
                }
                _ => StepOutcome::Advance(ActionStep::SelectingEnemySequence),
            };
        }
        // Step the command cursor toward the desired command.
        *ctx.btn = tap_press(SosAction::MenuDown);
        StepOutcome::Stay
    }

    /// Navigate the ability submenu (combo on the battle selector, skill on the
    /// skill selector) to this action's move and confirm it. Backs out to
    /// [`SelectingCommand`](ActionStep::SelectingCommand) if the move can't be
    /// found/cast, advances to target select once committed.
    fn execute_selecting_skill(&self, ctx: &mut ActionCtx) -> StepOutcome {
        match self.battle_command() {
            BattleCommand::Combo => drive_combo_submenu(self.internal_name(), ctx),
            _ => drive_skill_submenu(self.internal_name(), ctx),
        }
    }

    /// Move the enemy cursor onto the action's target and confirm, bailing to the
    /// default target after [`MAX_TARGET_TAPS`]. Recovers to the menu we're
    /// actually in if the previous confirm didn't take.
    fn execute_selecting_enemy_sequence(&self, ctx: &mut ActionCtx) -> StepOutcome {
        if !ctx.btn.done() {
            ctx.btn.update(ctx.gamepad, ctx.dt);
            return StepOutcome::Stay;
        }
        if ctx.in_submenu() {
            // Still in an ability submenu — handle it there.
            *ctx.state_timer = 0.0;
            return StepOutcome::Advance(ActionStep::SelectingSkill);
        }
        if ctx.cmd.battle_command_has_focus {
            // Still on the command ring — the confirm didn't take; retry.
            *ctx.state_timer = 0.0;
            return StepOutcome::Advance(ActionStep::SelectingCommand);
        }
        let cursor_target = ctx.cmd.selected_attack_target_guid.clone();
        let on_target = match (ctx.want_target, &cursor_target) {
            (Some(want), Some(have)) => want == have,
            (None, _) => true,
            _ => false,
        };
        if on_target || *ctx.target_taps >= MAX_TARGET_TAPS {
            *ctx.btn = confirm_press();
            *ctx.state_timer = 0.0;
            return StepOutcome::Advance(ActionStep::ConfirmEnemySequence);
        }
        // If the previous tap didn't move the cursor, that axis is exhausted
        // (edge / wrong direction) — rotate to the next.
        if cursor_target == *ctx.last_cursor {
            *ctx.target_dir = (*ctx.target_dir + 1) % TARGET_DIRS.len();
        }
        *ctx.last_cursor = cursor_target;
        *ctx.btn = tap_press(TARGET_DIRS[*ctx.target_dir]);
        *ctx.target_taps += 1;
        StepOutcome::Stay
    }

    /// Drive the target-confirm press; once it lands, arm the timed-hit edge
    /// detector and enter [`TimingSequence`](ActionStep::TimingSequence).
    fn execute_confirm_enemy_sequence(&self, ctx: &mut ActionCtx) -> StepOutcome {
        if ctx.btn.update(ctx.gamepad, ctx.dt) {
            ctx.gamepad.release(&SosAction::Confirm);
            *ctx.state_timer = 0.0;
            *ctx.last_timed_ready = false;
            return StepOutcome::Advance(ActionStep::TimingSequence);
        }
        StepOutcome::Stay
    }

    /// Land the action's timed input for the whole animation, per its
    /// [`timing_type`](Self::timing_type). Resolves ([`Done`](StepOutcome::Done))
    /// when a menu returns (the action ended); a stuck window triggers a mash.
    fn execute_timing_sequence(&self, ctx: &mut ActionCtx) -> StepOutcome {
        // A menu returning means the action resolved — release (dropping any held
        // charge so it can't auto-confirm the menu) and let the controller pick
        // the next action.
        if ctx.cmd.battle_command_has_focus || ctx.in_submenu() {
            ctx.gamepad.release(&SosAction::Confirm);
            *ctx.state_timer = 0.0;
            return StepOutcome::Done;
        }
        let timed_ready = ctx.timed_ready();
        match self.timing_type() {
            TimingType::Charge => {
                // Settle → hold to build the charge → release to fire. No timed
                // window is involved; it's driven off the state timer alone.
                let hold_end = CHARGE_SETTLE + CHARGE_HOLD;
                if *ctx.state_timer >= CHARGE_SETTLE && *ctx.state_timer < hold_end {
                    ctx.gamepad.press(&SosAction::Confirm);
                } else {
                    ctx.gamepad.release(&SosAction::Confirm);
                }
                // Safety net: if the cast still hasn't resolved a while after
                // releasing, mash to force the turn along.
                if *ctx.state_timer >= hold_end + CHARGE_RESOLVE_GRACE {
                    ctx.mash();
                }
            }
            TimingType::OneHit | TimingType::MultiHit => {
                // Tap Confirm on each rising edge of the window.
                ctx.gamepad.release(&SosAction::Confirm);
                if timed_ready && !*ctx.last_timed_ready {
                    *ctx.btn = confirm_press();
                }
                ctx.btn.update(ctx.gamepad, ctx.dt);
                if timed_ready {
                    *ctx.state_timer = 0.0; // window activity = progress
                } else if *ctx.state_timer >= STUCK_TIMEOUT {
                    ctx.mash();
                }
            }
            TimingType::None => {
                // No input; just wait for the action to resolve.
                ctx.gamepad.release(&SosAction::Confirm);
                if *ctx.state_timer >= STUCK_TIMEOUT {
                    ctx.mash();
                }
            }
        }
        *ctx.last_timed_ready = timed_ready;
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

    /// Whether this action's move is loaded (unlocked) for its character.
    fn move_loaded(&self, cmd: &CombatManagerData) -> bool {
        cmd.moves
            .iter()
            .filter(|cm| cm.character == self.character())
            .flat_map(|cm| &cm.moves)
            .any(|m| m.move_id.as_deref() == Some(self.internal_name()) && m.loaded)
    }
}

/// Every skill action the appraiser considers. One entry per skill module.
pub fn skill_actions() -> Vec<Box<dyn Action>> {
    vec![
        Box::new(sunball::Sunball),
        Box::new(crescent_arc::CrescentArc),
    ]
}

/// The timing type of the skill with this `combatMoveId`, if registered.
pub fn skill_timing(internal_name: &str) -> Option<TimingType> {
    skill_actions()
        .iter()
        .find(|a| a.internal_name() == internal_name)
        .map(|a| a.timing_type())
}
