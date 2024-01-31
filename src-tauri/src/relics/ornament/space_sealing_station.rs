use crate::{RelicSetKit, RelicSetKitParams};

pub struct SpaceSealingStation;
pub static SPACE_SEALING_STATION: SpaceSealingStation = SpaceSealingStation;

impl RelicSetKit for SpaceSealingStation {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.atk_pct += 0.12;
    }

    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        if p.character_stats.spd(p.boosts) >= 120.0 {
            p.boosts.atk_pct += 0.12;
        }
    }
}
