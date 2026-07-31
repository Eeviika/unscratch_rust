use std::collections::HashMap;

pub struct Project {
    pub extensions: Vec<String>,
    pub extension_urls: HashMap<String, String>,
    pub metadata: ProjectMetadata,
}

pub struct ProjectMetadata {
    semver: String,
    vm: String,
    platform: PlatformMetadata,
}

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
