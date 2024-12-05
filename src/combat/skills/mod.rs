use crate::memory::combat_manager::CombatDamageType;
use crate::state::GameState;

trait Action {
    fn is_usable(&self, _ctx: GameState) -> bool {
        false
    }
    // fn execute(&self, ctx: GameState, step: ActionStep) {
    //     match step {
    //         ActionStep::SelectingCommand => self.execute_selecting_command(ctx),
    //         // ActionStep::Boost => self.execute_boost(ctx),
    //         ActionStep::ConfirmCommand => self.execute_confirm_command(ctx),
    //         ActionStep::SelectingSkill => self.execute_select_skill(ctx),
    //         ActionStep::ConfirmSkill => self.execute_confirm_skill(ctx),
    //         ActionStep::SelectingEnemySequence => self.execute_selecting_enemy_sequence(ctx),
    //         ActionStep::ConfirmEnemySequence => self.execute_confirm_enemy_sequence(ctx),
    //         // ActionStep::SelectingPlayerSequence => {},
    //         ActionStep::TimingSequence => self.execute_timing_sequence(ctx),
    //         ActionStep::ActionComplete => (),
    //     }
    // }
    // fn execute_boost(&self, _ctx: GameState) {}
    // fn execute_selecting_command(&self, _ctx: GameState) {}
    // fn execute_confirm_command(&self, _ctx: GameState) {}
    // fn execute_select_skill(&self, _ctx: GameState) {}
    // fn execute_confirm_skill(&self, _ctx: GameState) {}
    // fn execute_selecting_enemy_sequence(&self, _ctx: GameState) {}
    // fn execute_confirm_enemy_sequence(&self, _ctx: GameState) {}
    // // fn execute_selecting_player_sequence(&self, _ctx: GameState) {}
    // fn execute_complete(&self, _ctx: GameState) {}
    
    // Returns true when timing sequence is done
    fn execute_timing_sequence(&self, _ctx: GameState) -> bool;
    fn calculate_value(&self, _ctx: GameState) -> u32 {
        // We would do a check for the type of skill and process that or override if its spaghetti
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
    Aoe,
    All,
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

struct Skill {
    internal_name: &'static str,
    timing_type: TimingType,
    target_type: TargetType,
    damage_types: Vec<CombatDamageType>,
    resource: SkillResource,
    battle_command: BattleCommand,
    cost: u32,
}

struct ZaleBasicAttack {
    data: Skill
}

impl Action for ZaleBasicAttack {
    fn execute_timing_sequence(&self, _ctx: GameState) -> bool {
        true
    }
}



// ZaleBasicAttack {
//     name: "Zale Basic Attack",
//     internal_name: "Combat",
//     timing_type: TimingType::MultiHit,
//     resource: SkillResource::None,
//     damage_types: &[CombatDamageType::Sword],
//     battle_command: BattleCommand::Attack,
//     cost: 0,
// }


