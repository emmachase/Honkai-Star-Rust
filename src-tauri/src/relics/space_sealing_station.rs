use super::{RelicSetKit, RelicSetKitParams};

pub struct SpaceSealingStation2Piece;
pub static SPACE_SEALING_STATION_2P: SpaceSealingStation2Piece = SpaceSealingStation2Piece;

impl RelicSetKit for SpaceSealingStation2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.atk_pct += 0.12;
    }

    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        if p.character_stats.spd(p.boosts) >= 120.0 {
            p.boosts.atk_pct += 0.12;
        }
    }
}
