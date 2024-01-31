use crate::{characters::StatColumnType, relics::{RelicSetKit, RelicSetKitParams}};

pub struct MusketeerOfWildWheat2Piece;
pub struct MusketeerOfWildWheat4Piece;
pub static MUSKETEER_OF_WILD_WHEAT_2P: MusketeerOfWildWheat2Piece = MusketeerOfWildWheat2Piece;
pub static MUSKETEER_OF_WILD_WHEAT_4P: MusketeerOfWildWheat4Piece = MusketeerOfWildWheat4Piece;

impl RelicSetKit for MusketeerOfWildWheat2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.atk_pct += 0.12;
    }
}

impl RelicSetKit for MusketeerOfWildWheat4Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.spd_flat += 0.10;
    }

    fn apply_stat_type_conditionals(&self, p: RelicSetKitParams, stat_type: StatColumnType) {
        if stat_type == StatColumnType::BasicDamage {
            p.boosts.all_type_dmg_boost += 0.10;
        }
    }
}
