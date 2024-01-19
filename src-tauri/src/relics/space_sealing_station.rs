use crate::characters::StatColumnType;

use super::{RelicSetKit, RelicSetKitParams};

pub struct SpaceSealingStation2Piece;

impl RelicSetKit for SpaceSealingStation2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.atk_pct += 0.12;
    }

    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        if p.character_stats.spd(p.boosts) >= 120.0 {
            p.boosts.atk_pct += 0.12;
        }
    }

    fn apply_stat_type_conditionals(&self, _p: RelicSetKitParams, _stat_type: StatColumnType) {
        // Nothing to do
    }
}
