pub mod i_shall_be_my_own_sword;

use crate::{promotions::LightConeState, damage::{Boosts, EnemyConfig}, characters::StatColumnType};

pub trait LightConeKit {
    fn apply_static_passives(&self, enemy_config: &EnemyConfig, light_cone_state: &LightConeState, boosts: &mut Boosts);
    fn apply_conditional_passives(&self, enemy_config: &EnemyConfig, stat_type: StatColumnType, light_cone_state: &LightConeState, boosts: &mut Boosts);
}
