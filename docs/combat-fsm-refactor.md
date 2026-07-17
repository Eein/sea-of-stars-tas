# Combat FSM refactor blueprint

Plan for restructuring the combat executor so per-move logic lives on the
abilities, mirroring the (clean) architecture of the Python TAS
[shenef/SoS-TAS](https://github.com/shenef/SoS-TAS). The current executor
works (wins fights) but started as a single
~600-line `TurnFsm` match in `CombatManager` with all move logic inline.

## The core insight

**The step-FSM belongs on the ability, not the controller.** In the Python
design the controller is a thin driver: it holds one committed action, calls
`action.execute()` each frame, and when the action reports `complete`, drops it
so the reasoner picks the next. All move-specific behaviour (which command, how
to time the hit, how to target) lives on the ability object, which carries its
own step enum and a `match step { … }` dispatch.

Our `Action` trait already has the stubs for this (`execute_selecting_command`,
`execute_confirm_command`, `execute_timing_sequence`, … in
`src/combat/skills/mod.rs`) and an `ActionStep` enum — they're currently unused.
The refactor is to move the body of each `TurnFsm` arm onto the matching
`execute_*` method and reduce the controller to a pipeline.

## Two nested layers (Python)

### 1. Coarse combat FSM — `engine/combat/combat_controller.py`

```
enum FSM { Idle, Combat, AfterCombat, LevelUpScreen }
```

- `update_state(delta)` — transitions the 4 states and lazily builds the right
  `EncounterController` via a factory (dispatches on controller type + enemy
  GUIDs → boss-specific controllers, else generic).
- `execute_combat(delta)` — in `Combat`, delegates to the encounter controller's
  pipeline.

### 2. Per-encounter pipeline — `engine/combat/controllers/encounter_controller.py`

Ordered "do one thing, bail if you did work" chain — **the control-flow idiom to
copy**. Each stage returns `true` to stop the frame:

```
set_neutral()
if execute_dialog()        { return }   // mash confirm through mid-fight dialog
if generate_action()       { return }   // reasoner picks best Action (only when a
                                        //   char is selected + command menu focused)
if execute_block()         { return }   // enemy turn (no char selected) → block/spam
if !has_action()           { return }
if execute_consideration() { return }   // swap acting character — ONLY at SelectingCommand
if execute_appraisal()     { return }   // action.appraisal.execute(); if complete → action = None
```

`EncounterController` holds `reasoner: SoSReasoner` and `action: Action`. Boss
encounters subclass it and override stages (e.g. inject `priority_targets`).

## The ability abstraction — `engine/combat/utility/sos_appraisal.py`

`SoSAppraisal` is a **self-contained micro-FSM** for executing one move.

Steps (`SoSAppraisalStep`):
```
SelectingCommand → Boost → ConfirmCommand
  → SelectingSkill/SelectingCombo → ConfirmSkill/ConfirmCombo
  → SelectingEnemySequence → ConfirmEnemySequence
  → TimingSequence → ActionComplete
```

`execute()` is just `match self.step`, one `execute_<step>()` per phase; each
reads live memory, presses one input, and advances `step` when the game confirms
the transition. `ActionComplete` sets `complete = True`.

Config lives as **fields on the ability**, so concrete moves usually override
*nothing but their timing*:
- `battle_command` (Attack/Skill/Combo/Item), `skill_command_index`
- `target_type`, `battle_command_targeting_type` (decoupled: Moonerang is a Skill
  to *select* but targets like an Attack)
- `timing_type` (None/OneHit/Charge/MultiHit), `boost`
- `damage_type: Vec`, `resource` + `cost`/`combo_cost`/`ultimate`
- hooks: `has_resources(actor)`, `is_player_timed_attack_ready()`,
  `adjust_value(enemy)`, `_fallback_for_missing_action_complete()`

Concrete moves (`engine/combat/appraisals/…`):
- `BasicAttack` — only sets fields (`Attack`, `OneHit`, damage from caster+boost).
- `ComboSkill` base — adds `casters`, `combo_cost`, `resource=ComboPoints`, and
  `can_use()` checking all required casters alive. `SolsticeStrike` just sets types.
- `Sunball` (Charge) — overrides `execute_timing_sequence()` only: hold confirm
  for a wall-clock `hold_time`.
- `Moonerang` (MultiHit) — overrides timing: tap confirm each time the projectile
  crosses a position threshold, looping until hit-count/char-selected/combat-done.

**Character is latched at `ConfirmCommand`** (`selected_character` drops from
memory mid-action); timing checks rely on the latched value. This is exactly our
`attack_timing` latch, generalized to the whole action.

## Decision layer (utility AI) — `utility/{sos_reasoner,sos_consideration}.py`

- **Consideration** = one actor + the abilities it could perform.
- **Appraisal** = one candidate move with a `value` score.
- **Action** = (consideration, appraisal) — "this character does this move here."
- **Reasoner** = picks the single highest-value Action.

`SoSReasoner.execute()`:
1. `generate_considerations(players)` — skip dead/disabled actors.
2. each consideration `calculate_actions()` → flatten to `Vec<Action>`.
3. sort by `appraisal.value` desc.
4. `filter_disabled_enemies` (drop 0-HP), then `filter_priority_targets` (if the
   context's `priority_targets` GUID list is non-empty, keep only actions hitting
   them — how bosses force focus).
5. return `actions[0]`.

Candidate generation (`SoSConsideration.generate_appraisals`) =
`_default_appraisals` (a `BasicAttack` per boost level 0–3) + `_character_appraisals`
(match on character, gated by learned-ability flags + `can_use()`), filtered by a
mana/combo/boost budget. `value *= boost + 1` biases toward boosting.

`calculate_actions()` expands **appraisal × enemy**: copies the appraisal, applies
a **spell-lock multiplier** (reward matching an enemy's locks with the move's
`damage_type`, extra for clearing all locks), sets `target`, calls
`adjust_value(enemy)`, wraps in `Action`. This is where break-lock strategy lives.

## Character selection / swapping

The AI picks the best Action across all living characters, then **waits until that
character is the active one**. `execute_consideration()` handles the swap but
**only while `step == SelectingCommand`**, so it never disturbs an in-progress
action. It checks `consideration.valid(selected_character, action)` (is the
selected char the action's actor?); if not, taps dpad left to rotate the cursor.
(There's a TODO in Python to handle swapping benched members onto the field —
relevant to us once a 3rd+ party member joins, per the combo note below.)

## Mapping to our code

| Python | Ours today | Refactor target |
|---|---|---|
| `SoSAppraisal` step FSM per ability | `TurnFsm` match in `src/combat/mod.rs` | Move each arm's body onto the `Action` trait `execute_*` methods (already stubbed in `src/combat/skills/mod.rs`) |
| `execute_timing_sequence()` override | `attack_timing` Charge/OneHit branch in `Attacking` | Each ability overrides only its timing; Sunball=Charge hold, Moonerang=MultiHit |
| config fields on the ability | scattered across `CombatAction` enum + trait defaults | Consolidate onto the ability (battle_command, timing_type, damage_type, resource, cost) |
| `Reasoner` → considerations → actions | `appraisal::generate_appraisals` | Keep; add the consideration (actor→abilities) layer + boost/spell-lock scoring |
| character latched at `ConfirmCommand` | `attack_timing` latch | Generalize: latch the whole committed action (+ its character) for the action's duration |
| `execute_consideration()` gated to `SelectingCommand` | the preemptive character-select gate | Same idea — only swap while selecting the command, never mid-action |
| `priority_targets` filter | — (not yet) | Add for boss focus-fire |

## Concrete steps for the refactor

1. **Thin the controller.** Replace the `TurnFsm` giant match with the pipeline of
   `if stage() { return }` calls. Keep the coarse gates already proven live:
   - `any_enabled` turn-gate (only act when a party member is enabled; else wait —
     this is the "not our turn / enemy acting" signal and is stable across the
     per-character `selected`/`enabled` flicker).
   - dialog mash, and the `CombatControllerType::Basic`-only guard.
2. **Give the ability the step machine.** Put `ActionStep`/`step` on the committed
   action; `execute()` = `match step`. Fill the `execute_*` methods with the logic
   currently in each `TurnFsm` arm (SelectCommand → command-ring nav; SelectSkill →
   skill submenu; SelectTarget → cursor; TimingSequence → the charge/one-hit logic).
3. **Latch the committed action** for its whole lifetime (not just timing) — resolves
   the live-`chosen`-flicker wedges. Re-appraise only when there's no
   committed action (mirrors `action = None` → `generate_action`).
4. **Consideration/character swap** only while `step == SelectingCommand`.
5. **Model config on abilities**, collapse the `CombatAction` enum's per-variant
   special-casing into ability fields + a couple of trait method overrides.

## Live-verified gotchas to preserve

- Charge (Sunball) is **not** tied to `timed_attack_ready` (that flag is a post-hit
  "trackingAfterHit" and never opens a press window here): it's settle → hold →
  release on a wall-clock timer. Basics/combos are OneHit by default.
- Submenus take a couple frames to open — don't judge one "closed" before a settle,
  or the FSM bounces on the command ring.
- Joint combos flip `selected` between their two participants; for the current
  two-character party a combo is reachable from either member's Combo menu, so no
  swap. A 3rd+ member will require keying the swap off the combo's participants.
- Recover from being left in a submenu (attach mid-menu) by cancelling out.

## Reference files (Python, `main` branch, repo root = source root)

- `engine/combat/combat_controller.py`
- `engine/combat/controllers/encounter_controller.py`
- `engine/combat/utility/core/{action,appraisal,consideration,reasoner}.py`
- `engine/combat/utility/{sos_appraisal,sos_consideration,sos_reasoner}.py`
- `engine/combat/contexts/reasoner_execution_context.py`
- `engine/combat/appraisals/basic_attack.py`, `appraisals/combos/{base,solstice_strike}.py`,
  `appraisals/zale/sunball.py`, `appraisals/valere/moonerang.py`
