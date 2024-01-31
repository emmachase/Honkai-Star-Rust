use crate::{RelicSetKit, RelicSetKitParams};

pub struct CelestialDifferentiator;
pub static CELESTIAL_DIFFERENTIATOR: CelestialDifferentiator = CelestialDifferentiator;

impl RelicSetKit for CelestialDifferentiator {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.crit_dmg += 0.16;
    }

    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        if p.conditionals.celestial_differentiator && p.character_stats.crit_dmg(&p.boosts) >= 1.20 {
            p.boosts.crit_rate += 0.6;
        }
    }
}
