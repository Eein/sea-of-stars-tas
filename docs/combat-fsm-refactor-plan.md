# Combat FSM refactor — staged execution plan

Turns [`combat-fsm-refactor.md`](combat-fsm-refactor.md) into commit-sized, in-game
verifiable stages. The executor **works today** (wins Basic fights); every stage
below must preserve that. Each stage is one commit, scoped to be smoke-tested
on a live fight before moving on.

## Design decisions the blueprint left open

Resolve these first — they set the trait signatures every later stage depends on.

### D1. Execution context (blocks step 2)

The `Action::execute_*` stubs (`skills/mod.rs:164-190`) are `fn(&self,
cmd: &CombatManagerData) -> bool`. That signature **cannot drive input**: no
gamepad, no `dt`, no button/timer state, no way to advance `step`. Introduce a
context the executor owns and passes in:

```rust
pub struct ActionCtx<'a> {
    pub cmd: &'a CombatManagerData,   // live combat snapshot
    pub gamepad: &'a mut Gamepad,     // the active player's pad
    pub btn: &'a mut ButtonPress,     // in-flight press state
    pub dt: f64,
    pub state_timer: f64,             // time in the current step
    pub step: ActionStep,             // current step (read)
    // scratch the arms need: char_swaps, target_taps, target_dir, last_cursor,
    // last_timed_ready — either here or kept on the controller and threaded in.
}
```

Each `execute_<step>(&self, ctx: &mut ActionCtx) -> StepOutcome` returns whether
it pressed, and the next step (or "stay"). Keep the return small:

```rust
pub enum StepOutcome { Stay, Advance(ActionStep), Done }
```

The controller maps `Advance`/`Done` onto `step`/committed-action lifetime.

### D2. `internal_name` must borrow self, not be `'static`

Combos are discovered from memory (`move_id: Option<String>`), so a `Combo`
action's name isn't `'static`. Change `fn internal_name(&self) -> &'static str`
to `-> &str`. Skills return a string literal (still fine); `Combo` returns
`&self.name`. Update the two callers in `appraisal.rs` and the `skill_timing`
lookup.

### D3. Actions are constructed bound to a character

`Sunball` is a unit struct returning `Zale`. `BasicAttack`/`Combo` apply to
whoever, so they carry a `character` field (mirrors Python's per-consideration
instantiation). `character()` returns the field.

## Stages

Each stage is one commit, independently in-game verifiable.

### Stage 1 — action foundation (additive, behavior-neutral)

Add `skills/basic_attack.rs` and `skills/combo.rs` as `Action` impls, config as
fields/overrides (`battle_command`, `timing_type`, `resource`, `cost`,
`damage_types`, `estimate_damage`). Apply D2 + D3. Do **not** wire into the
appraiser or executor yet — this stage only has to compile and leave the running
executor untouched.

*Verify:* fights still play identically (no code path changed); `cargo build`
+ `clippy` green.

### Stage 2 — appraiser emits actions

Bridge `Appraisal` → `Box<dyn Action>` (a constructor that maps each
`CombatAction` variant to the matching impl bound to `attacker`). Have
`generate_appraisals` build candidates *through* the impls so scoring lives on
`estimate_damage`. Keep the `CombatAction` enum for the GUI label for now.

*Verify:* the appraisal ranking/GUI is unchanged on a live fight (same top pick,
same numbers) — this is a pure refactor of how the same scores are produced.

### Stage 3 — the ability step machine (the big one)

Add `ActionCtx`/`StepOutcome` (D1). Port each `TurnFsm` arm body into the
matching `execute_<step>`:

| `TurnFsm` arm | `execute_*` |
|---|---|
| `SelectCommand` (ring nav + confirm) | `execute_selecting_command` / `execute_confirm_command` |
| `SelectComboAbility` / `SelectSkillAbility` | `execute_selecting_skill` / `execute_confirm_skill` |
| `SelectTarget` / `ConfirmTarget` | `execute_selecting_enemy_sequence` / `execute_confirm_enemy_sequence` |
| `Attacking` timing block | `execute_timing_sequence` (Sunball=Charge override, base=OneHit) |

Preserve every session-learned gotcha: `SUBMENU_SETTLE`, the charge
settle→hold→release timers (Sunball is **not** `timed_attack_ready`), the mash
safety nets, submenu-recovery cancels.

*Verify:* a full Basic fight end-to-end — basic attack, a combo, and Sunball each
execute and land their timing.

### Stage 4 — thin the controller to the pipeline

Replace the `TurnFsm` giant match with the `if stage() { return }` chain
(`controllers/mod.rs` already stubs the stages). Keep the coarse gates:
`any_enabled` turn-gate, dialog mash, `CombatControllerType::Basic` guard. The
controller now: appraise → (if no committed action) generate → drive
`action.execute(step)` → drop the action when `Done`.

*Verify:* same as stage 3, plus multi-turn fights and enemy turns (blocking/idle
gate) behave.

### Stage 5 — latch + swap gate + cleanup

- **Latch** the committed action (+ its character) for its whole lifetime, not
  just `attack_timing`; re-appraise only when there's no committed action. This
  is what fixes the live-`chosen`-flicker problems.
- **Character swap** only while `step == SelectingCommand`.
- Collapse the `CombatAction` per-variant special-casing in `mod.rs`/appraiser
  into ability fields now that everything routes through the trait.

*Verify:* the live-`chosen`-flicker wedges don't recur over several fights.

## Not in scope yet
`priority_targets` boss focus-fire, benched-member swaps (3rd+ party member),
non-`Basic` controllers. Add after the pipeline lands.

## Status (live-verified)

Stages 1–3 are **done and verified in-game** (drove `--route combat` on normal
encounters, fights won end-to-end):

- **Stage 1–2** — action foundation + appraiser routes through the `Action` impls.
- **Stage 3** — the ability step machine. `TurnFsm` collapsed to
  `Idle → SelectCharacter → Driving`; per-step logic lives on `execute_*` via
  `ActionCtx`/`StepOutcome`. Verified: basic attack + targeting, and Sunball
  select + **charge** all execute.
- **Pulled forward from Stage 5** — the `CommittedAction` latch. Driving rebuilt
  the action from the live `chosen` each frame, which abandoned Sunball's charge
  hold on any flicker; latching the action when leaving `SelectCharacter` fixed it.
- **Tuning** — `CHARGE_SETTLE` 1.50 → 0.35 (the old wait stalled the hold; charge
  screen opens fast).
- **Appraiser** — `chosen` now takes the top-*scored* appraisal (removed the
  `TEMP(slice2 RE)` skill-preference hack).

### Combos — blocked on an availability signal (deferred)

Combos never fire because `generate_appraisals` filters on `CombatMove.loaded`,
which is **always false for combos** (party-level combos have no per-fighter
`combatMoveComponent`; `loaded` only marks instantiated skills/basics). RE dump
of `allMoveDefinitions` found:

- `unlockable` field: `0` for the default combos (`DualAttack`, `DualAttackKids`,
  `SpectacleStrike`), `1` for everything that must be learned. But `unlockable`
  is a **static move-def property**, not a runtime "learned now" flag — so `1`
  alone can't tell a learned combo from an un-learned one.
- Probed `learned`/`isLearned`/`unlocked`/`isUnlocked`/`learnedByDefault`/
  `available` on the move def — **none resolve**; the runtime learned flag has
  another name / lives elsewhere.

Next: find the authoritative available/learned-combo source (a
learned-moves list on the fighter/party, or read the combo selection screen's
item list), then gate combos on `unlockable == 0 || <learned>` in
`generate_appraisals`. Dropping the `loaded` filter naively would wedge the
executor on un-learned combos it can't find in the menu.

### Remaining refactor stages
Stage 4 (thin the controller onto the `EncounterController` pipeline) and the
rest of Stage 5 (swap-gate cleanup, collapse `CombatAction` special-casing) are
still open — the controller is already thin, so these are mostly reorganisation.
