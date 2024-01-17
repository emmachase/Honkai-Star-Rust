use crate::{data_mappings::RelicSet, data::{EffectPropertyType, RelicSlot, Element}, damage::{Level, Boosts}, characters::apply_effect_boost};

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
    fn permutation_subset(&self, batch: &PermutationSubset) -> Permutations<T>;
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

        let mut result: Vec<&T> = Vec::new();
        let mut index = self.index;
        for item in self.items.iter() {
            result.push(&item[index % item.len()]);
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