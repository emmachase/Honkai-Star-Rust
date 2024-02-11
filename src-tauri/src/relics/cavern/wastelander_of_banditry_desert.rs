use crate::{data::Element, relics::{RelicSetKit, RelicSetKitParams}};

pub struct WastelanderOfBanditryDesert2Piece;
pub struct WastelanderOfBanditryDesert4Piece;
pub static WASTELANDER_OF_BANDITRY_DESERT_2P: WastelanderOfBanditryDesert2Piece = WastelanderOfBanditryDesert2Piece;
pub static WASTELANDER_OF_BANDITRY_DESERT_4P: WastelanderOfBanditryDesert4Piece = WastelanderOfBanditryDesert4Piece;

impl RelicSetKit for WastelanderOfBanditryDesert2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        if p.character_element == Element::Imaginary {
            p.boosts.elemental_dmg_boost += 0.10;
        }
    }
}

impl RelicSetKit for WastelanderOfBanditryDesert4Piece {
    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        if p.enemy_config.debuff_count > 0 {
            p.boosts.crit_rate += 0.10;
        }

        if p.conditionals.wastelander_of_banditry_desert_4p {
            p.boosts.crit_rate += 0.10;
            p.boosts.crit_dmg += 0.20;
        }
    }
}
