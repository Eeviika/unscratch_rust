use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct Project {
    pub extensions: Vec<String>,
    pub external_extensions: HashMap<String, String>,
    pub metadata: ProjectMetadata,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectMetadata {
    semver: String,
    vm: String,
    platform: PlatformMetadata,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PlatformMetadata {
    name: String,
    url: String,
}

impl Default for PlatformMetadata {
    fn default() -> Self {
        PlatformMetadata {
            name: "Scratch".into(),
            url: "https://scratch.mit.edu".into(),
        }
    }
}
