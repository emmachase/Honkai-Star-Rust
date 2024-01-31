use crate::{data::Element, relics::{RelicSetKit, RelicSetKitParams}};

pub struct LongevousDisciple2Piece;
pub struct LongevousDisciple4Piece;
pub static LONGEVOUS_DISCIPLE_2P: LongevousDisciple2Piece = LongevousDisciple2Piece;
pub static LONGEVOUS_DISCIPLE_4P: LongevousDisciple4Piece = LongevousDisciple4Piece;

impl RelicSetKit for LongevousDisciple2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        if p.character_element == Element::Physical {
            p.boosts.hp_pct += 0.12;
        }
    }
}

impl RelicSetKit for LongevousDisciple4Piece {
    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        p.boosts.crit_rate += p.conditionals.longevous_disciple_4p_stacks as f64 * 0.08;
    }
}
