use std::{collections::HashMap, fs::File, io::Write, process::Command};

use codegen::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct IdNameDescriptor {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct IdNameMeta {
    id: String,
    name: String,
    raw_name: String,
}

const CHARACTERS_JSON: &str = include_str!("./src/data/characters.json");
const RELIC_SETS_JSON: &str = include_str!("./src/data/relic_sets.json");
const LIGHT_CONES_JSON: &str = include_str!("./src/data/light_cones.json");

fn camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut next_upper = true;
    for c in s.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => if next_upper {
                result.push(c.to_ascii_uppercase());
                next_upper = false;
            } else {
                result.push(c);
            }
            '&' => result.push_str("And"),
            _ => next_upper = true,
        }
    }
    return result;
}

fn gen_enum(scope: &mut Scope, name: &str, json: &str, name_mapper: fn(&IdNameDescriptor) -> Option<String>) {
    let relic_sets: HashMap<String, IdNameDescriptor> = serde_json::from_str(json).unwrap();
    let mut relic_sets = relic_sets.values()
        .map(|d| IdNameMeta { id: d.id.clone(), name: name_mapper(&d).unwrap_or(camel_case(&d.name)), raw_name: d.name.clone() })
        .collect::<Vec<_>>();
    relic_sets.sort_by(|a, b| a.id.cmp(&b.id));


    let relic_enum = scope.new_enum(name).vis("pub");
    relic_enum.derive("Debug").derive("Clone").derive("Copy").derive("PartialEq").derive("Eq").derive("Hash").derive("Serialize").derive("Deserialize").derive("Type");
    for relic_set in &relic_sets {
        relic_enum.new_variant(relic_set.name.clone());
    }

    let enum_impl = scope.new_impl(name);

    {
        // Generate const count
        enum_impl.associate_const("COUNT", "usize", relic_sets.len().to_string(), "pub");

        // Generate to_id() function
        let to_id_fn = enum_impl.new_fn("to_id").vis("pub const").arg_self().ret("&'static str");

        let mut block = Block::new("match self");
        for relic_set in &relic_sets {
            block.line(format!("{}::{} => \"{}\",", name, relic_set.name, relic_set.id));
        }

        to_id_fn.push_block(block);
    }

    {
        // Generate from_name() function
        let from_name_fn = enum_impl.new_fn("from_name").vis("pub").arg("s", "&str").ret("Option<Self>");

        let mut block = Block::new("match s");
        for relic_set in &relic_sets {
            if relic_set.raw_name == "{NICKNAME}" {
                continue;
            }

            block.line(format!("\"{}\" => Some({}::{}),", relic_set.raw_name, name, relic_set.name));
        }
        block.line("_ => None,");

        from_name_fn.push_block(block);
    }
}

fn main() {
    let mut scope = Scope::new();
    scope.import("serde", "{Deserialize, Serialize}");
    scope.import("specta", "Type");

    gen_enum(&mut scope, "RelicSet", RELIC_SETS_JSON, |_| None);
    gen_enum(&mut scope, "LightCone", LIGHT_CONES_JSON, |_| None);
    gen_enum(&mut scope, "Character", CHARACTERS_JSON, |d| match d.id.as_str() {
        "8001" => Some("PhysicalTrailblazerM".to_owned()),
        "8002" => Some("PhysicalTrailblazerF".to_owned()),
        "8003" => Some("FireTrailblazerM".to_owned()),
        "8004" => Some("FireTrailblazerF".to_owned()),
        _ => None
    });

    let code = scope.to_string();
    let code = "// GENERATED CODE - DO NOT EDIT MANUALLY\n#![allow(dead_code)]\n\n".to_owned() + &code;

    let mut file = File::create("src/data.gen.rs").unwrap();
    file.write(code.as_bytes()).unwrap();

    // Format the generated code
    Command::new("rustfmt")
        .arg("src/relics.gen.rs")
        .output()
        .expect("Failed to format relics.gen.rs");

    tauri_build::build()
}
