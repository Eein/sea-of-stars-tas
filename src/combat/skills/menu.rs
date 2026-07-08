//! Input primitives for driving the battle menus: button-press builders and
//! the shared submenu drivers the default `Action` steps delegate to.

use super::step::{ActionCtx, DriveResult};
use crate::control::SosAction;
use crate::seq::button::ButtonPress;

/// Cursor taps to spend chasing the chosen target/ability before bailing to
/// whatever the cursor lands on (mirrors the Python bot's bail-out).
pub(super) const MAX_TARGET_TAPS: u32 = 16;
/// Grace after confirming a Skill/Combo command before an unopened submenu is
/// treated as "closed" — the submenu takes a couple frames to appear, and
/// bouncing early wedges us on the command ring.
const SUBMENU_SETTLE: f64 = 0.30;
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
/// stays true, skill focus doesn't) to `want_ability` and confirm it.
/// [`Ok`](DriveResult::Ok) once the combo is committed (target select is up);
/// [`Error`](DriveResult::Error) if the combo can't be found or isn't castable.
pub(super) fn drive_combo_submenu(want_ability: &str, ctx: &mut ActionCtx) -> DriveResult {
    if !ctx.btn.done() {
        ctx.btn.update(ctx.gamepad, ctx.dt);
        return DriveResult::Wait;
    }
    let highlighted = ctx.cmd.highlighted_combo_id.clone();
    let in_submenu = highlighted.is_some();
    let on_combo = highlighted.as_deref() == Some(want_ability);
    if !ctx.cmd.battle_command_has_focus {
        // Combo confirmed — we're in target select now.
        ctx.scratch.timer = 0.0;
        ctx.scratch.taps = 0;
        return DriveResult::Ok;
    }
    if !in_submenu && ctx.scratch.timer < SUBMENU_SETTLE {
        // Submenu still opening after the confirm — wait for it.
        return DriveResult::Wait;
    }
    if !in_submenu
        || ctx.scratch.taps >= MAX_TARGET_TAPS
        || (on_combo && !ctx.cmd.highlighted_combo_castable)
    {
        // Shouldn't be here, can't find it, or it isn't castable — back out.
        ctx.scratch.timer = 0.0;
        ctx.scratch.taps = 0;
        return DriveResult::Error;
    }
    if on_combo {
        *ctx.btn = confirm_press();
        ctx.scratch.timer = 0.0;
        return DriveResult::Wait;
    }
    *ctx.btn = tap_press(SosAction::MenuDown);
    ctx.scratch.taps += 1;
    DriveResult::Wait
}

/// Drive the skill submenu (which lives on the *skill* selector — skill focus
/// true, battle focus drops) to `want_ability` and confirm it.
/// [`Ok`](DriveResult::Ok) once the skill is committed (target select is up);
/// [`Error`](DriveResult::Error) if the submenu doesn't open or the skill can't
/// be found/cast.
pub(super) fn drive_skill_submenu(want_ability: &str, ctx: &mut ActionCtx) -> DriveResult {
    if !ctx.btn.done() {
        ctx.btn.update(ctx.gamepad, ctx.dt);
        return DriveResult::Wait;
    }
    let in_submenu = ctx.cmd.skill_command_has_focus;
    let highlighted = ctx.cmd.highlighted_skill_id.clone();
    let on_skill = highlighted.as_deref() == Some(want_ability);
    if !in_submenu && !ctx.cmd.battle_command_has_focus {
        // Off the ring with no skill submenu → the skill was confirmed; target
        // select now.
        ctx.scratch.timer = 0.0;
        ctx.scratch.taps = 0;
        return DriveResult::Ok;
    }
    if !in_submenu && ctx.scratch.timer < SUBMENU_SETTLE {
        // Submenu still opening after the confirm — wait for it.
        return DriveResult::Wait;
    }
    if !in_submenu
        || ctx.scratch.taps >= MAX_TARGET_TAPS
        || (on_skill && !ctx.cmd.highlighted_skill_castable)
    {
        // Settled and still on the ring (submenu didn't open), can't find it, or
        // it isn't castable — back out via command select.
        ctx.scratch.timer = 0.0;
        ctx.scratch.taps = 0;
        return DriveResult::Error;
    }
    if on_skill {
        *ctx.btn = confirm_press();
        ctx.scratch.timer = 0.0;
        return DriveResult::Wait;
    }
    *ctx.btn = tap_press(SosAction::MenuDown);
    ctx.scratch.taps += 1;
    DriveResult::Wait
}

/// Step the enemy cursor toward `ctx.want_target`. [`Ok`](DriveResult::Ok)
/// once the cursor is on the wanted target — or the tap budget is spent, in
/// which case we accept whatever it's on (mirrors the Python bot's bail-out).
/// Never errors: there is always *a* target to confirm.
pub(super) fn drive_target_cursor(ctx: &mut ActionCtx) -> DriveResult {
    let cursor = ctx.cmd.selected_attack_target_guid.clone();
    let on_target = match (ctx.want_target, &cursor) {
        (Some(want), Some(have)) => want == have,
        (None, _) => true,
        _ => false,
    };
    if on_target || ctx.scratch.taps >= MAX_TARGET_TAPS {
        return DriveResult::Ok;
    }
    // If the previous tap didn't move the cursor, that axis is exhausted
    // (edge / wrong direction) — rotate to the next.
    if cursor == ctx.scratch.last_cursor {
        ctx.scratch.cursor_dir = (ctx.scratch.cursor_dir + 1) % TARGET_DIRS.len();
    }
    ctx.scratch.last_cursor = cursor;
    *ctx.btn = tap_press(TARGET_DIRS[ctx.scratch.cursor_dir]);
    ctx.scratch.taps += 1;
    DriveResult::Wait
}
