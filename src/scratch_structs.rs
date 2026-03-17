use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ScratchValue {
    Boolean(bool),
    Number(f64),
    Text(String),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ScratchMonitorValue {
    Item(ScratchValue),
    List(Vec<ScratchValue>),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ScratchBlockEntry {
    Block(ScratchBlock),
    Reporter(ScratchReporter),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ScratchReporter {
    Primitive(u8, String),
    Reference(u8, String, String),
    TopLevel(u8, String, String, f64, f64),
}

#[derive(Deserialize)]
pub struct ScratchProjectMetadata {
    semver: String,
    vm: String,
    agent: String,
    platform: Option<ScratchPlatformMetadata>, // Turbowarp Compat.
}

#[derive(Deserialize)]
pub struct ScratchPlatformMetadata {
    name: String,
    url: String,
}

#[derive(Deserialize)]
pub struct ScratchBlock {
    opcode: String,
    next: Option<String>,
    parent: Option<String>,
    inputs: HashMap<String, Vec<Value>>, // i hate scratch so much
    fields: HashMap<String, Vec<Value>>, // this is why scratch sucks
    shadow: bool,
    #[serde(rename = "topLevel")]
    top_level: bool,
    x: Option<i32>,
    y: Option<i32>,
}

#[derive(Deserialize)]
pub struct ScratchVariable(String, ScratchValue);

#[derive(Deserialize)]
pub struct ScratchList(String, Vec<ScratchValue>);

#[derive(Deserialize)]
pub struct ScratchComment {
    #[serde(rename = "blockId")]
    block_id: Option<String>,
    x: Option<i32>,
    y: Option<i32>,
    width: i32,
    height: i32,
    minimized: bool,
    text: String,
}

#[derive(Deserialize)]
pub struct ScratchCostume {
    name: String,
    #[serde(rename = "bitmapResolution")]
    bitmap_resolution: Option<u32>,
    #[serde(rename = "dataFormat")]
    data_format: String,
    #[serde(rename = "assetId")]
    asset_id: String,
    md5ext: String,
    #[serde(rename = "rotationCenterX")]
    rotation_center_x: i32,
    #[serde(rename = "rotationCenterY")]
    rotation_center_y: i32,
}

#[derive(Deserialize)]
pub struct ScratchSound {
    name: String,
    #[serde(rename = "assetId")]
    asset_id: String,
    #[serde(rename = "dataFormat")]
    data_format: String,
    format: Option<String>,
    rate: u32,
    #[serde(rename = "sampleCount")]
    sample_count: u32,
    md5ext: String,
}

#[derive(Deserialize)]
pub struct ScratchTarget {
    x: Option<i32>,
    y: Option<i32>,
    size: Option<i32>,
    direction: Option<i32>,
    visible: Option<bool>,
    draggable: Option<bool>,
    #[serde(rename = "rotationStyle")]
    rotation_style: Option<String>,
    #[serde(rename = "isStage")]
    is_stage: Option<bool>,
    name: String,
    #[serde(rename = "currentCostume")]
    current_costume: u32,
    volume: i32,
    #[serde(rename = "layerOrder")]
    layer_order: u32,
    tempo: Option<u32>,
    #[serde(rename = "videoTransparency")]
    video_transparency: Option<u32>,
    #[serde(rename = "textToSpeechLanguage")]
    text_to_speech_language: Option<String>,
    #[serde(rename = "videoState")]
    video_state: Option<String>,
    variables: HashMap<String, ScratchVariable>,
    lists: HashMap<String, ScratchList>,
    broadcasts: HashMap<String, String>,
    blocks: HashMap<String, ScratchBlockEntry>,
    comments: HashMap<String, ScratchComment>,
    costumes: Vec<ScratchCostume>,
    sounds: Vec<ScratchSound>,
}

#[derive(Deserialize)]
pub struct ScratchMonitor {
    id: String,
    mode: String,
    opcode: String,
    params: HashMap<String, String>,
    #[serde(rename = "spriteName")]
    sprite_name: Option<String>,
    value: ScratchMonitorValue,
    width: u32,
    height: u32,
    visible: bool,
    #[serde(rename = "sliderMin")]
    slider_min: Option<isize>,
    #[serde(rename = "sliderMax")]
    slider_max: Option<isize>,
    #[serde(rename = "isDiscrete")]
    is_discrete: Option<bool>,
}

#[derive(Deserialize)]
pub struct ScratchProject {
    targets: Vec<ScratchTarget>,
    monitors: Vec<ScratchMonitor>,
    extensions: Vec<String>,
    #[serde(rename = "extensionURLs")]
    extension_urls: Option<HashMap<String, String>>,
    meta: ScratchProjectMetadata,
}
