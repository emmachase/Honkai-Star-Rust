use crate::{data::Element, relics::{RelicSetKit, RelicSetKitParams}};

pub struct ChampionOfStreetwiseBoxing2Piece;
pub struct ChampionOfStreetwiseBoxing4Piece;
pub static CHAMPION_OF_STREETWISE_BOXING_2P: ChampionOfStreetwiseBoxing2Piece = ChampionOfStreetwiseBoxing2Piece;
pub static CHAMPION_OF_STREETWISE_BOXING_4P: ChampionOfStreetwiseBoxing4Piece = ChampionOfStreetwiseBoxing4Piece;

impl RelicSetKit for ChampionOfStreetwiseBoxing2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        if p.character_element == Element::Physical {
            p.boosts.elemental_dmg_boost += 0.10;
        }
    }
}

impl RelicSetKit for ChampionOfStreetwiseBoxing4Piece {
    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        p.boosts.atk_pct += p.conditionals.champion_of_streetwise_boxing_4p_stacks as f64 * 0.05;
    }
}
