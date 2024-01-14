use crate::{data_mappings::Character, data::{use_character, use_character_trace_node, CharacterDescriptor}};

#[derive(Debug, Clone, Copy)]
pub struct CharacterTraceIds {
    pub ability_1: &'static str,
    pub ability_2: &'static str,
    pub ability_3: &'static str,
    pub stat_1   : &'static str,
    pub stat_2   : &'static str,
    pub stat_3   : &'static str,
    pub stat_4   : &'static str,
    pub stat_5   : &'static str,
    pub stat_6   : &'static str,
    pub stat_7   : &'static str,
    pub stat_8   : &'static str,
    pub stat_9   : &'static str,
    pub stat_10  : &'static str,
}

impl CharacterTraceIds {
    pub fn from_character(character: &'static CharacterDescriptor) -> Self {
        let mut trace_ids: Vec<&str> = Vec::new();
        for trace_id in &character.skill_trees {
            let trace = use_character_trace_node(&trace_id);
            if trace.name != "" {
                trace_ids.push(trace_id);
            }
        }

        assert_eq!(trace_ids.len(), 13);

        return Self {
            ability_1: trace_ids[0],
            ability_2: trace_ids[1],
            ability_3: trace_ids[2],
            stat_1   : trace_ids[3],
            stat_2   : trace_ids[4],
            stat_3   : trace_ids[5],
            stat_4   : trace_ids[6],
            stat_5   : trace_ids[7],
            stat_6   : trace_ids[8],
            stat_7   : trace_ids[9],
            stat_8   : trace_ids[10],
            stat_9   : trace_ids[11],
            stat_10  : trace_ids[12],
        };
    }
}
