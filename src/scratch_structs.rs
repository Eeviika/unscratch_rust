use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
#[serde(untagged)]
enum ScratchValue {
    Number(f64),
    Boolean(bool),
    Text(String),
}

#[derive(Deserialize)]
struct ProjectMetadata {
    semver: String,
    vm: String,
    agent: String,
    platform: Option<HashMap<String, String>>, // Turbowarp Compat.
}

#[derive(Deserialize)]
struct ScratchBlock {
    opcode: String,
    shadow: bool,
    #[serde(rename = "topLevel")]
    top_level: bool,
    x: Option<isize>,
    y: Option<isize>,
    next: Option<String>,
    parent: Option<String>,
}

#[derive(Deserialize)]
struct ScratchVariable(String, ScratchValue);
