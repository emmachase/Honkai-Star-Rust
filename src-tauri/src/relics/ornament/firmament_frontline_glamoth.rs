use crate::{RelicSetKit, RelicSetKitParams};

pub struct FirmamentFrontlineGlamoth;
pub static FIRMAMENT_FRONTLINE_GLAMOTH: FirmamentFrontlineGlamoth = FirmamentFrontlineGlamoth;

impl RelicSetKit for FirmamentFrontlineGlamoth {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        p.boosts.atk_pct += 0.12;
    }

    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        let spd = p.character_stats.spd(p.boosts);

        if spd >= 160.0 {
            p.boosts.all_type_dmg_boost += 0.18;
        } else if spd >= 135.0 {
            p.boosts.all_type_dmg_boost += 0.12;
        }
    }
}
