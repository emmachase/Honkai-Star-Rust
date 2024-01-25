use super::{RelicSetKit, RelicSetKitParams};

pub struct PrisonerInDeepConfinement2Piece;
pub struct PrisonerInDeepConfinement4Piece;
pub static PRISONER_IN_DEEP_CONFINEMENT_2P: PrisonerInDeepConfinement2Piece = PrisonerInDeepConfinement2Piece;
pub static PRISONER_IN_DEEP_CONFINEMENT_4P: PrisonerInDeepConfinement4Piece = PrisonerInDeepConfinement4Piece;

impl RelicSetKit for PrisonerInDeepConfinement2Piece {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.atk_pct += 0.12;
    }
}

impl RelicSetKit for PrisonerInDeepConfinement4Piece {
    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        p.boosts.def_shred += p.conditionals.prisoner_in_deep_confinement_4p_stacks as f64 * 0.06;
    }
}
