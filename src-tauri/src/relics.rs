pub mod cavern;
pub mod ornament;

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{data_mappings::RelicSet, data::{EffectPropertyType, RelicSlot, Element}, damage::{Level, Boosts, EnemyConfig, CharacterStats}, characters::{apply_effect_boost, StatColumnType}};

use self::{
    cavern::{
        band_of_sizzling_thunder::{BAND_OF_SIZZLING_THUNDER_2P, BAND_OF_SIZZLING_THUNDER_4P},
        champion_of_streetwise_boxing::{CHAMPION_OF_STREETWISE_BOXING_2P, CHAMPION_OF_STREETWISE_BOXING_4P},
        eagle_of_twilight_line::EAGLE_OF_TWILIGHT_LINE_2P,
        firesmith_of_lava_forging::{FIRESMITH_OF_LAVA_FORGING_2P, FIRESMITH_OF_LAVA_FORGING_4P},
        genius_of_brilliant_stars::{GENIUS_OF_BRILLIANT_STARS_2P, GENIUS_OF_BRILLIANT_STARS_4P},
        guard_of_wuthering_snow::GUARD_OF_WUTHERING_SNOW_2P,
        hunter_of_glacial_forest::{HUNTER_OF_GLACIAL_FOREST_2P, HUNTER_OF_GLACIAL_FOREST_4P},
        knight_of_purity_palace::{KNIGHT_OF_PURITY_PALACE_2P, KNIGHT_OF_PURITY_PALACE_4P},
        longevous_disciple::{LONGEVOUS_DISCIPLE_2P, LONGEVOUS_DISCIPLE_4P},
        messenger_traversing_hackerspace::{MESSENGER_TRAVERSING_HACKERSPACE_2P, MESSENGER_TRAVERSING_HACKERSPACE_4P},
        musketeer_of_wild_wheat::{MUSKETEER_OF_WILD_WHEAT_2P, MUSKETEER_OF_WILD_WHEAT_4P},
        passerby_of_wandering_cloud::PASSERBY_OF_WANDERING_CLOUD_2P,
        prisoner_in_deep_confinement::{PRISONER_IN_DEEP_CONFINEMENT_2P, PRISONER_IN_DEEP_CONFINEMENT_4P},
        the_ashblazing_grand_duke::{THE_ASHBLAZING_GRAND_DUKE_2P, THE_ASHBLAZING_GRAND_DUKE_4P},
        thief_of_shooting_meteor::{THIEF_OF_SHOOTING_METEOR_2P, THIEF_OF_SHOOTING_METEOR_4P},
        wastelander_of_banditry_desert::{WASTELANDER_OF_BANDITRY_DESERT_2P, WASTELANDER_OF_BANDITRY_DESERT_4P},
        pioneer_diver_of_dead_waters::{PIONEER_DIVER_OF_DEAD_WATERS_2P, PIONEER_DIVER_OF_DEAD_WATERS_4P},
        watchmaker_master_of_dream_machinations::{WATCHMAKER_MASTER_OF_DREAM_MACHINATIONS_2P, WATCHMAKER_MASTER_OF_DREAM_MACHINATIONS_4P},
    },
    ornament::{
        space_sealing_station::SPACE_SEALING_STATION,
        fleet_of_the_ageless::FLEET_OF_THE_AGELESS,
        pan_cosmic_commercial_enterprise::PAN_COSMIC_COMMERCIAL_ENTERPRISE,
        belobog_of_the_architects::BELOBOG_OF_THE_ARCHITECTS,
        celestial_differentiator::CELESTIAL_DIFFERENTIATOR,
        inert_salsotto::INERT_SALSOTTO,
        talia_kingdom_of_banditry::TALIA_KINGDOM_OF_BANDITRY,
        sprightly_vonwacq::SPRIGHTLY_VONWACQ,
        rutilant_arena::RUTILANT_ARENA,
        broken_keel::BROKEN_KEEL,
        firmament_frontline_glamoth::FIRMAMENT_FRONTLINE_GLAMOTH,
        penacony_land_of_the_dreams::PENACONY_LAND_OF_THE_DREAMS,
    }
};


pub type RelicStat = (EffectPropertyType, f64);

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
pub struct Relic {
    pub id: String,

    pub set: RelicSet,
    pub slot: RelicSlot,
    pub level: Level,
    pub main_stat: RelicStat,
    pub sub_stats: Vec<RelicStat>,
}

pub struct RelicIterator<'a> {
    relic: &'a Relic,
    index: usize,
}

impl<'a> Iterator for RelicIterator<'a> {
    type Item = RelicStat;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 0 {
            self.index += 1;
            return Some(self.relic.main_stat);
        } else if self.index <= self.relic.sub_stats.len() {
            let index = self.index - 1;
            self.index += 1;
            return Some(self.relic.sub_stats[index]);
        } else {
            return None;
        }
    }
}

impl Relic {
    pub fn iter(&self) -> RelicIterator {
        return RelicIterator {
            relic: self,
            index: 0,
        };
    }

    pub fn apply(&self, effective_element: Element, boosts: &mut Boosts) {
        for (effect, value) in self.iter() {
            apply_effect_boost(effect, value, boosts);
        }
    }
}

// pub struct Permutations<'a, T> {
//     items: &'a Vec<Vec<T>>,
//     indexes: Vec<usize>,
//     done: bool,
// }

// trait Permute<T> {
//     fn permutations(&self) -> Permutations<T>;
// }

// impl<T> Permute<T> for Vec<Vec<T>> {
//     fn permutations(&self) -> Permutations<T> {
//         Permutations {
//             items: self,
//             indexes: vec![0; self.len()],
//             done: false,
//         }
//     }
// }

// impl<'a, T> Iterator for Permutations<'a, T> {
//     type Item = Vec<&'a T>;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.done {
//             return None;
//         }

//         let mut result: Vec<&'a T> = Vec::new();
//         for (i, item) in self.items.iter().enumerate() {
//             result.push(&item[self.indexes[i]]);
//         }

//         let mut index = 0;
//         loop {
//             if index >= self.indexes.len() {
//                 self.done = true;
//                 break;
//             }

//             self.indexes[index] += 1;
//             if self.indexes[index] >= self.items[index].len() {
//                 self.indexes[index] = 0;
//                 index += 1;
//             } else {
//                 break;
//             }
//         }

//         return Some(result);
//     }
// }

pub struct Permutations<'a, T> {
    items: &'a Vec<Vec<T>>,
    index: usize,
    stop_at: usize,
}

pub struct EnumeratedPermutations<'a, T> {
    items: &'a Vec<Vec<T>>,
    index: usize,
    stop_at: usize,
}

#[derive(Debug, Clone)]
pub struct PermutationSubset {
    pub index: usize,
    pub stop_at: usize,
}

impl<'a, T> Permutations<'a, T> {
    pub fn size(&self) -> usize {
        return self.items.iter().map(|v| v.len()).product();
    }
}

pub trait Permute<T> {
    fn permutations(&self) -> Permutations<T>;
    fn enumerated_permutations(&self) -> EnumeratedPermutations<T>;

    fn permutation_subset(&self, batch: &PermutationSubset) -> Permutations<T>;
    fn enumerated_permutation_subset(&self, batch: &PermutationSubset) -> EnumeratedPermutations<T>;

    fn permutation_batches(&self, batch_count: usize) -> Vec<PermutationSubset>;
}

impl<T> Permute<T> for Vec<Vec<T>> {
    fn permutations(&self) -> Permutations<T> {
        Permutations {
            items: self,
            index: 0,
            stop_at: self.iter().map(|v| v.len()).product(),
        }
    }

    fn permutation_subset(&self, batch: &PermutationSubset) -> Permutations<T> {
        Permutations {
            items: self,
            index: batch.index,
            stop_at: batch.stop_at,
        }
    }

    fn enumerated_permutations(&self) -> EnumeratedPermutations<T> {
        EnumeratedPermutations {
            items: self,
            index: 0,
            stop_at: self.iter().map(|v| v.len()).product(),
        }
    }

    fn enumerated_permutation_subset(&self, batch: &PermutationSubset) -> EnumeratedPermutations<T> {
        EnumeratedPermutations {
            items: self,
            index: batch.index,
            stop_at: batch.stop_at,
        }
    }

    fn permutation_batches(&self, batch_count: usize) -> Vec<PermutationSubset> {
        let mut batches: Vec<PermutationSubset> = Vec::new();
        let size: usize = self.iter().map(|v| v.len()).product();
        let mut batch_size = size / batch_count;
        if size % batch_count != 0 {
            batch_size += 1;
        }

        for i in 0..batch_count {
            batches.push(PermutationSubset {
                index: i * batch_size,
                stop_at: ((i + 1) * batch_size).min(size),
            });
        }

        return batches;
    }
}

impl<'a, T> Iterator for Permutations<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.stop_at {
            return None;
        }

        let mut result = Vec::new();
        let mut index = self.index;
        for item in self.items.iter() {
            result.push(&item[index % item.len()]);
            index /= item.len();
        }

        self.index += 1;

        return Some(result);
    }
}

impl<'a, T> Iterator for EnumeratedPermutations<'a, T> {
    type Item = [(&'a T, usize); 6]; // Vec<(&'a T, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.stop_at {
            return None;
        }

        let xd = &self.items[0][0]; // Hack to just initialize the slice with junk before we fill it
        let mut result: [(&T, usize); 6] = [
            (xd, 0),
            (xd, 0),
            (xd, 0),
            (xd, 0),
            (xd, 0),
            (xd, 0),
        ];
        let mut index = self.index;
        for (i, item) in self.items.iter().enumerate() {
            let subindex = index % item.len();
            result[i] = (&item[subindex], subindex);
            index /= item.len();
        }

        self.index += 1;

        return Some(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations() {
        let items = vec![
            vec![11, 21],
            vec![12, 22],
            vec![13, 23],
        ];

        let permutations: Vec<Vec<&i32>> = items.permutations().collect();
        assert_eq!(permutations, vec![
            vec![&11, &12, &13],
            vec![&21, &12, &13],
            vec![&11, &22, &13],
            vec![&21, &22, &13],
            vec![&11, &12, &23],
            vec![&21, &12, &23],
            vec![&11, &22, &23],
            vec![&21, &22, &23],
        ]);
    }
}

pub struct RelicSetKitParams<'a> {
    pub enemy_config: &'a EnemyConfig,
    pub conditionals: &'a ConditionalRelicSetEffects,
    pub character_stats: &'a CharacterStats,
    pub character_element: Element,
    pub boosts: &'a mut Boosts,
}

macro_rules! clone_params {
    ($params:ident) => {
        RelicSetKitParams {
            enemy_config: $params.enemy_config,
            conditionals: $params.conditionals,
            character_stats: $params.character_stats,
            character_element: $params.character_element,
            boosts: $params.boosts,
        }
    };
}

pub trait RelicSetKit {
    /**
     * This function is called once for each relic permutation.
     * It should apply relic-set passive effects that affect the character's base stats. (i.e. it shows up in the character's stat sheet)
     */
    fn apply_base_passives(&self, _p: RelicSetKitParams) {}

    /**
     * This function is called once for each relic permutation.
     * It should apply relic-set effects that are conditional based on relic stats (e.g. +10% DMG when SPD > 160)
     */
    fn apply_common_conditionals(&self, _p: RelicSetKitParams) {}

    /**
     * This function is called multiple times for each relic permutation.
     * It should apply relic-set effects that are conditional based on the type of stat being calculated (e.g. +10% Ultimate DMG)
     */
    fn apply_stat_type_conditionals(&self, _p: RelicSetKitParams, _stat_type: StatColumnType) {}

    /**
     * This function is called multiple times for each relic permutation, and per hit split.
     * Applies effects that stack per hit (I'm looking at you Ashblazing Duke)
     */
    fn apply_inter_hit_effects(&self, _split: (usize, &f64), _p: RelicSetKitParams, _stat_type: StatColumnType) {}
}

impl RelicSetKit for Vec<Box<dyn RelicSetKit>> {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        for kit in self.iter() {
            kit.apply_base_passives(clone_params!(p));
        }
    }

    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        for kit in self.iter() {
            kit.apply_common_conditionals(clone_params!(p));
        }
    }

    fn apply_stat_type_conditionals(&self, p: RelicSetKitParams, stat_type: StatColumnType) {
        for kit in self.iter() {
            kit.apply_stat_type_conditionals(clone_params!(p), stat_type);
        }
    }

    fn apply_inter_hit_effects(&self, split: (usize, &f64), p: RelicSetKitParams, stat_type: StatColumnType) {
        for kit in self.iter() {
            kit.apply_inter_hit_effects(split, clone_params!(p), stat_type);
        }
    }
}

impl RelicSetKit for [Option<&dyn RelicSetKit>] {
    fn apply_base_passives(&self, p: RelicSetKitParams) {
        for kit in self.iter() {
            if let Some(kit) = kit {
                kit.apply_base_passives(clone_params!(p));
            } else {
                break;
            }
        }
    }

    fn apply_common_conditionals(&self, p: RelicSetKitParams) {
        for kit in self.iter() {
            if let Some(kit) = kit {
                kit.apply_common_conditionals(clone_params!(p));
            } else {
                break;
            }
        }
    }

    fn apply_stat_type_conditionals(&self, p: RelicSetKitParams, stat_type: StatColumnType) {
        for kit in self.iter() {
            if let Some(kit) = kit {
                kit.apply_stat_type_conditionals(clone_params!(p), stat_type);
            } else {
                break;
            }
        }
    }

    fn apply_inter_hit_effects(&self, split: (usize, &f64), p: RelicSetKitParams, stat_type: StatColumnType) {
        for kit in self.iter() {
            if let Some(kit) = kit {
                kit.apply_inter_hit_effects(split, clone_params!(p), stat_type);
            } else {
                break;
            }
        }
    }
}


impl RelicSet {
    pub fn get_2p_effect(&self) -> Option<&dyn RelicSetKit> {
        match self {
            RelicSet::PasserbyOfWanderingCloud => Some(&PASSERBY_OF_WANDERING_CLOUD_2P),
            RelicSet::MusketeerOfWildWheat => Some(&MUSKETEER_OF_WILD_WHEAT_2P),
            RelicSet::KnightOfPurityPalace => Some(&KNIGHT_OF_PURITY_PALACE_2P),
            RelicSet::HunterOfGlacialForest => Some(&HUNTER_OF_GLACIAL_FOREST_2P),
            RelicSet::ChampionOfStreetwiseBoxing => Some(&CHAMPION_OF_STREETWISE_BOXING_2P),
            RelicSet::GuardOfWutheringSnow => Some(&GUARD_OF_WUTHERING_SNOW_2P),
            RelicSet::FiresmithOfLavaForging => Some(&FIRESMITH_OF_LAVA_FORGING_2P),
            RelicSet::GeniusOfBrilliantStars => Some(&GENIUS_OF_BRILLIANT_STARS_2P),
            RelicSet::BandOfSizzlingThunder => Some(&BAND_OF_SIZZLING_THUNDER_2P),
            RelicSet::EagleOfTwilightLine => Some(&EAGLE_OF_TWILIGHT_LINE_2P),
            RelicSet::ThiefOfShootingMeteor => Some(&THIEF_OF_SHOOTING_METEOR_2P),
            RelicSet::WastelanderOfBanditryDesert => Some(&WASTELANDER_OF_BANDITRY_DESERT_2P),
            RelicSet::LongevousDisciple => Some(&LONGEVOUS_DISCIPLE_2P),
            RelicSet::MessengerTraversingHackerspace => Some(&MESSENGER_TRAVERSING_HACKERSPACE_2P),
            RelicSet::TheAshblazingGrandDuke => Some(&THE_ASHBLAZING_GRAND_DUKE_2P),
            RelicSet::PrisonerInDeepConfinement => Some(&PRISONER_IN_DEEP_CONFINEMENT_2P),
            RelicSet::PioneerDiverOfDeadWaters => Some(&PIONEER_DIVER_OF_DEAD_WATERS_2P),
            RelicSet::WatchmakerMasterOfDreamMachinations => Some(&WATCHMAKER_MASTER_OF_DREAM_MACHINATIONS_2P),

            RelicSet::SpaceSealingStation => Some(&SPACE_SEALING_STATION),
            RelicSet::FleetOfTheAgeless => Some(&FLEET_OF_THE_AGELESS),
            RelicSet::PanCosmicCommercialEnterprise => Some(&PAN_COSMIC_COMMERCIAL_ENTERPRISE),
            RelicSet::BelobogOfTheArchitects => Some(&BELOBOG_OF_THE_ARCHITECTS),
            RelicSet::CelestialDifferentiator => Some(&CELESTIAL_DIFFERENTIATOR),
            RelicSet::InertSalsotto => Some(&INERT_SALSOTTO),
            RelicSet::TaliaKingdomOfBanditry => Some(&TALIA_KINGDOM_OF_BANDITRY),
            RelicSet::SprightlyVonwacq => Some(&SPRIGHTLY_VONWACQ),
            RelicSet::RutilantArena => Some(&RUTILANT_ARENA),
            RelicSet::BrokenKeel => Some(&BROKEN_KEEL),
            RelicSet::FirmamentFrontlineGlamoth => Some(&FIRMAMENT_FRONTLINE_GLAMOTH),
            RelicSet::PenaconyLandOfTheDreams => Some(&PENACONY_LAND_OF_THE_DREAMS),
        }
    }

    pub fn get_4p_effect(&self) -> Option<&dyn RelicSetKit> {
        match self {
            RelicSet::MusketeerOfWildWheat => Some(&MUSKETEER_OF_WILD_WHEAT_4P),
            RelicSet::KnightOfPurityPalace => Some(&KNIGHT_OF_PURITY_PALACE_4P),
            RelicSet::HunterOfGlacialForest => Some(&HUNTER_OF_GLACIAL_FOREST_4P),
            RelicSet::ChampionOfStreetwiseBoxing => Some(&CHAMPION_OF_STREETWISE_BOXING_4P),
            RelicSet::FiresmithOfLavaForging => Some(&FIRESMITH_OF_LAVA_FORGING_4P),
            RelicSet::GeniusOfBrilliantStars => Some(&GENIUS_OF_BRILLIANT_STARS_4P),
            RelicSet::BandOfSizzlingThunder => Some(&BAND_OF_SIZZLING_THUNDER_4P),
            RelicSet::ThiefOfShootingMeteor => Some(&THIEF_OF_SHOOTING_METEOR_4P),
            RelicSet::WastelanderOfBanditryDesert => Some(&WASTELANDER_OF_BANDITRY_DESERT_4P),
            RelicSet::LongevousDisciple => Some(&LONGEVOUS_DISCIPLE_4P),
            RelicSet::MessengerTraversingHackerspace => Some(&MESSENGER_TRAVERSING_HACKERSPACE_4P),
            RelicSet::TheAshblazingGrandDuke => Some(&THE_ASHBLAZING_GRAND_DUKE_4P),
            RelicSet::PrisonerInDeepConfinement => Some(&PRISONER_IN_DEEP_CONFINEMENT_4P),
            RelicSet::PioneerDiverOfDeadWaters => Some(&PIONEER_DIVER_OF_DEAD_WATERS_4P),
            RelicSet::WatchmakerMasterOfDreamMachinations => Some(&WATCHMAKER_MASTER_OF_DREAM_MACHINATIONS_4P),

            // No 4p effect on these.
            // Manually list them here so we get a compiler error if we add a new relic set without accounting for it.
            RelicSet::PasserbyOfWanderingCloud      |
            RelicSet::GuardOfWutheringSnow          |
            RelicSet::EagleOfTwilightLine           |
            RelicSet::SpaceSealingStation           |
            RelicSet::FleetOfTheAgeless             |
            RelicSet::PanCosmicCommercialEnterprise |
            RelicSet::BelobogOfTheArchitects        |
            RelicSet::CelestialDifferentiator       |
            RelicSet::InertSalsotto                 |
            RelicSet::TaliaKingdomOfBanditry        |
            RelicSet::SprightlyVonwacq              |
            RelicSet::RutilantArena                 |
            RelicSet::BrokenKeel                    |
            RelicSet::FirmamentFrontlineGlamoth     |
            RelicSet::PenaconyLandOfTheDreams       => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ConditionalRelicSetEffects {
    pub hunter_of_glacial_forest_4p: bool,
    pub champion_of_streetwise_boxing_4p_stacks: u8,
    pub firesmith_of_lava_forging_4p: bool,
    pub genius_of_brilliant_stars_4p: bool,
    pub band_of_sizzling_thunder_4p: bool,
    pub wastelander_of_banditry_desert_4p: bool,
    pub longevous_disciple_4p_stacks: u8,
    pub messenger_traversing_hackerspace_4p: bool,
    pub the_ashblazing_grand_duke_4p_stacks: u8,
    pub prisoner_in_deep_confinement_4p_stacks: u8,
    pub pioneer_diver_of_dead_waters_4p: bool,
    pub watchmaker_master_of_dream_machinations_4p: bool,

    pub celestial_differentiator: bool,
}

impl Default for ConditionalRelicSetEffects {
    fn default() -> Self {
        Self {
            hunter_of_glacial_forest_4p: true,
            champion_of_streetwise_boxing_4p_stacks: 5,
            firesmith_of_lava_forging_4p: true,
            genius_of_brilliant_stars_4p: true,
            band_of_sizzling_thunder_4p: true,
            wastelander_of_banditry_desert_4p: false,
            longevous_disciple_4p_stacks: 2,
            messenger_traversing_hackerspace_4p: false,
            the_ashblazing_grand_duke_4p_stacks: 0,
            prisoner_in_deep_confinement_4p_stacks: 0,
            pioneer_diver_of_dead_waters_4p: true,
            watchmaker_master_of_dream_machinations_4p: false,

            celestial_differentiator: false,
        }
    }
}
