//! Input primitives for driving the battle menus: button-press builders and
//! the shared submenu drivers the default `Action` steps delegate to.

use super::step::{ActionCtx, DriveResult};
use crate::control::SosAction;
use crate::seq::button::ButtonPress;
use crate::util::vec3_ext::Vector3Ext;

/// Cursor taps to spend chasing the chosen target/ability before bailing to
/// whatever the cursor lands on (mirrors the Python bot's bail-out).
pub(super) const MAX_TARGET_TAPS: u32 = 16;
/// Grace after confirming a Skill/Combo command before an unopened submenu is
/// treated as "closed" — the submenu takes a couple frames to appear after the
/// confirm.
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
        ctx.scratch.visited_targets.clear();
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
        ctx.scratch.visited_targets.clear();
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

/// Best first direction toward the wanted target from the enemy under the
/// cursor, by world-position deltas: under the game's mostly fixed combat
/// camera, screen-right tracks `+x` and screen-up tracks `+z` (depth). The
/// game itself resolves presses by *screen-space* angular scoring
/// (`CombatTargetSelectionScreen.FindTarget`, RVA 0xD537F0, over
/// `WorldToScreenPoint` vectors), so this is only a first guess — the
/// edge/cycle rotation in [`drive_target_cursor`] recovers when the camera
/// makes it wrong. `None` when either position is unreadable.
fn aim_at_target(ctx: &ActionCtx) -> Option<usize> {
    let want = ctx.want_target?;
    let cursor = ctx.cmd.selected_attack_target_guid.as_deref()?;
    let position_of = |id: &str| {
        ctx.cmd
            .enemies
            .items
            .iter()
            .find(|e| e.unique_id == id)
            .and_then(|e| e.position)
    };
    let want_pos = position_of(want)?;
    let cursor_pos = position_of(cursor)?;
    let dx = want_pos.get_x() - cursor_pos.get_x();
    let dz = want_pos.get_z() - cursor_pos.get_z();
    let dir = if dx.abs() >= dz.abs() {
        if dx > 0.0 {
            SosAction::MenuRight
        } else {
            SosAction::MenuLeft
        }
    } else if dz > 0.0 {
        SosAction::MenuUp
    } else {
        SosAction::MenuDown
    };
    TARGET_DIRS.iter().position(|d| *d == dir)
}

/// Step the enemy cursor toward `ctx.want_target`. [`Ok`](DriveResult::Ok)
/// once the cursor is on the wanted target — or the tap budget is spent, in
/// which case we accept whatever it's on (mirrors the Python bot's bail-out).
/// Never errors: there is always *a* target to confirm.
///
/// A direction is abandoned (rotate to the next) when it stops moving the
/// cursor (an edge) *or* when it lands on an enemy already visited this step —
/// cycling means the axis can't reach the target (e.g. in a triangle layout,
/// Left/Right just ping-pongs between the bottom two enemies while the top
/// one needs Up).
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
    let moved = cursor != ctx.scratch.last_cursor;
    let cycled = cursor
        .as_ref()
        .is_some_and(|c| ctx.scratch.visited_targets.contains(c));
    if let Some(c) = &cursor
        && !cycled
    {
        ctx.scratch.visited_targets.push(c.clone());
    }
    if ctx.scratch.taps == 0 {
        // First tap: aim by geometry when both positions are known.
        if let Some(dir) = aim_at_target(ctx) {
            ctx.scratch.cursor_dir = dir;
        }
    } else if !moved || cycled {
        // No progress this tap — the direction hit an edge or looped back
        // onto visited ground; try the next one.
        ctx.scratch.cursor_dir = (ctx.scratch.cursor_dir + 1) % TARGET_DIRS.len();
    }
    ctx.scratch.last_cursor = cursor;
    *ctx.btn = tap_press(TARGET_DIRS[ctx.scratch.cursor_dir]);
    ctx.scratch.taps += 1;
    DriveResult::Wait
}
