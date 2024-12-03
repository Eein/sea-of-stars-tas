use crate::memory::combat_manager::CombatDamageType;
use crate::state::GameState;

enum ActionStep {
    SelectingCommand,
    Boost,
    ConfirmCommand,
    SelectingSkill,
    ConfirmSkill,
    SelectingCombo,
    ConfirmCombo,
    SelectingEnemySequence,
    ConfirmEnemySequence,
    SelectingPlayerSequence,
    TimingSequence,
    ActionComplete,
}

trait Action {
    fn is_usable(&self, _ctx: GameState) -> bool {
        false
    }
    fn execute(&self, ctx: GameState, step: ActionStep) {
        match step {
            ActionStep::SelectingCommand => self.execute_selecting_command(ctx),
            ActionStep::Boost => self.execute_boost(ctx),
            ActionStep::ConfirmCommand => self.execute_confirm_command(ctx),
            ActionStep::SelectingSkill => self.execute_select_skill(ctx),
            ActionStep::ConfirmSkill => self.execute_confirm_skill(ctx),
            ActionStep::SelectingCombo => self.execute_select_combo(ctx),
            ActionStep::ConfirmCombo => self.execute_confirm_combo(ctx),
            ActionStep::SelectingEnemySequence => self.execute_selecting_enemy_sequence(ctx),
            ActionStep::ConfirmEnemySequence => self.execute_confirm_enemy_sequence(ctx),
            ActionStep::SelectingPlayerSequence => {},
            ActionStep::TimingSequence => self.execute_timing_sequence(ctx),
            ActionStep::ActionComplete => (),
        }
    }
    fn execute_timing_sequence(&self, _ctx: GameState) {}
    fn execute_selecting_command(&self, _ctx: GameState) {}
    fn execute_boost(&self, _ctx: GameState) {}
    fn execute_confirm_command(&self, _ctx: GameState) {}
    fn execute_select_skill(&self, _ctx: GameState) {}
    fn execute_confirm_skill(&self, _ctx: GameState) {}
    fn execute_select_combo(&self, _ctx: GameState) {}
    fn execute_confirm_combo(&self, _ctx: GameState) {}
    fn execute_selecting_enemy_sequence(&self, _ctx: GameState) {}
    fn execute_confirm_enemy_sequence(&self, _ctx: GameState) {}
    // fn execute_selecting_player_sequence(&self, _ctx: GameState) {}
    fn execute_complete(&self, _ctx: GameState) {}
    fn calculate_value(&self, _ctx: GameState) -> u32 {
        0
    }
}

enum TimingType {
    None,
    OneHit,
    Charge,
    MultiHit,
}

enum TargetType {
    None,
    Player,
    Enemy,
}

enum SkillResource {
    None,
    Mana,
    ComboPoints,
    UltimateGuage,
}

enum BattleCommand {
    Attack,
    Skill,
    Combo,
    Item,
}

struct ZaleBasicAttack {
    action_step: ActionStep,
    internal_name: &'static str,
    timing_type: TimingType,
    damage_types: Vec<CombatDamageType>,
    resource: SkillResource,
    battle_command: BattleCommand,
    cost: u32,
}

impl Action for ZaleBasicAttack {}

// ZaleBasicAttack {
//     name: "Zale Basic Attack",
//     internal_name: "Combat",
//     timing_type: TimingType::MultiHit,
//     resource: SkillResource::None,
//     damage_types: &[CombatDamageType::Sword],
//     battle_command: BattleCommand::Attack,
//     cost: 0,
// }


