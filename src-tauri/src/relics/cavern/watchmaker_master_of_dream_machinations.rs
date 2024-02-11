use crate::relics::{RelicSetKit, RelicSetKitParams};

pub struct WatchmakerMasterOfDreamMachinations2Piece;
pub struct WatchmakerMasterOfDreamMachinations4Piece;
pub static WATCHMAKER_MASTER_OF_DREAM_MACHINATIONS_2P: WatchmakerMasterOfDreamMachinations2Piece = WatchmakerMasterOfDreamMachinations2Piece;
pub static WATCHMAKER_MASTER_OF_DREAM_MACHINATIONS_4P: WatchmakerMasterOfDreamMachinations4Piece = WatchmakerMasterOfDreamMachinations4Piece;

impl RelicSetKit for WatchmakerMasterOfDreamMachinations2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.break_effect += 0.16;
    }
}

impl RelicSetKit for WatchmakerMasterOfDreamMachinations4Piece {
    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        if p.conditionals.watchmaker_master_of_dream_machinations_4p {
            p.boosts.break_effect += 0.30;
        }
    }
}
