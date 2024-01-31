use crate::{RelicSetKit, RelicSetKitParams};

pub struct PenaconyLandOfTheDreams;
pub static PENACONY_LAND_OF_THE_DREAMS: PenaconyLandOfTheDreams = PenaconyLandOfTheDreams;

impl RelicSetKit for PenaconyLandOfTheDreams {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.energy_recharge += 0.05;
    }
}
