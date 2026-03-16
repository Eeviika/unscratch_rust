use std::collections::HashMap;

use serde::Deserialize;

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
    x: Option<i32>,
    y: Option<i32>,
    next: Option<String>,
    parent: Option<String>,
}
