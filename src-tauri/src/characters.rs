pub mod jingliu;
pub mod sparkle;

use serde::{Serialize, Deserialize};
use specta::Type;

use crate::{data::{use_character_trace_node, CharacterDescriptor, EffectPropertyType, Element}, damage::{Boosts, EnemyConfig, CharacterStats}, promotions::CharacterState, data_mappings::Character};

#[derive(Debug, Clone)]
pub struct CharacterTraceIds {
    pub ability_1: String,
    pub ability_2: String,
    pub ability_3: String,
    pub stat_1   : String,
    pub stat_2   : String,
    pub stat_3   : String,
    pub stat_4   : String,
    pub stat_5   : String,
    pub stat_6   : String,
    pub stat_7   : String,
    pub stat_8   : String,
    pub stat_9   : String,
    pub stat_10  : String,
}

impl CharacterTraceIds {
    pub fn from_character(character: &CharacterDescriptor) -> Self {
        let mut trace_ids: Vec<&str> = Vec::new();
        for trace_id in &character.skill_trees {
            let trace = use_character_trace_node(&trace_id);
            if trace.name != "" {
                trace_ids.push(trace_id);
            }
        }

        assert_eq!(trace_ids.len(), 13);

        return Self {
            ability_1: trace_ids[0].to_owned(),
            ability_2: trace_ids[1].to_owned(),
            ability_3: trace_ids[2].to_owned(),
            stat_1   : trace_ids[3].to_owned(),
            stat_2   : trace_ids[4].to_owned(),
            stat_3   : trace_ids[5].to_owned(),
            stat_4   : trace_ids[6].to_owned(),
            stat_5   : trace_ids[7].to_owned(),
            stat_6   : trace_ids[8].to_owned(),
            stat_7   : trace_ids[9].to_owned(),
            stat_8   : trace_ids[10].to_owned(),
            stat_9   : trace_ids[11].to_owned(),
            stat_10  : trace_ids[12].to_owned(),
        };
    }
}

fn apply_std_trace_effect(trace_id: &str, boosts: &mut Boosts) {
    let trace = use_character_trace_node(trace_id);
    for effect in &trace.levels[0].properties {
        apply_effect_boost(effect.property_type, effect.value, boosts);
    }
}

pub fn apply_effect_boost(effect: EffectPropertyType, value: f64, boosts: &mut Boosts) {
    match effect {
        EffectPropertyType::HPDelta                   => boosts.hp_flat += value,
        EffectPropertyType::AttackDelta               => boosts.atk_flat += value,
        EffectPropertyType::DefenceDelta              => boosts.def_flat += value,
        EffectPropertyType::SpeedDelta                => boosts.spd_flat += value,
        EffectPropertyType::HPAddedRatio              => boosts.hp_pct += value,
        EffectPropertyType::AttackAddedRatio          => boosts.atk_pct += value,
        EffectPropertyType::DefenceAddedRatio         => boosts.def_pct += value,
        EffectPropertyType::CriticalChanceBase        => boosts.crit_rate += value,
        EffectPropertyType::CriticalDamageBase        => boosts.crit_dmg += value,
        EffectPropertyType::HealRatioBase             => boosts.outgoing_healing_boost += value,
        EffectPropertyType::StatusProbabilityBase     => boosts.effect_hit_rate += value,
        EffectPropertyType::BreakDamageAddedRatioBase => boosts.break_effect += value,
        EffectPropertyType::SPRatioBase               => boosts.energy_recharge += value,
        EffectPropertyType::StatusResistanceBase      => boosts.effect_res += value,

        EffectPropertyType::AllDamageTypeAddedRatio   => boosts.all_type_dmg_boost += value,
        EffectPropertyType::PhysicalAddedRatio        => boosts.elemental_dmg_boost[Element::Physical]  += value,
        EffectPropertyType::FireAddedRatio            => boosts.elemental_dmg_boost[Element::Fire]      += value,
        EffectPropertyType::IceAddedRatio             => boosts.elemental_dmg_boost[Element::Ice]       += value,
        EffectPropertyType::ThunderAddedRatio         => boosts.elemental_dmg_boost[Element::Thunder]   += value,
        EffectPropertyType::WindAddedRatio            => boosts.elemental_dmg_boost[Element::Wind]      += value,
        EffectPropertyType::QuantumAddedRatio         => boosts.elemental_dmg_boost[Element::Quantum]   += value,
        EffectPropertyType::ImaginaryAddedRatio       => boosts.elemental_dmg_boost[Element::Imaginary] += value,
    }
}

pub fn apply_minor_trace_effects(character: &CharacterDescriptor, character_state: &CharacterState, boosts: &mut Boosts) {
    let trace_ids = CharacterTraceIds::from_character(character);

    // Major traces are applied in the character kit, since they are character-specific
    // But minor traces can be done automatically

    if character_state.traces.stat_1  { apply_std_trace_effect(&trace_ids.stat_1,  boosts); }
    if character_state.traces.stat_2  { apply_std_trace_effect(&trace_ids.stat_2,  boosts); }
    if character_state.traces.stat_3  { apply_std_trace_effect(&trace_ids.stat_3,  boosts); }
    if character_state.traces.stat_4  { apply_std_trace_effect(&trace_ids.stat_4,  boosts); }
    if character_state.traces.stat_5  { apply_std_trace_effect(&trace_ids.stat_5,  boosts); }
    if character_state.traces.stat_6  { apply_std_trace_effect(&trace_ids.stat_6,  boosts); }
    if character_state.traces.stat_7  { apply_std_trace_effect(&trace_ids.stat_7,  boosts); }
    if character_state.traces.stat_8  { apply_std_trace_effect(&trace_ids.stat_8,  boosts); }
    if character_state.traces.stat_9  { apply_std_trace_effect(&trace_ids.stat_9,  boosts); }
    if character_state.traces.stat_10 { apply_std_trace_effect(&trace_ids.stat_10, boosts); }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
pub enum StatColumnType {
    BasicDamage,
    SkillDamage,
    SkillHeal,
    SkillShield,
    UltimateDamage,
    UltimateHeal,
    UltimateShield,
    FollowUpDamage,
}

impl StatColumnType {
    pub fn to_name(&self) -> &'static str {
        match self {
            StatColumnType::BasicDamage => "Basic DMG",
            StatColumnType::SkillDamage => "Skill DMG",
            StatColumnType::SkillHeal => "Skill Heal",
            StatColumnType::SkillShield => "Skill Shield",
            StatColumnType::UltimateDamage => "Ult DMG",
            StatColumnType::UltimateHeal => "Ult Heal",
            StatColumnType::UltimateShield => "Ult Shield",
            StatColumnType::FollowUpDamage => "FUA DMG",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct StatColumnDesc {
    pub column_type: StatColumnType,
    pub hit_splits: Vec<f64>
}

#[macro_export]
macro_rules! col {
    ($column_type:ident: [$($hit_split:expr),*]) => {
        {
            use crate::characters::{StatColumnType, StatColumnDesc};
            StatColumnDesc {
                column_type: StatColumnType::$column_type,
                hit_splits: vec![$($hit_split),*]
            }
        }
    };
}

pub trait CharacterKit {
    /**
     * This function is called once outside of the permutation loop.
     * It should apply character passive effects that affect the character's base stats. (i.e. it shows up in the character's stat sheet)
     */
    fn apply_base_effects(&self, _enemy_config: &EnemyConfig, _character_state: &CharacterState, _boosts: &mut Boosts) {}

    /**
     * This function is called once outside of the permutation loop.
     * It should apply character passive effects that affect the only this character's combat stats. (i.e. it only shows up during combat)
     * If the effect affects the character's teammates, it should be applied in [`CharacterKit::apply_shared_combat_passives()`] instead.
     */
    fn apply_base_combat_effects(&self, _enemy_config: &EnemyConfig, _character_state: &CharacterState, _boosts: &mut Boosts) {}

    /**
     * This function is called once outside of the permutation loop.
     * It should apply character passive effects that affect the this character's and teammates' combat stats. (i.e. it only shows up during combat)
     */
    fn apply_shared_combat_effects(&self, _enemy_config: &EnemyConfig, _own_character_state: &CharacterState, _boosts: &mut Boosts) {}

    /**
     * This function is called once outside of the permutation loop.
     * It should apply character effects that affect the this character's teammates' combat stats.
     * Apply effects here if the effect does not apply to the character itself, or the effect on teammates' is conditional on the character's stats (code duplication sad face).
     */
    fn apply_teammate_combat_effects(&self, _enemy_config: &EnemyConfig, _character_state: &CharacterState, _boosts: &mut Boosts) {}

    /**
     * This function is called once for each relic permutation.
     * It should apply character effects that are conditional based on relic stats (e.g. +10% DMG when SPD > 160)
     * If the effect does not depend on relic stats, it should be applied in [`CharacterKit::apply_base_combat_passives()`] instead.
     */
    fn apply_general_conditionals(&self, _enemy_config: &EnemyConfig, _character_state: &CharacterState, _character_stats: &CharacterStats, _boosts: &mut Boosts) {}

    /**
     * This function is called multiple times for each relic permutation.
     * It should apply character effects that are conditional based on the type of stat being calculated (e.g. +10% Ultimate DMG)
     */
    fn apply_stat_type_conditionals(&self, _enemy_config: &EnemyConfig, _stat_type: StatColumnType, _character_state: &CharacterState, _character_stats: &CharacterStats, _boosts: &mut Boosts) {}

    fn get_stat_columns(&self, enemy_config: &EnemyConfig) -> Vec<StatColumnDesc>;
    // fn get_hit_split(&self, column_type: StatColumnType) -> Vec<f64>;
    fn compute_stat_column(&self, column_type: StatColumnType, split: (usize, &f64), character_state: &CharacterState, character_stats: &CharacterStats, boosts: &Boosts, enemy_config: &EnemyConfig) -> f64;
}

#[macro_export]
macro_rules! wrong_config {
    ($expected:ty) => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let name = name.strip_suffix("::f").unwrap();
        let expected = std::any::type_name::<$expected>();
        panic!("Config Type passed to {} was not {}. This is a bug in the code. Please report this to the developer.", name, expected);
    }}
}

#[macro_export]
macro_rules! shared_configs {
    (
        prefix $prefix:ident;

        Base {
            $( $base_field:ident: $base_type:ty ),* $(,)?
        }

        Teammate {
            $( $teammate_field:ident: $teammate_type:ty ),* $(,)?
        }

        Shared {
            $( $shared_field:ident: $shared_type:ty ),* $(,)?
        }
    ) => {
        paste::paste! {
            #[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize, specta::Type)]
            pub struct [<$prefix BaseConfig>] {
                $( pub $base_field: $base_type, )*
                $( pub $shared_field: $shared_type, )*
            }

            #[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize, specta::Type)]
            pub struct [<$prefix TeammateConfig>] {
                $( pub $teammate_field: $teammate_type, )*
                $( pub $shared_field: $shared_type, )*
            }

            #[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize, specta::Type)]
            pub struct [<$prefix SharedConfig>] {
                $( pub $shared_field: $shared_type ),*
            }

            impl From<[<$prefix Config>]> for [<$prefix SharedConfig>] {
                fn from(value: [<$prefix Config>]) -> Self {
                    match value {
                        [<$prefix Config>]::Own(config) => Self {
                            $( $shared_field: config.$shared_field ),*
                        },
                        [<$prefix Config>]::Teammate(config) => Self {
                            $( $shared_field: config.$shared_field ),*
                        },
                    }
                }
            }

            #[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize, specta::Type)]
            pub enum [<$prefix Config>] {
                Own([<$prefix BaseConfig>]),
                Teammate([<$prefix TeammateConfig>]),
            }
        }
    };
}

//=== Self Reminder: Add new characters down here! :) ===

#[derive(Debug, Type, Serialize, Deserialize)]
pub enum CharacterConfig {
    Jingliu(jingliu::JingliuConfig),
    Sparkle(sparkle::SparkleConfig),
}

#[derive(Debug, Type, Serialize)]
pub enum CharacterDescriptions {
    Jingliu(jingliu::JingliuDescriptions),
    Sparkle(sparkle::SparkleDescriptions),
}

impl CharacterConfig {
    pub fn get_character_id(&self) -> Character {
        match self {
            CharacterConfig::Jingliu(_) => Character::Jingliu,
            CharacterConfig::Sparkle(_) => Character::Sparkle,
        }
    }

    pub fn get_kit(&self) -> Box<dyn CharacterKit+Send+Sync> {
        match self {
            CharacterConfig::Jingliu(config) => Box::new(jingliu::Jingliu::new(*config)),
            CharacterConfig::Sparkle(config) => Box::new(sparkle::Sparkle::new(*config)),
        }
    }
}

impl CharacterDescriptions {
    pub fn get(character: Character) -> Self {
        match character {
            Character::Jingliu => CharacterDescriptions::Jingliu(jingliu::JingliuDescriptions::get()),
            Character::Sparkle => CharacterDescriptions::Sparkle(sparkle::SparkleDescriptions::get()),
            _ => panic!("Character {:?} does not have descriptions", character),
        }
    }
}
