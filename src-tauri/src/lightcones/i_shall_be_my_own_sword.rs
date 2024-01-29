use serde::{Deserialize, Serialize};
use serde_tuple::Deserialize_tuple;
use specta::Type;

use crate::{promotions::LightConeState, damage::{Boosts, EnemyConfig}, characters::StatColumnType, data::use_light_cone_effects, data_mappings::LightCone, util::deserialize::deserialize_u8};

use super::LightConeKit;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Type)]
pub struct IShallBeMyOwnSwordConfig {
    pub eclipse_stacks: u8,
    pub max_stack_def_pen: bool,
}

pub struct IShallBeMyOwnSword {
    pub descriptions: Vec<IShallBeMyOwnSwordDesc>,
    pub config: IShallBeMyOwnSwordConfig,
}

impl IShallBeMyOwnSword {
    pub fn new(config: IShallBeMyOwnSwordConfig) -> Self {
        return Self {
            descriptions: IShallBeMyOwnSwordDesc::get(),
            config,
        }
    }
}

/**
 * Increases the wearer's CRIT DMG by #1[i]%. When an ally (excluding the wearer) gets 
 * attacked or loses HP, the wearer gains 1 stack of Eclipse, up to a max of #2[i] stack(s). 
 * Each stack of Eclipse increases the DMG of the wearer's next attack by #3[f1]%. When #2[i] 
 * stack(s) are reached, additionally enables that attack to ignore #4[i]% of the enemy's DEF. 
 * This effect will be removed after the wearer uses an attack.
 */
#[derive(Debug, Clone, Copy, Deserialize_tuple)]
pub struct IShallBeMyOwnSwordDesc {
    pub crit_dmg_pct: f64,
    #[serde(deserialize_with = "deserialize_u8")]
    pub max_stacks: u8,
    pub dmg_per_stack: f64,
    pub def_pen_pct: f64,
}

impl IShallBeMyOwnSwordDesc {
    pub fn get() -> Vec<Self> {
        return use_light_cone_effects(LightCone::IShallBeMyOwnSword).superimpositions();
    }
}

impl LightConeKit for IShallBeMyOwnSword {
    fn apply_base_passives(&self, _enemy_config: &EnemyConfig, light_cone_state: &LightConeState, boosts: &mut Boosts) {
        let desc = self.descriptions[light_cone_state.superimposition as usize];

        boosts.crit_dmg += desc.crit_dmg_pct;
    }

    fn apply_base_combat_passives(&self, _enemy_config: &EnemyConfig, light_cone_state: &LightConeState, boosts: &mut Boosts) {
        let desc = self.descriptions[light_cone_state.superimposition as usize];

        boosts.all_type_dmg_boost += self.config.eclipse_stacks as f64 * desc.dmg_per_stack;
        if self.config.max_stack_def_pen {
            boosts.def_shred += desc.def_pen_pct;
        }
    }

    fn apply_common_conditionals(&self, _enemy_config: &EnemyConfig, _light_cone_state: &LightConeState, _boosts: &mut Boosts) {
        // No conditional passives
    }

    fn apply_stat_type_conditionals(&self, _enemy_config: &EnemyConfig, _stat_type: StatColumnType, _light_cone_state: &LightConeState, _boosts: &mut Boosts) {
        // No stat passives
    }
}
