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
#[serde(untagged)]
enum ScratchBlockEntry {
    Block(ScratchBlock),
    Reporter(ScratchReporter),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ScratchReporter {
    Primitive(u8, String),
    Reference(u8, String, String),
    TopLevel(u8, String, String, String, String),
}

#[derive(Deserialize)]
struct ScratchProjectMetadata {
    semver: String,
    vm: String,
    agent: String,
    platform: Option<ScratchPlatformMetadata>, // Turbowarp Compat.
}

#[derive(Deserialize)]
struct ScratchPlatformMetadata {
    name: String,
    url: String,
}

#[derive(Deserialize)]
struct ScratchBlock {
    opcode: String,
    next: Option<String>,
    parent: Option<String>,
    inputs: HashMap<String, Vec<Value>>, // i hate scratch so much
    fields: HashMap<String, Vec<Value>>, // this is why scratch sucks
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

#[derive(Deserialize)]
struct ScratchComment {
    #[serde(rename = "blockId")]
    block_id: Option<String>,
    x: Option<isize>,
    y: Option<isize>,
    width: isize,
    height: isize,
    minimized: bool,
    text: String,
}

#[derive(Deserialize)]
struct ScratchCostume {
    name: String,
    #[serde(rename = "bitmapResolution")]
    bitmap_resolution: i32,
    #[serde(rename = "dataFormat")]
    data_format: String,
    #[serde(rename = "assetId")]
    asset_id: String,
    md5ext: String,
    #[serde(rename = "rotationCenterX")]
    rotation_center_x: isize,
    #[serde(rename = "rotationCenterY")]
    rotation_center_y: isize,
}

#[derive(Deserialize)]
struct ScratchSound {
    name: String,
    #[serde(rename = "assetId")]
    asset_id: String,
    #[serde(rename = "dataFormat")]
    data_format: String,
    format: Option<String>,
    rate: isize,
    #[serde(rename = "sampleCount")]
    sample_count: isize,
    md5ext: String,
}

#[derive(Deserialize)]
struct ScratchTarget {
    x: Option<isize>,
    y: Option<isize>,
    size: Option<isize>,
    direction: Option<isize>,
    visible: Option<bool>,
    draggable: Option<bool>,
    rotation_style: Option<String>,
    is_stage: Option<bool>,
    name: String,
    current_costume: isize,
    volume: i32,
    layer_order: isize,
    tempo: Option<isize>,
    video_transparency: Option<isize>,
    text_to_speech_language: Option<String>,
    video_state: Option<String>,
    variables: HashMap<String, ScratchVariable>,
    lists: HashMap<String, ScratchList>,
    broadcasts: HashMap<String, String>,
    blocks: HashMap<String, ScratchBlockEntry>,
    comments: HashMap<String, ScratchComment>,
    costumes: Vec<ScratchCostume>,
    sounds: Vec<ScratchSound>,
}
