pub mod hunter_of_glacial_forest;
pub mod space_sealing_station;

use crate::{data_mappings::RelicSet, data::{EffectPropertyType, RelicSlot, Element}, damage::{Level, Boosts, EnemyConfig, CharacterStats}, characters::{apply_effect_boost, StatColumnType}, promotions::CharacterState};

use self::{space_sealing_station::SpaceSealingStation2Piece, hunter_of_glacial_forest::{HunterOfGlacialForest2Piece, HunterOfGlacialForest4Piece}};

pub type RelicStat = (EffectPropertyType, f64);

#[derive(Debug, Clone)]
pub struct Relic {
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
            apply_effect_boost(effective_element, effect, value, boosts);
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

impl PermutationSubset {
    pub fn size(&self) -> usize {
        return self.stop_at - self.index;
    }
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
    type Item = Vec<(&'a T, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.stop_at {
            return None;
        }

        let mut result = Vec::new();
        let mut index = self.index;
        for item in self.items.iter() {
            let subindex = index % item.len();
            result.push((&item[subindex], subindex));
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
    fn apply_base_passives(&self, p: RelicSetKitParams);

    // /**
    //  * This function is called once for each relic permutation.
    //  * It should apply relic-set passive effects that affect the character's combat stats. (i.e. it only shows up during combat)
    //  */
    // fn apply_base_combat_passives(&self, enemy_config: &EnemyConfig, boosts: &mut Boosts);

    /**
     * This function is called once for each relic permutation.
     * It should apply relic-set effects that are conditional based on relic stats (e.g. +10% DMG when SPD > 160)
     */
    fn apply_common_conditionals(&self, p: RelicSetKitParams);

    /**
     * This function is called multiple times for each relic permutation.
     * It should apply relic-set effects that are conditional based on the type of stat being calculated (e.g. +10% Ultimate DMG)
     */
    fn apply_stat_type_conditionals(&self, p: RelicSetKitParams, stat_type: StatColumnType);
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
}

impl RelicSetKit for [Option<Box<dyn RelicSetKit>>] {
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
}

impl RelicSet {
    pub fn get_2p_effect(&self) -> Option<Box<dyn RelicSetKit>> {
        match self {
            RelicSet::HunterOfGlacialForest => Some(Box::new(HunterOfGlacialForest2Piece)),

            RelicSet::SpaceSealingStation => Some(Box::new(SpaceSealingStation2Piece)),
            _ => None // TODO: Implement other relic sets
        }
    }

    pub fn get_4p_effect(&self) -> Option<Box<dyn RelicSetKit>> {
        match self {
            RelicSet::HunterOfGlacialForest => Some(Box::new(HunterOfGlacialForest4Piece)),

            _ => None // TODO: Implement other relic sets
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ConditionalRelicSetEffects {
    pub hunter_of_glacial_forest_4p: bool,
}

impl Default for ConditionalRelicSetEffects {
    fn default() -> Self {
        Self { 
            hunter_of_glacial_forest_4p: true
        }
    }
}
