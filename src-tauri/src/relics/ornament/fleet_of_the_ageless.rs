use crate::{RelicSetKit, RelicSetKitParams};

pub struct FleetOfTheAgeless;
pub static FLEET_OF_THE_AGELESS: FleetOfTheAgeless = FleetOfTheAgeless;

impl RelicSetKit for FleetOfTheAgeless {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.hp_pct += 0.12;
    }

    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        if p.character_stats.spd(p.boosts) >= 120.0 {
            p.boosts.atk_pct += 0.08;
        }
    }
}
