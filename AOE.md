# AOE splash targeting — mechanics, live case study, and damage combinations

Findings from reverse-engineering how Sea of Stars decides which enemies an AOE
move (Crescent Arc) splashes, applied to the live 3-enemy boss fight
(boss + two adds), with the total damage of every possible main-target choice.

## How the game selects AOE targets

Decompiled from `PlayerRadiusTargetSelector` (RVA 0x6893D0, `SelectAOETargets`):

1. The player confirms a **main target**. The game takes that target's AOE
   anchor position — `CombatTarget.GetAOEOverlapPosition()` (RVA 0xF04BC0):
   the `dependencies.aoeOverlapPosition` Transform if set, else the target's
   own transform position.
2. It casts **`Physics.OverlapSphere(anchor, radius)`** and collects every
   `CombatTargetHitZone` collider the sphere touches. The radius is
   **`GlobalCombatSettings.playerAOERadius` = 3.0** (read live), unless the
   move's selector sets `overridePlayerAOERadius`/`customAOERadius`
   (`twoWaysAOE` additionally chains reverse checks; Crescent Arc appears to
   use the plain radius).
3. Every collected target except the main one is a **secondary** target: its
   `HitData.isAOE` flag is set (`CrescentArc.ApplyRealHits`, RVA 0x5BD5D0),
   and the damage pipeline multiplies its final damage by
   **`aoeDamageMultiplier` = 0.6** (`PlayerAttackDamage`, ctor default).

Because the sphere overlaps *colliders* (which have extent), the effective
reach is `radius + collider extent toward the center`, not center-to-center
distance. That matters below.

## The live case (captured 2026-07-08, `tas-cli --dump-state`)

| enemy | hp | pdef | mdef | type mods | position (x, y, z) |
|---|---|---|---|---|---|
| Boss `c05c96b0` | 250 | 75 | 50 | Sun ×1.25, Moon ×1.25 | (−39.38, 2.00, 237.82) |
| Add-Right `a975c82e` | 1 | 0 | 0 | — | (−34.93, 2.00, 236.62) |
| Add-Left `a975c82e` | 1 | 0 | 0 | — | (−43.49, 2.00, 236.49) |

Players: Zale matk 15 (Sunball power 12), Valere matk 13 (CrescentArc power 0).
`playerAOERadius` = 3.0. Random damage roll 0–3.

> ⚠️ Both adds report the **same** `unique_id` (`a975c82e-…`) — instances of
> the same enemy share the guid. The appraiser's target-by-guid cursor
> matching cannot distinguish them; worth fixing (e.g. also match by index or
> position) before AOE targeting relies on it.

### Distances (anchor to anchor)

| pair | distance |
|---|---|
| Boss ↔ Add-Right | 4.61 |
| Boss ↔ Add-Left | 4.32 |
| Add-Right ↔ Add-Left | 8.56 |

### Calibrating collider extent

Observed in game: targeting Add-Right splashes the boss; targeting the boss
splashes both adds; the adds never splash each other. With radius 3.0:

- Boss hit from 4.61 away ⇒ its hit-zone collider extends ≥ 1.61 toward the
  adds. Adds hit from 4.61 ⇒ their colliders extend ≥ 1.61 too.
- Adds never hit each other at 8.56 ⇒ extents < 5.56. ✓ consistent.

**Working model:** `hit iff anchor_distance ≤ radius + ~1.7` for this fight.
Reading the actual `CombatTargetHitZone` collider bounds (native
`BoxCollider` size/center) would make this exact — listed as follow-up.

## Damage model (from the decompiled special-move pipeline)

`PlayerSpecialMoveDamage.CalculatePreModifiersDamage` (RVA 0x4CFE70) — same
formula now implemented in `src/combat/damage.rs::special_move_damage`:

```text
base      = matk + specialMovePower + roll            (roll = 0..3)
processed = (1 − mdef/150) × typeModifier × base
final     = round(processed + t·(qteMult − 1)·(processed − roll))
qteMult   = 1.3 + 0.2·successCount = 1.5 on a timed success (t = 1)
secondary = round(pre-round value × 0.6)              (isAOE targets)
```

Crescent Arc per target, max roll (3), timed success (min-roll values in
parens):

| enemy | main-target damage | secondary (×0.6) |
|---|---|---|
| Boss (mdef 50, Moon ×1.25) | **18** (16) | **11** (10) |
| Either add (no def/mods) | **22** (20) | **14** (12) — overkill, hp 1 |

## All main-target combinations (Crescent Arc, max roll)

| main target | hit set | boss dmg | adds killed | raw total |
|---|---|---|---|---|
| **Boss** | Boss + both adds | **18** | **2/2** | 18 + 11 + 11 = 40 |
| Add-Right | Add-Right + Boss | 11 | 1/2 | 22 + 11 = 33 |
| Add-Left | Add-Left + Boss | 11 | 1/2 | 22 + 11 = 33 |

**Conclusion: target the boss.** It's strictly dominant here — max boss
damage (18 vs 11) *and* both 1-hp adds die to the 0.6× splash. The general
appraiser rule this case suggests: score an AOE candidate as
`Σ min(damage_i, hp_i)` over its predicted hit set, using the sphere model
above to predict the set.

For comparison, a max-charge Sunball on the boss is single-target **36**
(`base = 15 + 12 + 3 = 30`, ×1.25 Moon→Sun mod, ×⅔ defense, ×1.5 charge).

## What was built for this

- `CombatEnemy.position` — world position of each enemy's AOE anchor, read
  through the managed Transform's native hierarchy (TRS + parent-index walk).
- `CombatManagerData.player_aoe_radius` — `GlobalCombatSettings.playerAOERadius`.
- `CombatMove.special_move_power` — the move component's `specialMovePower`
  (Sunball reads 12, validating the whole chain).
- `damage::special_move_damage` — the decompiled formula; Sunball and
  Crescent Arc appraisals now use it via `Action::special_move_estimate`.
- `tas-cli --dump-state` now prints enemies (hp/defs/mods/position), players,
  loaded moves with power, and the AOE radius.

## Follow-ups

1. Read `CombatTargetHitZone` collider bounds to replace the ~1.7 extent
   calibration with exact geometry.
2. Read the move's `targetSelector` subclass + `customAOERadius`/`twoWaysAOE`
   to model per-move radii.
3. Distinguish same-guid enemy instances for cursor targeting.
4. Teach the appraiser to score AOE moves by predicted hit set
   (`Σ min(damage, hp)`), replacing the current per-enemy scoring.
5. Verify whether Crescent Arc's damage splits a physical (Blunt) stream —
   the model above assumes Moon-only; boss pdef 75 would change its numbers.
