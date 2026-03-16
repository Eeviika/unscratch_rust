use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

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
    platform: Option<PlatformMetadata>, // Turbowarp Compat.
}

#[derive(Deserialize)]
struct PlatformMetadata {
    name: String,
    url: String,
}

#[derive(Deserialize)]
struct ScratchBlock {
    opcode: String,
    next: Option<String>,
    parent: Option<String>,
    inputs: HashMap<String, Vec<Value>>,
    fields: HashMap<String, Vec<Value>>,
    shadow: bool,
    #[serde(rename = "topLevel")]
    top_level: bool,
    x: Option<isize>,
    y: Option<isize>,
}

#[derive(Deserialize)]
struct ScratchVariable(String, ScratchValue);

#[derive(Deserialize)]
struct ScratchList(String, Vec<ScratchValue>);
