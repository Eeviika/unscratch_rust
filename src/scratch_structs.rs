use serde::Deserialize;

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
