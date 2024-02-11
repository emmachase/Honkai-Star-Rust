use crate::relics::{RelicSetKit, RelicSetKitParams};

pub struct PioneerDiverOfDeadWaters2Piece;
pub struct PioneerDiverOfDeadWaters4Piece;
pub static PIONEER_DIVER_OF_DEAD_WATERS_2P: PioneerDiverOfDeadWaters2Piece = PioneerDiverOfDeadWaters2Piece;
pub static PIONEER_DIVER_OF_DEAD_WATERS_4P: PioneerDiverOfDeadWaters4Piece = PioneerDiverOfDeadWaters4Piece;

impl RelicSetKit for PioneerDiverOfDeadWaters2Piece {
    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        if p.enemy_config.debuff_count > 0 {
            p.boosts.all_type_dmg_boost += 0.12;
        }
    }
}

impl RelicSetKit for PioneerDiverOfDeadWaters4Piece {
    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        let multiplier = if p.conditionals.pioneer_diver_of_dead_waters_4p {2.0} else {1.0};

        p.boosts.crit_rate += 0.04*multiplier;

        if p.enemy_config.debuff_count >= 3 {
            p.boosts.crit_dmg += 0.12*multiplier;
        } else if p.enemy_config.debuff_count >= 2 {
            p.boosts.crit_dmg += 0.08*multiplier;
        }
    }
}
