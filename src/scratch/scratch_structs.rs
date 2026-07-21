use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum ScratchValue {
    Boolean(bool),
    Number(f64),
    Text(String),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum ScratchMonitorValue {
    Item(ScratchValue),
    List(Vec<ScratchValue>),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum ScratchBlockEntry {
    Block(ScratchBlock),
    Reporter(ScratchReporter),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum ScratchReporter {
    Primitive(u8, String),
    Reference(u8, String, String),
    TopLevel(u8, String, String, f64, f64),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ScratchProjectMetadata {
    pub semver: String,
    pub vm: String,
    pub agent: String,
    pub platform: Option<ScratchPlatformMetadata>, // Turbowarp Compat.
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ScratchPlatformMetadata {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ScratchBlock {
    pub opcode: String,
    pub next: Option<String>,
    pub parent: Option<String>,
    pub inputs: HashMap<String, Vec<Value>>, // i hate scratch so much
    pub fields: HashMap<String, Vec<Value>>, // this is why scratch sucks
    pub shadow: bool,
    #[serde(rename = "topLevel")]
    pub top_level: bool,
    pub x: Option<f32>,
    pub y: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ScratchVariable(pub String, pub ScratchValue);

#[derive(Deserialize, Serialize, Debug)]
pub struct ScratchList(pub String, pub Vec<ScratchValue>);

#[derive(Deserialize, Serialize, Debug)]
pub struct ScratchComment {
    #[serde(rename = "blockId")]
    pub block_id: Option<String>,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub width: f32,
    pub height: f32,
    pub minimized: bool,
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ScratchCostume {
    pub name: String,
    #[serde(rename = "bitmapResolution")]
    pub bitmap_resolution: Option<u32>,
    #[serde(rename = "dataFormat")]
    pub data_format: String,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    pub md5ext: String,
    #[serde(rename = "rotationCenterX")]
    pub rotation_center_x: f32,
    #[serde(rename = "rotationCenterY")]
    pub rotation_center_y: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ScratchSound {
    pub name: String,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[serde(rename = "dataFormat")]
    pub data_format: String,
    pub format: Option<String>,
    pub rate: u32,
    #[serde(rename = "sampleCount")]
    pub sample_count: u32,
    pub md5ext: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ScratchTarget {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub size: Option<f32>,
    pub direction: Option<f32>,
    pub visible: Option<bool>,
    pub draggable: Option<bool>,
    #[serde(rename = "rotationStyle")]
    pub rotation_style: Option<String>,
    #[serde(rename = "isStage")]
    pub is_stage: Option<bool>,
    pub name: String,
    #[serde(rename = "currentCostume")]
    pub current_costume: u32,
    pub volume: u32,
    #[serde(rename = "layerOrder")]
    pub layer_order: u32,
    pub tempo: Option<u32>,
    #[serde(rename = "videoTransparency")]
    pub video_transparency: Option<u32>,
    #[serde(rename = "textToSpeechLanguage")]
    pub text_to_speech_language: Option<String>,
    #[serde(rename = "videoState")]
    pub video_state: Option<String>,
    pub variables: HashMap<String, ScratchVariable>,
    pub lists: HashMap<String, ScratchList>,
    pub broadcasts: HashMap<String, String>,
    pub blocks: HashMap<String, ScratchBlockEntry>,
    pub comments: HashMap<String, ScratchComment>,
    pub costumes: Vec<ScratchCostume>,
    pub sounds: Vec<ScratchSound>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ScratchMonitor {
    pub id: String,
    pub mode: String,
    pub opcode: String,
    pub params: HashMap<String, String>,
    #[serde(rename = "spriteName")]
    pub sprite_name: Option<String>,
    pub value: ScratchMonitorValue,
    pub width: u32,
    pub height: u32,
    pub visible: bool,
    #[serde(rename = "sliderMin")]
    pub slider_min: Option<f32>,
    #[serde(rename = "sliderMax")]
    pub slider_max: Option<f32>,
    #[serde(rename = "isDiscrete")]
    pub is_discrete: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ScratchProject {
    pub targets: Vec<ScratchTarget>,
    pub monitors: Vec<ScratchMonitor>,
    pub extensions: Vec<String>,
    #[serde(rename = "extensionURLs")]
    pub extension_urls: Option<HashMap<String, String>>,
    pub meta: ScratchProjectMetadata,
}
