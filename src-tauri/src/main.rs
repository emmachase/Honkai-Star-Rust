// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// #![allow(dead_code)] // TODO: remove

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::sync::RwLock;
use std::{thread, sync::Arc};

use characters::{CharacterKit, CharacterConfig, StatColumnType, StatColumnDesc};
use damage::CharacterStats;
use data::{CharacterDescriptor, RelicSlot, EffectPropertyType};
use data_mappings::RelicSet;
use lightcones::LightConeConfig;
use relics::{Relic, RelicSetKit, ConditionalRelicSetEffects, RelicSetKitParams};
use scans::{KelZRelic, KelZScan};
use serde::{Serialize, Deserialize};
use specta::Type;
use tauri::State;

use crate::relics::Permute;
use crate::scans::TEST_SCAN;
use crate::{data::use_character, damage::{Boosts, EnemyConfig}, data_mappings::LightCone, promotions::{CharacterState, calculate_character_base_stats, LightConeState}, characters::apply_minor_trace_effects, lightcones::LightConeKit};

#[path = "data.gen.rs"]
mod data_mappings;
mod scans;

mod damage;
mod data;
mod util;
mod promotions;
mod characters;
mod lightcones;
mod relics;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(async)]
#[specta::specta]
fn prank_him_john(
    // storage: State<Storage>,
    relics: Vec<Relic>,
    character_cfg: CharacterConfig,
    character_state: CharacterState,
    light_cone_cfg: LightConeConfig,
    light_cone_state: LightConeState,
    enemy_config: EnemyConfig
) -> SortResultsSerde { // Vec<ResolvedCalculatorResult> {
    let character_id = character_cfg.get_character_id();
    let character = use_character(character_id);

    let light_cone_id = light_cone_cfg.get_light_cone_id();

    let character_stats = calculate_character_base_stats((character_id, character_state), Some((light_cone_id, light_cone_state)));

    let kit = character_cfg.get_kit();
    let lc_kit = light_cone_cfg.get_kit();

    // let all_relics = storage.relics.read().unwrap();

    let relics_by_slot = vec![
        relics.clone().into_iter().filter(|r| r.slot == RelicSlot::Head).collect::<Vec<_>>(),
        relics.clone().into_iter().filter(|r| r.slot == RelicSlot::Hands).collect::<Vec<_>>(),
        relics.clone().into_iter().filter(|r| r.slot == RelicSlot::Chest).collect::<Vec<_>>(),
        relics.clone().into_iter().filter(|r| r.slot == RelicSlot::Feet).collect::<Vec<_>>(),
        relics.clone().into_iter().filter(|r| r.slot == RelicSlot::PlanarSphere).collect::<Vec<_>>(),
        relics.clone().into_iter().filter(|r| r.slot == RelicSlot::LinkRope).collect::<Vec<_>>(),
    ];

    // println!("Relics: {:?}", relics_by_slot);

    let time = std::time::Instant::now();

    let cols = calculate_cols(
        CalculatorParameters {
            character: character.clone(),
            character_kit: Arc::from(kit),
            character_state,
            character_stats,
            light_cone: light_cone_id,
            light_cone_kit: Arc::from(lc_kit),
            light_cone_state,
            enemy_config,
            relic_conditionals: ConditionalRelicSetEffects::default(),
        },
        relics_by_slot.clone()
    );

    let duration = time.elapsed();

    println!("Checked {} perms in {}s", relics_by_slot.permutations().size(), duration.as_secs_f64());

    // cols.into_iter().map(|result| {
    //     let resolved_relics = result.relic_perm.into_iter().enumerate().map(|(i, j)| relics_by_slot[i][j].clone()).collect::<Vec<_>>();
    //     ResolvedCalculatorResult {
    //         relic_perm: resolved_relics,
    //         cols: result.cols,
    //         calculated_stats: result.calculated_stats
    //     }
    // }).collect()
    SortResultsSerde::from((cols, &relics_by_slot))
}

#[tauri::command(async)]
#[specta::specta]
fn get_filtered_relic_count(
    storage: State<Storage>,
    filters: RelicFilters
) {

}

#[tauri::command(async)]
#[specta::specta]
fn parse_kelz(
    scan: String
) -> Result<Vec<Relic>, String> {
    let scan: KelZScan = serde_json::from_str(scan.as_str()).map_err(|e| format!("{}", e))?;
    Ok(scan.relics.into_iter().filter_map(|r| r.to_relic()).collect::<Vec<_>>())
}

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
struct RelicFilters {
    chest        : Vec<EffectPropertyType>,
    feet         : Vec<EffectPropertyType>,
    planar_sphere: Vec<EffectPropertyType>,
    link_rope    : Vec<EffectPropertyType>,
}

#[derive(Clone)]
struct CalculatorParameters {
    character: CharacterDescriptor,
    character_kit: Arc<dyn CharacterKit+Sync+Send>,
    character_state: CharacterState,
    character_stats: CharacterStats,
    light_cone: LightCone,
    light_cone_kit: Arc<dyn LightConeKit+Sync+Send>,
    light_cone_state: LightConeState,
    enemy_config: EnemyConfig,
    relic_conditionals: ConditionalRelicSetEffects
}

#[derive(Debug, PartialEq, Clone)]
struct CalculatorResult {
    relic_perm: Vec<usize>, // Relic id per slot
    cols: Vec<(String, f64)>,
    calculated_stats: (CharacterStats, CharacterStats), // (Base stats, Combat stats)
    comparable: f64
}

impl Comparable for CalculatorResult {
    fn get_comparable(&self) -> f64 {
        self.comparable
    }
}

#[derive(Debug, PartialEq)]
struct PreCalculatorResult<'a> {
    cols: &'a Vec<(StatColumnType, f64)>,
    calculated_stats: &'a (CharacterStats, CharacterStats) // (Base stats, Combat stats)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
struct ResolvedCalculatorResult {
    relic_perm: Vec<Relic>,
    cols: Vec<(String, f64)>,
    calculated_stats: (CharacterStats, CharacterStats) // (Base stats, Combat stats)
}

impl From<(CalculatorResult, &Vec<Vec<Relic>>)> for ResolvedCalculatorResult {
    fn from((result, relics_by_slot): (CalculatorResult, &Vec<Vec<Relic>>)) -> Self {
        let resolved_relics = result.relic_perm.into_iter().enumerate().map(|(i, j)| relics_by_slot[i][j].clone()).collect::<Vec<_>>();
        Self {
            relic_perm: resolved_relics,
            cols: result.cols,
            calculated_stats: result.calculated_stats
        }
    }
}

#[derive(Debug, Default)]
pub struct SortResults {
    // pub level: Level,
    // pub ascension: Ascension,
    // pub element: Element,

    pub hp: BinaryHeap<Reverse<CalculatorResult>>,
    pub atk: BinaryHeap<Reverse<CalculatorResult>>,
    pub def: BinaryHeap<Reverse<CalculatorResult>>,
    pub spd: BinaryHeap<Reverse<CalculatorResult>>,
    pub effect_res: BinaryHeap<Reverse<CalculatorResult>>,
    pub crit_rate: BinaryHeap<Reverse<CalculatorResult>>,
    pub crit_dmg: BinaryHeap<Reverse<CalculatorResult>>,
    pub break_effect: BinaryHeap<Reverse<CalculatorResult>>,
    pub energy_recharge: BinaryHeap<Reverse<CalculatorResult>>,
    pub outgoing_healing_boost: BinaryHeap<Reverse<CalculatorResult>>,
    pub elemental_dmg_bonus: BinaryHeap<Reverse<CalculatorResult>>,
    pub effect_hit_rate: BinaryHeap<Reverse<CalculatorResult>>,

    pub cols: Vec<(StatColumnType, BinaryHeap<Reverse<CalculatorResult>>)>,
}

trait Comparable {
    fn get_comparable(&self) -> f64;
}

trait AddToHeap<H, I, P> {
    // fn add_to_heap_i(&mut self, top_k: usize, item: &I, relic_perm: P, maker: fn(P, &I) -> H, i: usize, comparator: fn(i:usize, &H, &I) -> Ordering);
    // fn add_to_heap(&mut self, top_k: usize, item: &I, relic_perm: P, maker: fn(P, &I) -> H, comparator: fn(&H, &I) -> Ordering);

    // fn add_to_heap_i(&mut self, top_k: usize, item: &I, relic_perm: P, maker: fn(P, &I, f64) -> H, i: usize, comparable: f64);
    fn add_to_heap(&mut self, top_k: usize, item: &I, relic_perm: P, maker: fn(P, &I, f64) -> H, comparable: f64);
}

fn eval_presult(relic_perm: &[(&Relic, usize); 6], item: &PreCalculatorResult, comparable: f64) -> CalculatorResult {
    let relic_perm = relic_perm.into_iter().map(|(_, i)| *i).collect::<Vec<usize>>();
    let cols = item.cols.clone().into_iter().map(|(column_type, value)| (column_type.to_name().to_owned(), value)).collect::<Vec<_>>();
    let calculated_stats = item.calculated_stats.clone(); // (params.character_stats + base_boosts, params.character_stats + total_combat_boosts)

    CalculatorResult {
        relic_perm,
        cols,
        calculated_stats,
        comparable
    }
}

impl<H: PartialEq+Ord+Comparable, I: PartialEq, P> AddToHeap<H, I, P> for BinaryHeap<Reverse<H>> {
    // fn add_to_heap_i(&mut self, top_k: usize, item: &I, relic_perm: P, maker: fn(P, &I, f64) -> H, i: usize, comparable: f64) {
    //     let cur_min = self.peek();
    //     // if cur_min.is_none() || comparator(i, &cur_min.unwrap().0, &item) == Ordering::Less {
    //     // let comparable = get_comparable(i, item);
    //     if self.len() < top_k || comparable < cur_min.unwrap().0.get_comparable() {
    //     // comparator(i, &cur_min.unwrap().0, &item) == Ordering::Less {
    //     // || match cur_min {
    //     //     None => true,
    //     //     Some(cur_min) => match comparator(i, &cur_min.0, item) {
    //     //         Ordering::Less => true,
    //     //         Ordering::Equal => self.len() < top_k,
    //     //         _ => false
    //     //     }
    //     // } {
    //         let result = maker(relic_perm, item, comparable);
    //         // H {
    //         //     relic_perm: relic_perm.into_iter().map(|(_, i)| *i).collect::<Vec<usize>>(),
    //         //     cols: item.cols.clone().into_iter().map(|(column_type, value)| (column_type.to_name().to_owned(), value)).collect::<Vec<_>>(),
    //         //     calculated_stats: item.calculated_stats.clone() // (params.character_stats + base_boosts, params.character_stats + total_combat_boosts)
    //         // };

    //     //cur_min.unwrap().0.get_comparable() < item.cols[0].1 { // result.get_comparable() {
    //         if self.len() >= top_k {
    //             self.pop(); // Remove the smallest element
    //         }

    //         self.push(Reverse(result));
    //     }
    // }

    fn add_to_heap(&mut self, top_k: usize, item: &I, relic_perm: P, maker: fn(P, &I, f64) -> H, comparable: f64) { // maker: fn(P, &I, f64) -> H, comparator: fn(&H, &I) -> Ordering) {
        let cur_min = self.peek();
        // if match cur_min {
        //     None => true,
        //     Some(cur_min) => match comparator(&cur_min.0, item) {
        //         Ordering::Less => true,
        //         Ordering::Equal => self.len() < top_k,
        //         _ => false
        //     }
        // } {
        if self.len() < top_k || cur_min.unwrap().0.get_comparable() < comparable {
        //|| comparator(&cur_min.unwrap().0, &item) == Ordering::Less {
            let result = maker(relic_perm, item, comparable);
            // let result = H {
            //     relic_perm: relic_perm.into_iter().map(|(_, i)| *i).collect::<Vec<usize>>(),
            //     cols: item.cols.clone().into_iter().map(|(column_type, value)| (column_type.to_name().to_owned(), value)).collect::<Vec<_>>(),
            //     calculated_stats: item.calculated_stats.clone() // (params.character_stats + base_boosts, params.character_stats + total_combat_boosts)
            // };

        //cur_min.unwrap().0.get_comparable() < item.cols[0].1 { // result.get_comparable() {
            if self.len() >= top_k {
                self.pop(); // Remove the smallest element
            }

            self.push(Reverse(result));
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct SortResultsSerde {
    // pub level: Level,
    // pub ascension: Ascension,
    // pub element: Element,

    pub hp: Vec<ResolvedCalculatorResult>,
    pub atk: Vec<ResolvedCalculatorResult>,
    pub def: Vec<ResolvedCalculatorResult>,
    pub spd: Vec<ResolvedCalculatorResult>,
    pub effect_res: Vec<ResolvedCalculatorResult>,
    pub crit_rate: Vec<ResolvedCalculatorResult>,
    pub crit_dmg: Vec<ResolvedCalculatorResult>,
    pub break_effect: Vec<ResolvedCalculatorResult>,
    pub energy_recharge: Vec<ResolvedCalculatorResult>,
    pub outgoing_healing_boost: Vec<ResolvedCalculatorResult>,
    pub elemental_dmg_bonus: Vec<ResolvedCalculatorResult>,
    pub effect_hit_rate: Vec<ResolvedCalculatorResult>,

    pub cols: Vec<(String, Vec<ResolvedCalculatorResult>)>,
}

impl From<(SortResults, &Vec<Vec<Relic>>)> for SortResultsSerde {
    fn from((sort, relics_by_slot): (SortResults, &Vec<Vec<Relic>>)) -> Self {
        Self {
            hp: sort.hp.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            atk: sort.atk.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            def: sort.def.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            spd: sort.spd.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            effect_res: sort.effect_res.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            crit_rate: sort.crit_rate.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            crit_dmg: sort.crit_dmg.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            break_effect: sort.break_effect.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            energy_recharge: sort.energy_recharge.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            outgoing_healing_boost: sort.outgoing_healing_boost.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            elemental_dmg_bonus: sort.elemental_dmg_bonus.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),
            effect_hit_rate: sort.effect_hit_rate.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect(),

            cols: sort.cols.into_iter().map(|(column_type, heap)| {
                (column_type.to_name().to_owned(), heap.into_sorted_vec().into_iter().map(|Reverse(result)| { ResolvedCalculatorResult::from((result, relics_by_slot)) }).collect())
            }).collect()
        }
    }
}

impl Eq for CalculatorResult {}

impl PartialOrd for CalculatorResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_comparable().partial_cmp(&other.get_comparable()) // TODO
    }
}

impl Ord for CalculatorResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("you done goofed")
    }
}

// impl CalculatorResult {
//     fn get_comparable(&self) -> f64 {
//         self.cols[0].1
//     }
// }

fn calculate_cols(
    params: CalculatorParameters,
    relics: Vec<Vec<Relic>>,
) -> SortResults { // (Vec<Relic>, Vec<(String, f64)>, (CharacterStats, CharacterStats)) {
    let mut base_boosts = Boosts::default();

    apply_minor_trace_effects(&params.character, &params.character_state, &mut base_boosts);
    params.light_cone_kit.apply_base_passives(&params.enemy_config, &params.light_cone_state, &mut base_boosts);
    params.character_kit.apply_base_passives(&params.enemy_config, &params.character_state, &mut base_boosts);

    let mut combat_boosts = Boosts::default();
    params.light_cone_kit.apply_base_combat_passives(&params.enemy_config, &params.light_cone_state, &mut combat_boosts);
    params.character_kit.apply_base_combat_passives(&params.enemy_config, &params.character_state, &mut combat_boosts);

    let thread_count = 16;
    let batches = relics.permutation_batches(thread_count);

    let top_k = 100; // TODO: Configurable

    let mut threads = vec![];
    for tid in 0..thread_count {
        let params = params.clone();
        let relics = relics.clone();
        let kit_cols = params.character_kit.get_stat_columns(&params.enemy_config);
        let batches = batches.clone();
        threads.push(thread::spawn(move || {
            // let mut results: BinaryHeap<Reverse<CalculatorResult>> = BinaryHeap::new(); // Reverse gives us a min-heap
            // results.reserve(top_k);

            let mut all_results = SortResults::default();

            for k in kit_cols.iter() {
                all_results.cols.push((k.column_type, BinaryHeap::new()));
            }


            let mut cols: Vec<(StatColumnType, f64)> = Vec::new();
            cols.reserve(3); // TODO

            for relic_perm in relics.enumerated_permutation_subset(&batches[tid]) {
                let mut base_boosts = base_boosts.clone();

                let mut sets = [0u8; RelicSet::COUNT];
                let mut active_sets = [None, None, None];
                let mut set_index = 0;
                for (&ref relic, _) in relic_perm.iter() {
                    relic.apply(params.character.element, &mut base_boosts);

                    sets[relic.set as usize] += 1;
                    let p = sets[relic.set as usize];

                    if p == 2 {
                        if let Some(effect) = relic.set.get_2p_effect() {
                            active_sets[set_index] = Some(effect);
                            set_index += 1;
                        }
                    } else if p == 4 {
                        if let Some(effect) = relic.set.get_4p_effect() {
                            active_sets[set_index] = Some(effect);
                            set_index += 1;
                        }
                    }
                }

                active_sets.apply_base_passives(RelicSetKitParams {
                    enemy_config: &params.enemy_config,
                    conditionals: &params.relic_conditionals,
                    character_stats: &params.character_stats,
                    character_element: params.character.element,
                    boosts: &mut base_boosts,
                });

                let mut total_combat_boosts = base_boosts + combat_boosts;
                params.light_cone_kit.apply_common_conditionals(&params.enemy_config, &params.light_cone_state, &mut total_combat_boosts);
                params.character_kit.apply_common_conditionals(&params.enemy_config, &params.character_state, &mut total_combat_boosts);
                active_sets.apply_common_conditionals(RelicSetKitParams {
                    enemy_config: &params.enemy_config,
                    conditionals: &params.relic_conditionals,
                    character_stats: &params.character_stats,
                    character_element: params.character.element,
                    boosts: &mut total_combat_boosts,
                });

                cols.clear();
                kit_cols.iter().for_each(|StatColumnDesc { column_type, hit_splits }| {
                    let mut skill_boosts = total_combat_boosts.clone();
                    let column_type = *column_type;

                    params.light_cone_kit.apply_stat_type_conditionals(&params.enemy_config, column_type, &params.light_cone_state, &mut skill_boosts);
                    params.character_kit.apply_stat_type_conditionals(&params.enemy_config, column_type, &params.character_state, &mut skill_boosts);
                    active_sets.apply_stat_type_conditionals(RelicSetKitParams {
                        enemy_config: &params.enemy_config,
                        conditionals: &params.relic_conditionals,
                        character_stats: &params.character_stats,
                        character_element: params.character.element,
                        boosts: &mut skill_boosts,
                    }, column_type);

                    let mut col_total = 0.0;
                    for split in hit_splits.iter().enumerate() {
                        active_sets.apply_inter_hit_effects(split, RelicSetKitParams {
                            enemy_config: &params.enemy_config,
                            conditionals: &params.relic_conditionals,
                            character_stats: &params.character_stats,
                            character_element: params.character.element,
                            boosts: &mut skill_boosts,
                        }, column_type);

                        col_total += params.character_kit.compute_stat_column(column_type, split, &params.character_state, &params.character_stats, &skill_boosts, &params.enemy_config);
                    }

                    cols.push((column_type, col_total));
                    // cols.push((column_type, params.character_kit.compute_stat_column(column_type, (0, &1.0), &params.character_state, &params.character_stats, &skill_boosts, &params.enemy_config)));
                });

                let presult = PreCalculatorResult {
                    cols: &cols,
                    calculated_stats: &(params.character_stats + base_boosts, params.character_stats + total_combat_boosts)
                };

                // let result = || CalculatorResult {
                //     relic_perm: relic_perm.into_iter().map(|(_, i)| i).collect::<Vec<usize>>(),
                //     cols: cols.clone().into_iter().map(|(column_type, value)| (column_type.to_name().to_owned(), value)).collect::<Vec<_>>(),
                //     calculated_stats: (params.character_stats + base_boosts, params.character_stats + total_combat_boosts)
                // };

                // let cur_min = results.peek();
                // if cur_min.is_none() || cur_min.unwrap().0.get_comparable() < cols[0].1 { // result.get_comparable() {
                //     if results.len() >= top_k {
                //         results.pop(); // Remove the smallest element
                //     }

                //     results.push(Reverse(result()));
                // }

                // pub hp: BinaryHeap<Reverse<CalculatorResult>>,
                // pub atk: BinaryHeap<Reverse<CalculatorResult>>,
                // pub def: BinaryHeap<Reverse<CalculatorResult>>,
                // pub spd: BinaryHeap<Reverse<CalculatorResult>>,
                // pub effect_res: BinaryHeap<Reverse<CalculatorResult>>,
                // pub crit_rate: BinaryHeap<Reverse<CalculatorResult>>,
                // pub crit_dmg: BinaryHeap<Reverse<CalculatorResult>>,
                // pub break_effect: BinaryHeap<Reverse<CalculatorResult>>,
                // pub energy_recharge: BinaryHeap<Reverse<CalculatorResult>>,
                // pub outgoing_healing_boost: BinaryHeap<Reverse<CalculatorResult>>,
                // pub elemental_dmg_bonus: BinaryHeap<Reverse<CalculatorResult>>,
                // pub effect_hit_rate: BinaryHeap<Reverse<CalculatorResult>>,

                {
                    // all_results.hp.add_to_heap(top_k, &presult, &relic_perm, eval_presult, |a, b| a.calculated_stats.1.hp.partial_cmp(&b.calculated_stats.1.hp).unwrap());
                    // all_results.atk.add_to_heap(top_k, &presult, &relic_perm, eval_presult, |a, b| a.calculated_stats.1.atk.partial_cmp(&b.calculated_stats.1.atk).unwrap());
                    // all_results.def.add_to_heap(top_k, &presult, &relic_perm, eval_presult, |a, b| a.calculated_stats.1.def.partial_cmp(&b.calculated_stats.1.def).unwrap());
                    // all_results.spd.add_to_heap(top_k, &presult, &relic_perm, eval_presult, |a, b| a.calculated_stats.1.spd.partial_cmp(&b.calculated_stats.1.spd).unwrap());
                    // all_results.effect_res.add_to_heap(top_k, &presult, &relic_perm, eval_presult, |a, b| a.calculated_stats.1.effect_res.partial_cmp(&b.calculated_stats.1.effect_res).unwrap());
                    // all_results.crit_rate.add_to_heap(top_k, &presult, &relic_perm, eval_presult, |a, b| a.calculated_stats.1.crit_rate.partial_cmp(&b.calculated_stats.1.crit_rate).unwrap());
                    // all_results.crit_dmg.add_to_heap(top_k, &presult, &relic_perm, eval_presult, |a, b| a.calculated_stats.1.crit_dmg.partial_cmp(&b.calculated_stats.1.crit_dmg).unwrap());
                    // all_results.break_effect.add_to_heap(top_k, &presult, &relic_perm, eval_presult, |a, b| a.calculated_stats.1.break_effect.partial_cmp(&b.calculated_stats.1.break_effect).unwrap());
                    // all_results.energy_recharge.add_to_heap(top_k, &presult, &relic_perm, eval_presult, |a, b| a.calculated_stats.1.energy_recharge.partial_cmp(&b.calculated_stats.1.energy_recharge).unwrap());
                    // all_results.outgoing_healing_boost.add_to_heap(top_k, &presult, &relic_perm, eval_presult, |a, b| a.calculated_stats.1.outgoing_healing_boost.partial_cmp(&b.calculated_stats.1.outgoing_healing_boost).unwrap());
                    // all_results.elemental_dmg_bonus.add_to_heap(top_k, &presult, &relic_perm, eval_presult, |a, b| a.calculated_stats.1.elemental_dmg_bonus.partial_cmp(&b.calculated_stats.1.elemental_dmg_bonus).unwrap());
                    // all_results.effect_hit_rate.add_to_heap(top_k, &presult, &relic_perm, eval_presult, |a, b| a.calculated_stats.1.effect_hit_rate.partial_cmp(&b.calculated_stats.1.effect_hit_rate).unwrap());

                    // for i in 0..cols.len() {
                    //     all_results.cols[i].1.add_to_heap_i(top_k, &presult, &relic_perm, eval_presult, i, |i, a, b| a.cols[i].1.partial_cmp(&b.cols[i].1).unwrap());
                    // }

                    all_results.hp.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.hp);
                    all_results.atk.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.atk);
                    all_results.def.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.def);
                    all_results.spd.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.spd);
                    all_results.effect_res.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.effect_res);
                    all_results.crit_rate.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.crit_rate);
                    all_results.crit_dmg.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.crit_dmg);
                    all_results.break_effect.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.break_effect);
                    all_results.energy_recharge.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.energy_recharge);
                    all_results.outgoing_healing_boost.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.outgoing_healing_boost);
                    all_results.elemental_dmg_bonus.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.elemental_dmg_bonus);
                    all_results.effect_hit_rate.add_to_heap(top_k, &presult, &relic_perm, eval_presult, presult.calculated_stats.1.effect_hit_rate);

                    for (i, col) in cols.iter().enumerate() {
                        all_results.cols[i].1.add_to_heap(top_k, &presult, &relic_perm, eval_presult, col.1);
                    }
                }
            }

            return all_results;
        }));
    }

    // let mut max: Option<Vec<(String, f64)>> = None;
    // let mut max_relics: Option<Vec<Relic>> = None;
    // let mut max_stats: Option<(CharacterStats, CharacterStats)> = None;

    let mut combined_results = SortResults::default();
    //: BinaryHeap<Reverse<CalculatorResult>> = BinaryHeap::new(); // Reverse gives us a min-heap

    for thread in threads {
        let results = thread.join().unwrap();

        for Reverse(result) in results.hp { combined_results.hp.add_to_heap(top_k, &result, &result.relic_perm, |p, a, c| a.clone(), result.comparable) }
        for Reverse(result) in results.atk { combined_results.atk.add_to_heap(top_k, &result, &result.relic_perm, |p, a, c| a.clone(), result.comparable) }
        for Reverse(result) in results.def { combined_results.def.add_to_heap(top_k, &result, &result.relic_perm, |p, a, c| a.clone(), result.comparable) }
        for Reverse(result) in results.spd { combined_results.spd.add_to_heap(top_k, &result, &result.relic_perm, |p, a, c| a.clone(), result.comparable) }
        for Reverse(result) in results.effect_res { combined_results.effect_res.add_to_heap(top_k, &result, &result.relic_perm, |p, a, c| a.clone(), result.comparable) }
        for Reverse(result) in results.crit_rate { combined_results.crit_rate.add_to_heap(top_k, &result, &result.relic_perm, |p, a, c| a.clone(), result.comparable) }
        for Reverse(result) in results.crit_dmg { combined_results.crit_dmg.add_to_heap(top_k, &result, &result.relic_perm, |p, a, c| a.clone(), result.comparable) }
        for Reverse(result) in results.break_effect { combined_results.break_effect.add_to_heap(top_k, &result, &result.relic_perm, |p, a, c| a.clone(), result.comparable) }
        for Reverse(result) in results.energy_recharge { combined_results.energy_recharge.add_to_heap(top_k, &result, &result.relic_perm, |p, a, c| a.clone(), result.comparable) }
        for Reverse(result) in results.outgoing_healing_boost { combined_results.outgoing_healing_boost.add_to_heap(top_k, &result, &result.relic_perm, |p, a, c| a.clone(), result.comparable) }
        for Reverse(result) in results.elemental_dmg_bonus { combined_results.elemental_dmg_bonus.add_to_heap(top_k, &result, &result.relic_perm, |p, a, c| a.clone(), result.comparable) }
        for Reverse(result) in results.effect_hit_rate { combined_results.effect_hit_rate.add_to_heap(top_k, &result, &result.relic_perm, |p, a, c| a.clone(), result.comparable) }

        // for i in 0..results.cols.len() {
        //     for Reverse(result) in results.cols[i].1 { combined_results.cols[i].1.add_to_heap_i(top_k, &result, &result.relic_perm, |p, a, c| a.clone(), i, |i, a, b| a.cols[i].1.partial_cmp(&b.cols[i].1).unwrap()) }
        // }

        for (i, col) in results.cols.into_iter().enumerate() {
            if combined_results.cols.len() <= i {
                combined_results.cols.push((col.0, BinaryHeap::new()));
            }

            for Reverse(result) in col.1 { combined_results.cols[i].1.add_to_heap(top_k, &result, &result.relic_perm, |p, a, c| a.clone(), result.comparable) }
        }

            // let cur_min = combined_results.peek();
            // if cur_min.is_none() || cur_min.unwrap().0.get_comparable() < result.cols[0].1 { // result.get_comparable() {
            //     // let result = CalculatorResult {
            //     //     relic_perm: relic_perm.into_iter().map(|(_, i)| i).collect::<Vec<usize>>(),
            //     //     cols: cols.clone().into_iter().map(|(column_type, value)| (column_type.to_name().to_owned(), value)).collect::<Vec<_>>(),
            //     //     calculated_stats: (params.character_stats + base_boosts, params.character_stats + total_combat_boosts)
            //     // };

            //     if combined_results.len() >= top_k {
            //         combined_results.pop(); // Remove the smallest element
            //     }

            //     combined_results.push(Reverse(result));
            // }

            // let resolved_relics = relic_perm.into_iter().enumerate().map(|(i, j)| relics[i][j].clone()).collect::<Vec<_>>();

            // if let Some(maxv) = &max {
            //     // for (i, (name, value)) in cols.iter().enumerate() {
            //     if cols[0].1 > maxv[0].1 {
            //         max = Some(cols);
            //         max_relics = Some(resolved_relics);
            //         max_stats = Some(calculated_stats);
            //     }
            // } else {
            //     max = Some(cols);
            //     max_relics = Some(resolved_relics);
            //     max_stats = Some(calculated_stats);
            // }
        // }
    }


    // let mut max_boosts: Option<Boosts> = None;

    // for relic_perm in relics.permutations() {
    //     let mut boosts = boosts.clone();
    //     for &relic in relic_perm.iter() {
    //         relic.apply(params.character.element, &mut boosts);
    //     }

    //     let cols = params.character_kit.get_stat_columns();
    //     let cols: Vec<(String, f64, Boosts)> = cols.iter().map(|&column_type| {
    //         let mut special_boosts = boosts.clone();
    //         params.light_cone_kit.apply_conditional_passives(&params.enemy_config, column_type, &params.light_cone_state, &mut special_boosts);
    //         params.character_kit.apply_conditional_passives(&params.enemy_config, column_type, &params.character_state, &mut special_boosts);

    //         (column_type.to_name().to_owned(), params.character_kit.compute_stat_column(column_type, &params.character_state, &params.character_stats, &special_boosts, &params.enemy_config), special_boosts)
    //     }).collect();


    // }

    // return (max_relics.unwrap(), max.unwrap(), max_stats.unwrap());

    return combined_results // .into_iter().map(|Reverse(result)| result).collect();
}

struct Storage {
    relics: RwLock<Vec<Relic>>
}

fn main() {
    let specta_builder = {
        let specta_builder = tauri_specta::ts::builder()
            .commands(tauri_specta::collect_commands![prank_him_john, get_filtered_relic_count, parse_kelz]);

        #[cfg(debug_assertions)]
        let specta_builder = specta_builder.path("../src/bindings.gen.ts");

        specta_builder.into_plugin()
    };

    let all_relics = TEST_SCAN.relics.iter().filter_map(|r| r.to_relic()).collect::<Vec<_>>();

    tauri::Builder::default()
        .manage(Storage { relics: RwLock::new(all_relics) })
        .plugin(specta_builder)
        .invoke_handler(tauri::generate_handler![prank_him_john, get_filtered_relic_count, parse_kelz])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
