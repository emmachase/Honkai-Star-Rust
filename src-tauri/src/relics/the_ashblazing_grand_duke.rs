use crate::characters::StatColumnType;

use super::{RelicSetKit, RelicSetKitParams};

pub struct TheAshblazingGrandDuke2Piece;
pub struct TheAshblazingGrandDuke4Piece;
pub static THE_ASHBLAZING_GRAND_DUKE_2P: TheAshblazingGrandDuke2Piece = TheAshblazingGrandDuke2Piece;
pub static THE_ASHBLAZING_GRAND_DUKE_4P: TheAshblazingGrandDuke4Piece = TheAshblazingGrandDuke4Piece;

impl RelicSetKit for TheAshblazingGrandDuke2Piece {
    fn apply_stat_type_conditionals(&self, p: RelicSetKitParams, stat_type: StatColumnType) {
        if stat_type == StatColumnType::FollowUpDamage {
            p.boosts.all_type_dmg_boost += 0.20;
        }
    }
}

impl RelicSetKit for TheAshblazingGrandDuke4Piece {
    fn apply_stat_type_conditionals(&self, p: RelicSetKitParams, stat_type: StatColumnType) {
        if stat_type != StatColumnType::FollowUpDamage {
            p.boosts.atk_pct += p.conditionals.the_ashblazing_grand_duke_4p_stacks as f64 * 0.06;
        }
    }

    fn apply_inter_hit_effects(&self, _split: (usize, &f64), p: RelicSetKitParams, stat_type: StatColumnType) {
        if stat_type == StatColumnType::FollowUpDamage {
            p.boosts.atk_pct += 0.06;
        }
    }
}
