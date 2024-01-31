use crate::{RelicSetKit, RelicSetKitParams};

pub struct PasserbyOfWanderingCloud2Piece;
pub static PASSERBY_OF_WANDERING_CLOUD_2P: PasserbyOfWanderingCloud2Piece = PasserbyOfWanderingCloud2Piece;

impl RelicSetKit for PasserbyOfWanderingCloud2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.outgoing_healing_boost += 0.10;
    }
}
