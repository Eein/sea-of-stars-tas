use crate::combat::damage;
use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use data::Item;
use data::prelude::{PlayerPartyCharacter, armor, trinkets, weapons};
use log::info;
use memory::game_engine::il2cpp::Class;
use memory::game_engine::il2cpp::unity_list::*;
use memory::game_engine::il2cpp::unity_serializable_dictionary::*;
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;
use memory::process::Process;
use memory::string::*;
use vec3_rs::Vector3;

#[derive(Default, Debug)]
pub enum CombatControllerType {
    #[default]
    Basic,
    FirstEncounter,
    SecondEncounter,
    DwellerOfStrife,
    DwellerOfDread,
    KOTutorial,
    LiveManaTutorial,
    ManaRegenTutorial,
    RoundsTutorial,
    SpellLockTutorial,
    TimedBlocksTutorial,
    TimedHitsTutorial,
    KidsCavernEncounter,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum CombatDamageType {
    #[default]
    None = 0,
    Any = 1,
    Sword = 2,
    Sun = 4,
    Moon = 8,
    Eclipse = 16,
    Poison = 32,
    Arcane = 64,
    Stun = 128,
    Blunt = 256,
    Magical = 252,
}

impl CombatDamageType {
    fn from_u32(value: u32) -> CombatDamageType {
        match value {
            0 => CombatDamageType::None,
            1 => CombatDamageType::Any,
            2 => CombatDamageType::Sword,
            4 => CombatDamageType::Sun,
            8 => CombatDamageType::Moon,
            16 => CombatDamageType::Eclipse,
            32 => CombatDamageType::Poison,
            64 => CombatDamageType::Arcane,
            128 => CombatDamageType::Stun,
            256 => CombatDamageType::Blunt,
            252 => CombatDamageType::Magical,
            _ => CombatDamageType::None,
        }
    }
}

/// The game's `EPlayableCharacterStat` (TypeDefIndex 4363) — which character
/// stat a serialized `PlayableCharacterStat` names (e.g. the stat a move's
/// Live Mana boost multiplies).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PlayableCharacterStat {
    HitPoint = 0,
    SkillPoint = 1,
    ComboPoint = 2,
    PhysicalAttack = 3,
    PhysicalDefense = 4,
    #[default]
    MagicalAttack = 5,
    MagicalDefense = 6,
    Level = 7,
}

impl PlayableCharacterStat {
    fn from_i32(value: i32) -> Option<PlayableCharacterStat> {
        Some(match value {
            0 => PlayableCharacterStat::HitPoint,
            1 => PlayableCharacterStat::SkillPoint,
            2 => PlayableCharacterStat::ComboPoint,
            3 => PlayableCharacterStat::PhysicalAttack,
            4 => PlayableCharacterStat::PhysicalDefense,
            5 => PlayableCharacterStat::MagicalAttack,
            6 => PlayableCharacterStat::MagicalDefense,
            7 => PlayableCharacterStat::Level,
            _ => return None,
        })
    }
}

#[derive(Default, Debug)]
pub struct LiveMana {
    pub big: u32,
    pub small: u32,
}

/// The step Zale's Sunball charge QTE is on (`SunboyShootQTESunballState.currentStep`).
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SunballChargeStep {
    /// Intro: "Hold A for power!" — the intro animation plays forward only while
    /// Confirm is held, then transitions to [`Charging`](Self::Charging). The
    /// projectile hasn't been (re)spawned yet, so its `level` is stale here.
    #[default]
    In,
    /// The charge is building: holding climbs the projectile's `level` one step
    /// at a time up to `max_level`.
    Charging,
    /// The sunball has been released/thrown.
    Shoot,
    /// Any other/unrecognised step value.
    Other,
}

impl SunballChargeStep {
    fn from_i32(value: i32) -> Self {
        match value {
            0 => Self::In,
            1 => Self::Charging,
            2 => Self::Shoot,
            _ => Self::Other,
        }
    }
}

/// Live state of Zale's Sunball charge, read from the active
/// `SunboyShootQTESunballState` while the charge QTE is on screen. Present only
/// while that state is active; `None` otherwise.
///
/// The move charges through discrete levels: holding Confirm advances the intro
/// then climbs the projectile's `level` one step at a time up to `max_level`.
/// Releasing exactly at `max_level` lands the strongest hit (and its timing
/// QTE). The executor (`CombatController::charge_should_hold`) turns this into a
/// hold/release decision — no blind timer.
#[derive(Default, Debug, Clone)]
pub struct SunballCharge {
    /// The projectile's current charge level (`SunballProjectile.level`),
    /// counting up from 0 as the charge builds. Only meaningful during
    /// [`Charging`](SunballChargeStep::Charging) — the projectile is pooled, so
    /// during [`In`](SunballChargeStep::In) (and the first Charging frame) it can
    /// still read the *previous* cast's level until the fresh projectile spawns.
    pub level: i32,
    /// The move's maximum charge level (`SunboyShootQTESunballState.sunballMaxLevel`,
    /// normally 4).
    pub max_level: i32,
    /// Which step the QTE is on.
    pub step: SunballChargeStep,
}

#[derive(Default, Debug, Clone)]
pub struct EquippedTrinket {
    pub trinket: Option<Item>,
}

#[derive(Debug, Clone, Default)]
pub struct CombatPlayer {
    // TODO(eein): use raw calcs for max_hp/max_mp
    pub level: u32, // Estimation
    pub max_hp: u32,
    pub max_mp: u32,
    pub current_hp: u32,
    pub current_mp: u32,
    pub base_physical_attack: u32,
    pub base_physical_defense: u32,
    pub base_magical_attack: u32,
    pub base_magical_defense: u32,
    pub physical_attack: u32,
    pub physical_defense: u32,
    pub magical_attack: u32,
    pub magical_defense: u32,
    pub selected: bool,
    pub character: PlayerPartyCharacter,
    pub timed_attack_ready: bool,
    pub dead: bool,
    pub enabled: bool, // this is if active on the screen
    pub mana_charge_count: u32,
    pub equipped_weapon: Option<Item>,
    pub equipped_armor: Option<Item>,
    pub equipped_trinkets: Vec<EquippedTrinket>,
    // pub equipped_group_trinket: Option<Item>,
}

#[derive(Debug, Default)]
pub struct CombatEnemy {
    pub current_hp: u32,
    pub unique_id: String,
    pub guid: String,
    pub max_hp: u32,
    pub speed: u32,
    pub physical_attack: u32,
    pub physical_defense: u32,
    pub magical_attack: u32,
    pub magical_defense: u32,
    pub turns_to_action: u8,
    pub total_spell_locks: u8,
    pub spell_locks: UnityList<CombatDamageType>,
    pub damage_type_modifiers:
        UnitySerializableDictionary<DamageTypeModifierKey, DamageTypeModifierValue>,
    pub damage_type_modifiers_override:
        UnitySerializableDictionary<DamageTypeModifierKey, DamageTypeModifierValue>,
    pub fleshmancer_minion: bool,
    pub level: u32,
    pub live_mana_spawn_quantity: u32,
    /// Whether this enemy was summoned (its target `owner.summoned == 1`), e.g. a
    /// boss's adds. Used to bias the appraiser toward killing the boss itself.
    pub summoned: bool,
    /// World position of the target's AOE-overlap anchor — the point the game
    /// centres a splash sphere on when this enemy is an AOE's main target
    /// (`CombatTarget.GetAOEOverlapPosition`, RVA 0xF04BC0). `None` when the
    /// anchor chain is unreadable.
    pub position: Option<Vector3<f32>>,
}

/// One entry of an actor's `allMoveDefinitions`.
///
/// Read by resolved field name (not the `UnityItem` trait) because the walk
/// needs il2cpp field-name resolution, which requires `module` — something
/// `UnityItem::read` doesn't receive. Grows the move's cost/damage-types as
/// appraisals need them.
#[derive(Default, Debug, Clone)]
pub struct CombatMove {
    /// The move's internal id/name (`combatMoveId`), used to tell moves apart
    /// (e.g. basic attack vs a named skill).
    pub move_id: Option<String>,
    /// Combo-point cost (`comboPointCost`).
    pub combo_point_cost: Option<u32>,
    /// Skill-point (MP) cost (`skillPointCost`).
    pub skill_point_cost: Option<u32>,
    /// Whether the move is instantiated for this fight (has a live
    /// `combatMoveComponent`). Locked/unavailable moves in `allMoveDefinitions`
    /// aren't loaded, so this doubles as a usable-this-fight signal.
    pub loaded: bool,
    /// The move component's serialized `specialMovePower` — the flat power term
    /// the special-move damage formula adds to the caster's attack stats. Only
    /// present when the move is loaded.
    pub special_move_power: Option<f32>,
    /// The damage effect's `manaChargeStatMultiplier` — each Live Mana charge
    /// adds this fraction of [`mana_charge_stat`](Self::mana_charge_stat) to
    /// the move's base damage. Only present when the move is loaded and has a
    /// damage effect (a `PlayerSpecialMoveDamage` in its `combatEffects`).
    pub mana_charge_multiplier: Option<f32>,
    /// The damage effect's `manaChargeDamageStat` — the caster stat each Live
    /// Mana charge multiplies (Zale/Valere casters use MagicalAttack).
    pub mana_charge_stat: Option<PlayableCharacterStat>,
    /// Whether the move deals damage: its `damageTypeDefinitions` list is
    /// non-empty. Heals/buffs have no damage types.
    pub is_damaging: bool,
    /// The move definition's `unlockable` flag: `0` for moves available by
    /// default (the base combos: `DualAttack`, `SpectacleStrike`, …), non-zero
    /// for moves that must be learned. Unlike `loaded`, this is present for
    /// combos, so it's how the appraiser tells an available combo apart from a
    /// locked one (party-level combos never have a live `combatMoveComponent`,
    /// so `loaded` is always false for them).
    pub unlockable: Option<i32>,
    /// The characters this move needs in the party to be castable
    /// (`requiredCharacters` on the move definition). Combos list their
    /// participants (e.g. SpectacleStrike needs Garl, DualAttackKids the
    /// kids); empty for moves with no requirement.
    pub required_characters: Vec<String>,
    /// Enemy `unique_id` under this move's single-target cursor, populated only
    /// when this move owns the active target-selector screen.
    pub main_target_guid: Option<String>,
    /// Enemy `unique_id` under this move's AoE cursor, populated only when this
    /// move owns the active target-selector screen.
    pub current_target_guid: Option<String>,
}

/// One party member's available moves, read from their combat actor's
/// `fighterDefinition.allMoveDefinitions`.
#[derive(Default, Debug, Clone)]
pub struct CharacterMoves {
    pub character: PlayerPartyCharacter,
    pub moves: Vec<CombatMove>,
    /// Battle-command class names disabled for this fighter this fight (e.g.
    /// `"BasicAttackBattleCommand"`, `"ComboBattleCommand"`), read from the
    /// fighter's `disabledBattleCommands` set. Tutorials use this to force a
    /// specific command; empty for normal fights.
    pub disabled_commands: Vec<String>,
}

impl CombatMove {
    /// Read a C# `System.String` object (chars at `+0x14`).
    fn read_string_obj(memory_context: &MemoryContext, str_obj: u64) -> Option<String> {
        let chars = memory_context
            .process
            .read_pointer::<ArrayWString<64>>(str_obj + 0x14)
            .ok()?;
        let out = String::from_utf16(chars.as_slice()).ok()?;
        (!out.is_empty()).then_some(out)
    }

    /// Read a C# `System.String` field by name.
    fn read_string(memory_context: &MemoryContext, obj: u64, field: &str) -> Option<String> {
        let str_obj = memory_context.read_named_ptr(obj, field)?;
        Self::read_string_obj(memory_context, str_obj)
    }

    /// Resolve a move entry to its `targetSelectorScreen`.
    ///
    /// Handles both element shapes: a static `MoveDefinition` (which wraps a
    /// `combatMoveComponent`) and a live loaded move that *is* the component.
    fn resolve_screen(memory_context: &MemoryContext, move_ptr: u64) -> Option<u64> {
        let component = memory_context
            .read_named_ptr(move_ptr, "combatMoveComponent")
            .unwrap_or(move_ptr);
        memory_context
            .read_named_ptr(component, "targetSelector")
            .and_then(|ts| memory_context.read_named_ptr(ts, "targetSelectorScreen"))
    }

    /// Resolve a `CombatTarget` to its enemy's `unique_id`:
    /// `target -> owner -> enemy -> uniqueID -> guid`.
    fn resolve_target_guid(memory_context: &MemoryContext, target: u64) -> Option<String> {
        let unique_id = memory_context
            .read_named_ptr(target, "owner")
            .and_then(|p| memory_context.read_named_ptr(p, "enemy"))
            .and_then(|p| memory_context.read_named_ptr(p, "uniqueID"))?;
        Self::read_string(memory_context, unique_id, "guid")
    }

    /// Guid off a named target field on the screen (e.g. `currentTarget`).
    fn screen_target_guid(
        memory_context: &MemoryContext,
        screen: u64,
        field: &str,
    ) -> Option<String> {
        memory_context
            .read_named_ptr(screen, field)
            .and_then(|t| Self::resolve_target_guid(memory_context, t))
    }

    /// The move's `targetSelectorScreen` iff it is `active`. Only the active
    /// screen reflects the live cursor; all others hold a stale last target.
    fn active_screen(memory_context: &MemoryContext, move_ptr: u64) -> Option<u64> {
        let screen = Self::resolve_screen(memory_context, move_ptr)?;
        let active = memory_context
            .field_offset_of(screen, "active")
            .and_then(|off| {
                memory_context
                    .process
                    .read_pointer::<u8>(screen + off as u64)
                    .ok()
            })
            .is_some_and(|b| matches!(b, 1));
        active.then_some(screen)
    }

    /// Read a move: its id/costs and, when it owns the live target cursor, the
    /// enemy under `mainTarget` (single-target) and `currentTarget` (AoE).
    fn read(memory_context: &MemoryContext, move_ptr: u64) -> CombatMove {
        let move_id = Self::read_string(memory_context, move_ptr, "combatMoveId");
        let combo_point_cost = memory_context.read_named::<u32>(move_ptr, "comboPointCost");
        let skill_point_cost = memory_context.read_named::<u32>(move_ptr, "skillPointCost");
        let component = memory_context.read_named_ptr(move_ptr, "combatMoveComponent");
        let loaded = component.is_some();
        // `specialMovePower` lives on the component's `CombatMove` base class;
        // field resolution walks parents, so the read works on any move type.
        let special_move_power =
            component.and_then(|c| memory_context.read_named::<f32>(c, "specialMovePower"));
        // The Live Mana boost fields live on the move's damage effect — the
        // `PlayerSpecialMoveDamage` ScriptableObject in the component's
        // `combatEffects`. Identified by carrying `manaChargeDamageStat` (heal
        // effects have a `manaChargeStatMultiplier` too, but pair it with
        // `manaChargeHealStat`).
        let damage_effect = component.and_then(|c| {
            memory_context
                .read_named_ptr(c, "combatEffects")
                .and_then(|list| {
                    memory_context
                        .list_item_ptrs(list)
                        .into_iter()
                        .filter_map(|entry| memory_context.read_named_ptr(entry, "effect"))
                        .find(|&effect| {
                            memory_context
                                .field_offset_of(effect, "manaChargeDamageStat")
                                .is_some()
                        })
                })
        });
        let mana_charge_multiplier = damage_effect
            .and_then(|e| memory_context.read_named::<f32>(e, "manaChargeStatMultiplier"));
        let mana_charge_stat = damage_effect
            .and_then(|e| memory_context.read_named_ptr(e, "manaChargeDamageStat"))
            .and_then(|s| memory_context.read_named::<i32>(s, "stat"))
            .and_then(PlayableCharacterStat::from_i32);
        // A move deals damage iff its damageTypeDefinitions list has entries.
        let is_damaging = memory_context
            .read_named_ptr(move_ptr, "damageTypeDefinitions")
            .is_some_and(|list| !memory_context.list_item_ptrs(list).is_empty());
        let unlockable = memory_context.read_named::<i32>(move_ptr, "unlockable");
        // `requiredCharacters` is a `List<CharacterDefinitionId>`; the struct
        // wraps a single string, so each list slot is that string's pointer.
        let required_characters = memory_context
            .read_named_ptr(move_ptr, "requiredCharacters")
            .map(|list| {
                memory_context
                    .list_item_ptrs(list)
                    .into_iter()
                    .filter_map(|s| Self::read_string_obj(memory_context, s))
                    .collect()
            })
            .unwrap_or_default();
        let (main_target_guid, current_target_guid) =
            match Self::active_screen(memory_context, move_ptr) {
                Some(screen) => (
                    Self::screen_target_guid(memory_context, screen, "mainTarget"),
                    Self::screen_target_guid(memory_context, screen, "currentTarget"),
                ),
                None => (None, None),
            };
        CombatMove {
            move_id,
            combo_point_cost,
            skill_point_cost,
            loaded,
            special_move_power,
            mana_charge_multiplier,
            mana_charge_stat,
            is_damaging,
            unlockable,
            required_characters,
            main_target_guid,
            current_target_guid,
        }
    }
}

#[derive(Default, Debug)]
pub struct CombatManagerData {
    pub encounter_active: bool,
    pub combat_controller_type: CombatControllerType,
    pub live_mana: LiveMana,
    pub combo_points: u32,
    pub combo_point_progress: u32,
    pub ultimate_progress: f32,
    pub enemies: UnityList<CombatEnemy>,
    pub players: UnityList<CombatPlayer>,
    pub selected_character: Option<PlayerPartyCharacter>,
    /// Whether the top-level battle command ring (Attack/Skill/Combo/Item) has
    /// focus.
    pub battle_command_has_focus: bool,
    /// Highlighted battle command index while the ring has focus
    /// (`Attack=0, Skill=1, Combo=2, Item=3`).
    pub battle_command_index: Option<i64>,
    /// Whether the skill/combo submenu has focus.
    pub skill_command_has_focus: bool,
    /// Highlighted item index in the skill/combo submenu.
    pub skill_command_index: Option<i64>,
    /// `unique_id` (UUID) of the enemy currently under the targeting cursor,
    /// if a target-select is active.
    pub selected_attack_target_guid: Option<String>,
    /// Each party member's available moves, for the appraiser to score.
    pub moves: Vec<CharacterMoves>,
    /// `combatMoveId` of the combo highlighted in the combo submenu, or `None`
    /// when the submenu isn't open.
    pub highlighted_combo_id: Option<String>,
    /// Whether the highlighted combo is currently castable (`canCast`).
    pub highlighted_combo_castable: bool,
    /// `combatMoveId` of the skill highlighted in the skill submenu, or `None`
    /// when the submenu isn't open.
    pub highlighted_skill_id: Option<String>,
    /// Whether the highlighted skill is currently castable (`canCast`).
    pub highlighted_skill_castable: bool,
    /// `GlobalCombatSettings.playerRandomDamageRange` (`min`, `max`), read live.
    /// Constant per session. `max` is the *exclusive* bound the game feeds to
    /// `UnityEngine.Random.RangeInt`, so the inclusive max roll is `max - 1`.
    /// See [`Self::damage_roll_bounds`].
    pub player_random_damage_range: Option<[i32; 2]>,
    /// `GlobalCombatSettings.playerAOERadius`, read live. Constant per session.
    /// The radius of the `Physics.OverlapSphere` a player AOE move casts around
    /// its main target's AOE anchor to gather splash targets
    /// (`PlayerRadiusTargetSelector.SelectAOETargets`, RVA 0x6893D0) — unless
    /// the move's selector overrides it with a `customAOERadius`.
    pub player_aoe_radius: Option<f32>,
    /// Zale's Sunball charge, present only while the charge QTE is active. Lets
    /// the executor hold the charge to max and release on the frame it peaks.
    pub sunball_charge: Option<SunballCharge>,
}

/// Sentinel returned by the game for an unset pointer.
const NULL_POINTER: u64 = 0xFFFF_FFFF;

/// `(Some(id), castable)` if present, else `(None, false)`.
fn split(hit: Option<(String, bool)>) -> (Option<String>, bool) {
    match hit {
        Some((id, castable)) => (Some(id), castable),
        None => (None, false),
    }
}

/// Read the highlighted ability in a submenu (combo or skill): the selector at
/// `offset` must be focused; its `items[selectedItemIndex]` is a
/// `{Combo,SpecialMove}SelectorItem` whose `playerCombatMoveDefinition.combatMoveId`
/// names the move and `canCast` says whether it's castable. `None` otherwise
/// (submenu closed, or the items aren't move items — e.g. the command ring).
fn read_highlighted_move(
    memory_context: &MemoryContext,
    enc: u64,
    offset: u64,
) -> Option<(String, bool)> {
    let selector = memory_context
        .process
        .read_pointer_path::<u64>(enc, &[0x140, 0x50, offset])
        .ok()
        .filter(|s| *s != NULL_POINTER && *s != 0)?;
    // Only trust the highlighted item while this selector has focus.
    memory_context
        .process
        .read_pointer::<u8>(selector + 0x3C)
        .ok()
        .filter(|f| matches!(f, 1))?;
    let idx = memory_context
        .process
        .read_pointer::<i64>(selector + 0x40)
        .ok()?;
    let items = memory_context.read_named_ptr(selector, "items")?;
    let item = *memory_context
        .list_item_ptrs(items)
        .get(usize::try_from(idx).ok()?)?;
    let move_def = memory_context.read_named_ptr(item, "playerCombatMoveDefinition")?;
    let id = CombatMove::read_string(memory_context, move_def, "combatMoveId")?;
    let castable = memory_context
        .field_offset_of(item, "canCast")
        .and_then(|off| {
            memory_context
                .process
                .read_pointer::<u8>(item + off as u64)
                .ok()
        })
        .is_some_and(|b| matches!(b, 1));
    Some((id, castable))
}

/// Read a command-menu selector (`currentEncounter -> 0x140 -> 0x50 -> offset`):
/// `(has_focus, highlighted_index)`. Returns `None` if the selector is unset.
fn read_command_selector(
    memory_context: &MemoryContext,
    enc: u64,
    offset: u64,
) -> Option<(bool, Option<i64>)> {
    let selector = memory_context
        .process
        .read_pointer_path::<u64>(enc, &[0x140, 0x50, offset])
        .ok()
        .filter(|s| *s != NULL_POINTER && *s != 0)?;
    let focus = memory_context
        .process
        .read_pointer::<u8>(selector + 0x3C)
        .map(|f| matches!(f, 1))
        .unwrap_or(false);
    let index = memory_context
        .process
        .read_pointer::<i64>(selector + 0x40)
        .ok();
    Some((focus, index))
}

impl Default for MemoryManager<CombatManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "CombatManager".to_string(),
            data: CombatManagerData::default(),
            manager: UnityMemoryManager::default(),
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

impl MemoryManagerUpdate for CombatManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;

        self.update_encounter_active(&memory_context)?;
        self.update_combat_settings(&memory_context)?;

        // Check if the encounter is active, then run the rest
        // of the updates.
        if self.encounter_active {
            self.update_combat_controller_type(&memory_context)?;
            self.update_live_mana(&memory_context)?;
            self.update_combo_points_and_ultimates(&memory_context)?;
            self.update_enemies(&memory_context)?;
            self.update_players(&memory_context)?;
            self.update_selected_character(&memory_context)?;
            self.update_battle_commands(&memory_context)?;
            self.update_moves(&memory_context)?;
            self.update_sunball_charge(&memory_context)?;
        }

        Ok(())
    }
}

impl CombatManagerData {
    /// Whether an ability submenu (combo or skill) is currently open. The combo
    /// submenu reuses the battle selector, so it's detected by its highlighted
    /// entry; the skill submenu has its own focus flag.
    pub fn ability_submenu_open(&self) -> bool {
        self.highlighted_combo_id.is_some() || self.skill_command_has_focus
    }

    pub fn update_selected_character(
        &mut self,
        _memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        for player in self.players.items.clone() {
            if player.selected {
                self.selected_character = Some(player.character);
                return Ok(());
            }
        }
        self.selected_character = None;

        Ok(())
    }

    /// Read the top-level battle command ring focus + highlighted index.
    ///
    /// TODO(verify-live): pointer path and offsets are ported from
    /// shenef/SoS-TAS `memory/combat_manager.py::_read_battle_commands`. The
    /// game's structs have drifted since (the enemy struct moved +0x10), so
    /// these must be re-derived against the running game before the executor
    /// trusts them. Reads fail-soft to `false`/`None`.
    pub fn update_battle_commands(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        self.battle_command_has_focus = false;
        self.battle_command_index = None;
        self.skill_command_has_focus = false;
        self.skill_command_index = None;

        self.highlighted_combo_id = None;

        let Ok(enc) = memory_context.follow_fields::<u64>(&["currentEncounter"]) else {
            return Ok(());
        };

        // Top-level command ring (Attack/Skill/Combo/Item) at 0x68.
        if let Some((focus, index)) = read_command_selector(memory_context, enc, 0x68) {
            self.battle_command_has_focus = focus;
            self.battle_command_index = index;
        }
        // Skill/Combo submenu selector at 0x78 (same layout).
        if let Some((focus, index)) = read_command_selector(memory_context, enc, 0x78) {
            self.skill_command_has_focus = focus;
            self.skill_command_index = index;
        }

        // The move highlighted in the ability submenus. Both submenus reuse a
        // command selector whose `items` become `{Combo,SpecialMove}SelectorItem`s
        // — the highlighted one's `playerCombatMoveDefinition.combatMoveId` names
        // the ability and `canCast` says whether it's castable. Combos live on the
        // battle selector (0x68), skills on the skill selector (0x78). Cleared
        // when the respective submenu isn't open.
        (self.highlighted_combo_id, self.highlighted_combo_castable) =
            split(read_highlighted_move(memory_context, enc, 0x68));
        (self.highlighted_skill_id, self.highlighted_skill_castable) =
            split(read_highlighted_move(memory_context, enc, 0x78));

        Ok(())
    }

    /// Enumerate each party member's moves and derive the live target cursor.
    ///
    /// Reads every actor's `fighterDefinition.allMoveDefinitions` into
    /// [`CharacterMoves`] (aligned to `players` order, which shares the
    /// `playerActors` list). In the same pass, the enemy under the active
    /// move's target-selector screen is captured as `selected_attack_target_guid`
    /// (there is no direct cursor field). All field-name resolved, so it survives
    /// struct-layout drift.
    pub fn update_moves(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        self.selected_attack_target_guid = None;
        self.moves.clear();

        let Ok(enc) = memory_context.follow_fields::<u64>(&["currentEncounter"]) else {
            return Ok(());
        };
        let Some(actors) = memory_context.read_named_ptr(enc, "playerActors") else {
            return Ok(());
        };

        // `players` is read from the same `playerActors` list earlier this frame,
        // so it aligns by index with the actors we iterate here.
        let characters: Vec<PlayerPartyCharacter> = self
            .players
            .items
            .iter()
            .map(|p| p.character.clone())
            .collect();

        // Every move's target-selector screen reflects the same live cursor;
        // prefer `mainTarget` (single-target) over `currentTarget` (AoE).
        let mut main_hit: Option<String> = None;
        let mut current_hit: Option<String> = None;

        for (index, actor) in memory_context
            .list_item_ptrs(actors)
            .into_iter()
            .enumerate()
        {
            let Some(fighter_def) = memory_context.read_named_ptr(actor, "fighterDefinition")
            else {
                continue;
            };

            // The commands the game has disabled for this fighter this fight
            // (a HashSet<Type> of *BattleCommand classes), resolved to their
            // class names. Tutorials use this to force a specific command.
            let disabled_commands = memory_context
                .read_named_ptr(fighter_def, "disabledBattleCommands")
                .map(|set| {
                    memory_context
                        .hashset_item_ptrs(set)
                        .into_iter()
                        .filter_map(|type_obj| memory_context.type_class_name::<64>(type_obj))
                        .collect()
                })
                .unwrap_or_default();

            let Some(move_list) = memory_context.read_named_ptr(fighter_def, "allMoveDefinitions")
            else {
                continue;
            };

            let mut moves = Vec::new();
            for move_ptr in memory_context.list_item_ptrs(move_list) {
                let move_def = CombatMove::read(memory_context, move_ptr);
                main_hit = main_hit.or(move_def.main_target_guid.clone());
                current_hit = current_hit.or(move_def.current_target_guid.clone());
                moves.push(move_def);
            }

            self.moves.push(CharacterMoves {
                character: characters.get(index).cloned().unwrap_or_default(),
                moves,
                disabled_commands,
            });
        }

        self.selected_attack_target_guid = main_hit.or(current_hit);
        Ok(())
    }

    /// Read Zale's live Sunball charge while the charge QTE is active.
    ///
    /// The charge lives on the `SunboyShootQTESunballState`, but reading it off
    /// `stateMachine.currentState` is unreliable: that pointer flickers off the
    /// state for stray frames (transitions, sub-states), and since releasing
    /// Confirm for even one frame fires the sunball early, a momentary miss ruins
    /// the cast. Instead we find the state's *persistent, pooled instance* in
    /// `stateMachine.stateInstances` (a stable object that never flickers) and
    /// gate "actively charging" on `sunballChargeDuration`, which the game sets
    /// positive on enter and resets to `-1` on exit. From that instance we read
    /// the projectile's `level` (its `sunballProjectileInstance.level`), the
    /// ceiling `sunballMaxLevel`, and `currentStep`. All field-name resolved, so
    /// it survives struct-layout drift. Cleared to `None` when no actor is
    /// charging.
    pub fn update_sunball_charge(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        self.sunball_charge = None;

        let Ok(enc) = memory_context.follow_fields::<u64>(&["currentEncounter"]) else {
            return Ok(());
        };
        let Some(actors) = memory_context.read_named_ptr(enc, "playerActors") else {
            return Ok(());
        };

        for actor in memory_context.list_item_ptrs(actors) {
            let Some(states) = memory_context
                .read_named_ptr(actor, "stateMachine")
                .and_then(|sm| memory_context.read_named_ptr(sm, "stateInstances"))
            else {
                continue;
            };

            // Find the charge state's persistent instance by its il2cpp class.
            // Unlike `currentState`, this list entry is stable frame-to-frame.
            let Some(state) = memory_context
                .list_item_ptrs(states)
                .into_iter()
                .find(|&s| {
                    Class::from_object(memory_context.process, s)
                        .and_then(|class| {
                            class
                                .class_name::<64>(memory_context.process, memory_context.module)
                                .ok()
                        })
                        .and_then(|name| name.validate_utf8().ok().map(str::to_string))
                        .is_some_and(|name| name == "SunboyShootQTESunballState")
                })
            else {
                continue;
            };

            // A charge is on screen only while the state is active: its duration
            // is set strictly positive on enter and reset to -1 on exit (and is
            // 0 on the pooled instance before the first cast) — so `> 0` cleanly
            // distinguishes an active cast from idle.
            let charging = memory_context
                .read_named::<f32>(state, "sunballChargeDuration")
                .is_some_and(|d| d > 0.0);
            if !charging {
                continue;
            }

            let max_level = memory_context
                .read_named::<i32>(state, "sunballMaxLevel")
                .unwrap_or(0);
            let level = memory_context
                .read_named_ptr(state, "sunballProjectileInstance")
                .and_then(|projectile| memory_context.read_named::<i32>(projectile, "level"))
                .unwrap_or(0);
            let step = SunballChargeStep::from_i32(
                memory_context
                    .read_named::<i32>(state, "currentStep")
                    .unwrap_or(-1),
            );

            self.sunball_charge = Some(SunballCharge {
                level,
                max_level,
                step,
            });
            return Ok(());
        }

        Ok(())
    }

    pub fn update_encounter_active(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(encounter_done) =
            memory_context.follow_fields::<u8>(&["currentEncounter", "encounterDone"])
        {
            self.encounter_active = matches!(encounter_done, 0)
        } else {
            self.encounter_active = false;
        }

        Ok(())
    }

    /// Identify the active encounter's controller by reflecting the concrete
    /// class name of `currentEncounter.controller`.
    ///
    /// The controller object's il2cpp class name *is* the encounter type
    /// (`EncounterController` for normal fights, `KidsCavernEncounter`,
    /// `FirstEncounter`, the tutorials, …)
    pub fn update_combat_controller_type(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        let Ok(enc) = memory_context.follow_fields::<u64>(&["currentEncounter"]) else {
            return Ok(());
        };
        let Some(controller) = memory_context.read_named_ptr(enc, "controller") else {
            return Ok(());
        };
        let Some(class) = Class::from_object(memory_context.process, controller) else {
            return Ok(());
        };
        let Ok(name) = class.class_name::<64>(memory_context.process, memory_context.module) else {
            return Ok(());
        };
        let Ok(name) = name.validate_utf8() else {
            return Ok(());
        };

        self.combat_controller_type = match name {
            "EncounterController" => CombatControllerType::Basic,
            "FirstEncounter" => CombatControllerType::FirstEncounter,
            "SecondEncounter" => CombatControllerType::SecondEncounter,
            "DwellerOfStrife" => CombatControllerType::DwellerOfStrife,
            "DwellerOfDread" => CombatControllerType::DwellerOfDread,
            "KOTutorial" => CombatControllerType::KOTutorial,
            "LiveManaTutorial" => CombatControllerType::LiveManaTutorial,
            "ManaRegenTutorial" => CombatControllerType::ManaRegenTutorial,
            "RoundsTutorial" => CombatControllerType::RoundsTutorial,
            "SpellLockTutorial" => CombatControllerType::SpellLockTutorial,
            "TimedBlocksTutorial" => CombatControllerType::TimedBlocksTutorial,
            "TimedHitsTutorial" => CombatControllerType::TimedHitsTutorial,
            "KidsCavernEncounter" => CombatControllerType::KidsCavernEncounter,
            other => {
                // Unmapped controllers fall back to Basic; log the raw name
                // (debug) so we can model new encounters explicitly.
                log::debug!("unmapped combat controller: {other}");
                CombatControllerType::Basic
            }
        };

        Ok(())
    }

    /// Read the global combat-balance settings the damage formulas depend on
    /// (currently just the basic-attack random damage roll range). These are
    /// constant per session, so this reads once and keeps the value; it
    /// fail-softs, leaving the previous value on a miss.
    pub fn update_combat_settings(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if self.player_random_damage_range.is_none()
            && let Ok(range) = memory_context
                .follow_fields::<[i32; 2]>(&["globalCombatSettings", "playerRandomDamageRange"])
        {
            self.player_random_damage_range = Some(range);
        }
        if self.player_aoe_radius.is_none()
            && let Ok(radius) =
                memory_context.follow_fields::<f32>(&["globalCombatSettings", "playerAOERadius"])
        {
            self.player_aoe_radius = Some(radius);
        }
        Ok(())
    }

    /// The basic-attack random damage roll bounds `(min_roll, max_roll)`,
    /// inclusive on both ends. The game rolls `Random.RangeInt(min, max)` which
    /// is max-*exclusive*, so the inclusive max roll is `max - 1`. Falls back to
    /// the historical constants until [`Self::update_combat_settings`] has read
    /// the live values.
    pub fn damage_roll_bounds(&self) -> (f32, f32) {
        match self.player_random_damage_range {
            Some([min, max]) => (min as f32, (max - 1) as f32),
            None => (damage::MIN_ROLL, damage::MAX_ROLL),
        }
    }

    pub fn update_live_mana(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        // Each particle list's live count is the `List<T>._size` field at
        // `list + 0x18`. An unreadable handler (no live-mana fight yet, or a
        // mid-transition null) reads as an empty pool.
        let particle_count = |list_field: &str| -> u32 {
            memory_context
                .follow_fields::<u64>(&["currentEncounter", "liveManaHandler", list_field])
                .ok()
                .filter(|list| *list != 0)
                .and_then(|list| memory_context.read_pointer::<u32>(list + 0x18).ok())
                .unwrap_or(0)
        };
        self.live_mana.small = particle_count("smallLiveManaParticles");
        self.live_mana.big = particle_count("bigLiveManaParticles");

        Ok(())
    }

    pub fn update_combo_points_and_ultimates(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(combo_points_panel_ptr) = memory_context.follow_fields::<u64>(&[
            "currentEncounter",
            "controller",
            "battleUI",
            "comboPointsPanel",
        ]) {
            // comboPointsPanel -> ultMeter -> targetFill
            if let Ok(combo_point_progress) = memory_context
                .process
                .read_pointer_path::<u32>(combo_points_panel_ptr, &[0x30, 0x58])
            {
                self.combo_point_progress = combo_point_progress
            } else {
                self.combo_point_progress = 0
            }
            if let Ok(combo_points) = memory_context
                .process
                .read_pointer_path::<u32>(combo_points_panel_ptr, &[0x30, 0x5C])
            {
                self.combo_points = combo_points
            } else {
                self.combo_points = 0
            }
            // comboPointsPanel -> comboPointsMeter -> currentComboPoints
            if let Ok(ultimate_progress) = memory_context
                .process
                .read_pointer_path::<f32>(combo_points_panel_ptr, &[0x28, 0x40])
            {
                self.ultimate_progress = ultimate_progress
            } else {
                self.ultimate_progress = 0.0
            }
        }

        Ok(())
    }

    pub fn update_enemies(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok(enemy_targets) =
            memory_context.follow_fields::<u64>(&["currentEncounter", "enemyTargets"])
        {
            self.enemies = UnityList::<CombatEnemy>::read(memory_context.process, enemy_targets)?;
            // Resolve each enemy's `summoned` flag from its target:
            // enemyTargets.items[x] -> item -> owner -> summoned. Aligned by index
            // with the list read above (both walk the same `_items` array).
            for (target, enemy) in memory_context
                .list_item_ptrs(enemy_targets)
                .into_iter()
                .zip(self.enemies.items.iter_mut())
            {
                enemy.summoned = memory_context
                    .read_named_ptr(target, "owner")
                    .and_then(|owner| memory_context.read_named::<u8>(owner, "summoned"))
                    .is_some_and(|summoned| summoned == 1);
                enemy.position = Self::aoe_anchor_position(memory_context, target);
            }
        }
        Ok(())
    }

    /// The world position an AOE sphere is centred on when `target` is the
    /// main target: `dependencies.aoeOverlapPosition`'s Transform position
    /// (mirrors `CombatTarget.GetAOEOverlapPosition`). The Transform is read
    /// through its managed wrapper's native object (`m_CachedPtr` at +0x10,
    /// local position at +0x90 in this Unity build). Combat actors sit under
    /// identity parents, so local position == world position here.
    fn aoe_anchor_position(memory_context: &MemoryContext, target: u64) -> Option<Vector3<f32>> {
        // Explicit anchor, when the target has one set.
        if let Some(anchor) = memory_context
            .read_named_ptr(target, "dependencies")
            .and_then(|deps| memory_context.read_named_ptr(deps, "aoeOverlapPosition"))
            && let Some(pos) = Self::transform_position(memory_context, anchor)
        {
            return Some(pos);
        }
        // Fallback (matches the game's): the target's own transform. The
        // managed MonoBehaviour's native component links to its GameObject
        // (+0x30), whose component array (+0x30) holds the Transform first.
        let native_component = Self::native_object(memory_context, target)?;
        let game_object = memory_context
            .process
            .read_pointer::<u64>(native_component + 0x30)
            .ok()
            .filter(|p| *p != 0)?;
        let components = memory_context
            .process
            .read_pointer::<u64>(game_object + 0x30)
            .ok()
            .filter(|p| *p != 0)?;
        let native_transform = memory_context
            .process
            .read_pointer::<u64>(components + 0x8)
            .ok()
            .filter(|p| *p != 0)?;
        Self::native_transform_position(memory_context, native_transform)
    }

    /// A managed Unity object's native counterpart (`m_CachedPtr` at +0x10).
    fn native_object(memory_context: &MemoryContext, managed: u64) -> Option<u64> {
        memory_context
            .process
            .read_pointer::<u64>(managed + 0x10)
            .ok()
            .filter(|p| *p != 0)
    }

    /// Position of a *managed* Transform, through its native object.
    fn transform_position(memory_context: &MemoryContext, transform: u64) -> Option<Vector3<f32>> {
        let native = Self::native_object(memory_context, transform)?;
        Self::native_transform_position(memory_context, native)
    }

    /// World position of a *native* Transform, computed from Unity's transform
    /// hierarchy: the native Transform holds a `TransformAccess` (hierarchy ptr
    /// at +0x38, node index at +0x40); the hierarchy stores per-node local TRS
    /// blocks (48 bytes: translation, rotation, scale — ptr at +0x18) and
    /// parent indices (+0x20). Battle hierarchies carry no rotation/scale, so
    /// the world position is the sum of local translations up the parent chain.
    fn native_transform_position(
        memory_context: &MemoryContext,
        native: u64,
    ) -> Option<Vector3<f32>> {
        const TRS_STRIDE: u64 = 48;
        let process = memory_context.process;
        let hierarchy = process
            .read_pointer::<u64>(native + 0x38)
            .ok()
            .filter(|p| *p != 0)?;
        let mut index = process.read_pointer::<i32>(native + 0x40).ok()?;
        let local_trs = process
            .read_pointer::<u64>(hierarchy + 0x18)
            .ok()
            .filter(|p| *p != 0)?;
        let parents = process
            .read_pointer::<u64>(hierarchy + 0x20)
            .ok()
            .filter(|p| *p != 0)?;

        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
        // Bounded walk so a bad read can't loop forever.
        for _ in 0..64 {
            if index < 0 {
                return Some(Vector3::new(x, y, z));
            }
            let [tx, ty, tz] = process
                .read_pointer::<[f32; 3]>(local_trs + index as u64 * TRS_STRIDE)
                .ok()?;
            x += tx;
            y += ty;
            z += tz;
            index = process
                .read_pointer::<i32>(parents + index as u64 * 4)
                .ok()?;
        }
        None
    }

    pub fn update_players(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok(players) =
            memory_context.follow_fields::<u64>(&["currentEncounter", "playerActors"])
        {
            let players = UnityList::<CombatPlayer>::read(memory_context.process, players)?;
            self.players = players;
        }
        Ok(())
    }
}

impl UnityItem for CombatEnemy {
    fn read(process: &Process, item_ptr: u64) -> Result<Self, MemoryError> {
        // Top level pointers
        let enemy_data = process.read_pointer_path::<u64>(item_ptr, &[0x80, 0x108])?;
        let casting_data = process.read_pointer_path::<u64>(item_ptr, &[0x80, 0x120])?;
        let current_hp = process.read_pointer_path::<u32>(item_ptr, &[0x9C])?;
        let guid_w_str =
            process.read_pointer_path::<ArrayWString<128>>(enemy_data, &[0x18, 0x14])?;
        let guid = String::from_utf16(guid_w_str.as_slice()).unwrap_or("Unknown".to_string());

        // enemy data
        let unique_id_w_str = process
            .read_pointer_path::<ArrayWString<36>>(item_ptr, &[0x80, 0xF8, 0xF0, 0x18, 0x14])?;
        let unique_id =
            String::from_utf16(unique_id_w_str.as_slice()).unwrap_or("Unknown".to_string());
        let max_hp = process.read_pointer::<u32>(enemy_data + 0x30)?;
        let speed = process.read_pointer::<u32>(enemy_data + 0x34)?;
        let physical_attack = process.read_pointer::<u32>(enemy_data + 0x3C)?;
        let physical_defense = process.read_pointer::<u32>(enemy_data + 0x38)?;
        let magical_attack = process.read_pointer::<u32>(enemy_data + 0x40)?;
        let magical_defense = process.read_pointer::<u32>(enemy_data + 0x44)?;

        // casting data
        let turns_to_action = process.read_pointer::<u8>(casting_data + 0x2C)?;
        let total_spell_locks = process.read_pointer::<u8>(casting_data + 0x30)?;

        let mut spell_locks = UnityList::<CombatDamageType>::default();

        if turns_to_action > 0 && total_spell_locks > 0 {
            spell_locks = if let Ok(locks) = process.read_pointer_path::<u64>(casting_data, &[0x20])
            {
                UnityList::<CombatDamageType>::read(process, locks)?
            } else {
                UnityList::<CombatDamageType>::default()
            };
        }

        let damage_type_modifiers_ptr =
            process.read_pointer_path::<u64>(item_ptr, &[0x80, 0x108, 0x20])?;
        let damage_type_modifiers = UnitySerializableDictionary::<
            DamageTypeModifierKey,
            DamageTypeModifierValue,
        >::read(
            process, damage_type_modifiers_ptr, 0x10, 0x8, 0xC
        )?;

        let damage_type_modifiers_override_ptr =
            process.read_pointer_path::<u64>(item_ptr, &[0x80, 0x108, 0x28])?;
        let damage_type_modifiers_override =
            UnitySerializableDictionary::<DamageTypeModifierKey, DamageTypeModifierValue>::read(
                process,
                damage_type_modifiers_override_ptr,
                0x10,
                0x8,
                0xC,
            )?;

        let live_mana_spawn_quantity = process.read_pointer::<u32>(enemy_data + 0x58)?;
        let level = process.read_pointer::<u32>(enemy_data + 0x5C)?;
        let fleshmancer_minion = if let Ok(minion) = process.read_pointer::<u32>(enemy_data + 0x60)
        {
            matches!(minion, 1)
        } else {
            false
        };

        Ok(CombatEnemy {
            guid,
            unique_id,
            current_hp,
            max_hp,
            speed,
            physical_attack,
            physical_defense,
            magical_attack,
            magical_defense,
            turns_to_action,
            total_spell_locks,
            spell_locks,
            damage_type_modifiers,
            damage_type_modifiers_override,
            fleshmancer_minion,
            level,
            live_mana_spawn_quantity,
            // Resolved in `update_enemies`, which has the memory context needed
            // to walk the field-name paths (`item -> owner -> summoned`, the
            // AOE anchor transform).
            summoned: false,
            position: None,
        })
    }
}

impl UnityItem for CombatPlayer {
    fn read(process: &Process, item_ptr: u64) -> Result<Self, MemoryError> {
        // Top level pointers
        // max_hp/mp may be 0x58 instead of 0x50
        let current_hp = process.read_pointer_path::<u32>(item_ptr, &[0x188, 0x28, 0x58])?;
        let current_mp = process.read_pointer_path::<u32>(item_ptr, &[0x188, 0x30, 0x58])?;

        let base_hp = process.read_pointer_path::<u32>(item_ptr, &[0x158, 0x30, 0x78, 0x20])?;
        let base_mp = process.read_pointer_path::<u32>(item_ptr, &[0x158, 0x30, 0x78, 0x24])?;

        // this get the current level up upgrades + 1, so if they're level 2 they should
        // have one upgrade.
        let level =
            process.read_pointer_path::<u32>(item_ptr, &[0x158, 0x30, 0x78, 0x120, 0x18])? + 1;

        let base_physical_defense =
            process.read_pointer_path::<u32>(item_ptr, &[0x158, 0x38, 0x30])?;
        let base_physical_attack =
            process.read_pointer_path::<u32>(item_ptr, &[0x158, 0x38, 0x28])?;
        let base_magical_attack =
            process.read_pointer_path::<u32>(item_ptr, &[0x158, 0x38, 0x2C])?;
        let base_magical_defense =
            process.read_pointer_path::<u32>(item_ptr, &[0x158, 0x38, 0x30])?;

        let selected = if let Ok(sel) = process.read_pointer_path::<u32>(item_ptr, &[0x188, 0x78]) {
            matches!(sel, 1)
        } else {
            false
        };

        let character = if let Ok(char) =
            process.read_pointer_path::<ArrayWString<128>>(item_ptr, &[0x188, 0x70, 0x14])
        {
            if let Ok(name) = String::from_utf16(char.as_slice()) {
                PlayerPartyCharacter::parse(&name)
            } else {
                PlayerPartyCharacter::None
            }
        } else {
            PlayerPartyCharacter::None
        };

        let timed_attack_ready =
            if let Ok(tar) = process.read_pointer_path::<u8>(item_ptr, &[0x168, 0x3A]) {
                matches!(tar, 1)
            } else {
                false
            };

        let dead = if let Ok(tar) = process.read_pointer_path::<u8>(item_ptr, &[0xD0]) {
            matches!(tar, 1)
        } else {
            false
        };

        let enabled = if let Ok(char_enabled) =
            process.read_pointer_path::<u8>(item_ptr, &[0x188, 0x68, 0x30])
        {
            matches!(char_enabled, 1)
        } else {
            false
        };
        let mana_charge_count = process.read_pointer_path::<u32>(item_ptr, &[0x150, 0x58])?;

        let equipped_weapon = if let Ok(weapon_guid) = process
            .read_pointer_path::<ArrayWString<128>>(item_ptr, &[0x158, 0x38, 0xB0, 0x18, 0x14])
        {
            if let Ok(name) = String::from_utf16(weapon_guid.as_slice()) {
                weapons().get(name.as_str()).cloned()
            } else {
                None
            }
        } else {
            None
        };

        let equipped_armor = if let Ok(armor_guid) = process
            .read_pointer_path::<ArrayWString<128>>(item_ptr, &[0x158, 0x38, 0xB8, 0x18, 0x14])
        {
            if let Ok(name) = String::from_utf16(armor_guid.as_slice()) {
                armor().get(name.as_str()).cloned()
            } else {
                None
            }
        } else {
            None
        };

        let equipped_trinkets = if let Ok(equipped_trinkets_ptr) =
            process.read_pointer_path::<u64>(item_ptr, &[0x158, 0x38, 0xC0])
        {
            UnityList::<EquippedTrinket>::read(process, equipped_trinkets_ptr)?
        } else {
            UnityList::<EquippedTrinket>::default()
        };

        // TODO(eein): Implement equipped group trinket once i can verify it.
        // let equipped_group_trinket = if let Ok(group_trinket_guid) = process.read_pointer_path::<ArrayWString<128>>(item_ptr, &[0x150, 0x30, 0xA8, 0x18, 0x14]){
        //     if let Ok(name) = String::from_utf16(armor_guid.as_slice()) {
        //         armor().get(name.as_str()).cloned()
        //     } else {
        //         None
        //     }
        // } else {
        //     None
        // };

        let mut max_hp = base_hp;
        let mut max_mp = base_mp;
        let mut physical_attack = base_physical_attack;
        let mut physical_defense = base_physical_defense;
        let mut magical_attack = base_magical_attack;
        let mut magical_defense = base_magical_defense;

        // Adds weapon stats
        if let Some(ref item) = equipped_weapon {
            physical_attack += item.physical_attack;
            physical_defense += item.physical_defense;
            magical_attack += item.magical_attack;
            magical_defense += item.magical_defense;
        }

        // Adds armor stats
        if let Some(ref item) = equipped_armor {
            physical_attack += item.physical_attack;
            physical_defense += item.physical_defense;
            magical_attack += item.magical_attack;
            magical_defense += item.magical_defense;
        }

        let stats =
            data::level_up_tables::get_sum_of_character_stats_by_level(character.clone(), level);
        max_hp += stats.hp;
        max_mp += stats.mp;
        physical_attack += stats.physical_attack;
        physical_attack += stats.physical_attack;
        magical_attack += stats.magical_attack;
        magical_defense += stats.magical_defense;

        Ok(CombatPlayer {
            level,
            max_hp,
            max_mp,
            current_hp,
            current_mp,
            base_physical_attack,
            base_physical_defense,
            base_magical_attack,
            base_magical_defense,
            selected,
            character,
            timed_attack_ready,
            dead,
            enabled,
            mana_charge_count,
            equipped_weapon,
            equipped_armor,
            equipped_trinkets: equipped_trinkets.items,
            // TODO(eein): find a group trinket and add this
            // equipped_group_trinket: None,
            physical_attack,
            physical_defense,
            magical_attack,
            magical_defense,
        })
    }
}
impl UnityItem for EquippedTrinket {
    fn read(process: &Process, item_ptr: u64) -> Result<EquippedTrinket, MemoryError> {
        let item_name = process.read_pointer_path::<ArrayWString<128>>(item_ptr, &[0x18, 0x14])?;
        if let Ok(name) = String::from_utf16(item_name.as_slice()) {
            let equipped_trinket = trinkets().get(name.as_str()).unwrap().clone();
            Ok(EquippedTrinket {
                trinket: Some(equipped_trinket),
            })
        } else {
            Ok(EquippedTrinket { trinket: None })
        }
    }
}

impl UnityItem for CombatDamageType {
    fn read(process: &Process, item_ptr: u64) -> Result<Self, MemoryError> {
        let lock = process.read_pointer::<u32>(item_ptr + 0x30)?;
        Ok(CombatDamageType::from_u32(lock))
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct DamageTypeModifierKey {
    pub key: CombatDamageType,
}

#[derive(Default, Debug)]
pub struct DamageTypeModifierValue {
    pub value: f32,
}

impl PartialEq for DamageTypeModifierValue {
    fn eq(&self, other: &Self) -> bool {
        (self.value - other.value).abs() < 0.001
    }
}

impl UnitySerializableDictKey for DamageTypeModifierKey {
    fn read(process: &Process, item_ptr: u64) -> Result<Self, MemoryError> {
        if let Ok(damage_type) = process.read::<u32>(item_ptr) {
            Ok(DamageTypeModifierKey {
                key: CombatDamageType::from_u32(damage_type),
            })
        } else {
            Err(MemoryError::ReadError)
        }
    }
}

impl UnitySerializableDictValue for DamageTypeModifierValue {
    fn read(process: &Process, item_ptr: u64) -> Result<Self, MemoryError> {
        if let Ok(value) = process.read::<f32>(item_ptr) {
            Ok(DamageTypeModifierValue { value })
        } else {
            Err(MemoryError::ReadError)
        }
    }
}
