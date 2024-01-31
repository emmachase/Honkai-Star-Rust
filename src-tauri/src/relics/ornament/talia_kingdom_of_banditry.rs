use crate::{RelicSetKit, RelicSetKitParams};

pub struct TaliaKingdomOfBanditry;
pub static TALIA_KINGDOM_OF_BANDITRY: TaliaKingdomOfBanditry = TaliaKingdomOfBanditry;

impl RelicSetKit for TaliaKingdomOfBanditry {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.break_effect += 0.16;
    }

    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        if p.character_stats.spd(p.boosts) >= 145.0 {
            p.boosts.break_effect += 0.20;
        }
    }
}
