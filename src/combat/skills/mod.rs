use data::prelude::PlayerPartyCharacter;

use crate::memory::combat_manager::CombatDamageType;
use crate::state::GameState;

pub trait Action {
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
    fn calculate_value(&self, _ctx: GameState) -> u32 {
        // We would do a check for the type of skill and process that or override if its spaghetti
        0
    }
}

pub enum TimingType {
    None,
    OneHit,
    Charge,
    MultiHit,
}

pub enum TargetType {
    None,
    Player,
    Enemy,
    Aoe,
    All,
}

pub enum SkillResource {
    None,
    Mana,
    ComboPoints,
    UltimateGuage,
}

pub enum BattleCommand {
    Attack,
    Skill,
    Combo,
    Item,
}

pub struct Skill {
    pub character: PlayerPartyCharacter,
    pub internal_name: &'static str,
    pub timing_type: TimingType,
    pub target_type: TargetType,
    pub damage_types: Vec<CombatDamageType>,
    pub resource: SkillResource,
    pub battle_command: BattleCommand,
    pub cost: u32,
    pub timing_controller: Box<dyn Timing>,
}

pub trait Timing {
    fn execute_timing_sequence(&self, _ctx: GameState) -> bool;
}

pub struct BasicAttack;
impl Timing for BasicAttack {
    fn execute_timing_sequence(&self, _ctx: GameState) -> bool {
        true
    }
}
