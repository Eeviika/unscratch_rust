use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct Sprite {
    pub general: GeneralData,
    pub looks: LooksData,
    pub audio: AudioData,
    pub video: VideoData,
    pub variables: HashMap<String, VariableValue>,
    pub costumes: HashMap<String, CostumeData>,
    pub sounds: HashMap<String, SoundData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GeneralData {
    name: String,
    x: f32,
    y: f32,
    size: f32,
    direction: f32,
    is_stage: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LooksData {
    visible: bool,
    draggable: bool,
    rotation_style: RotationStyle,
    current_costume: u32,
    layer: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AudioData {
    volume: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    tts_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tempo: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VideoData {
    #[serde(skip_serializing_if = "Option::is_none")]
    video_transparency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_state: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CostumeData {
    #[serde(skip_serializing_if = "Option::is_none")]
    bitmap_resolution: Option<u32>,
    md5_hash: String,
    asset_id: String,
    asset_filename: String,
    center_x: f32,
    center_y: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SoundData {
    rate: u32,
    sample_count: u32,
    md5_hash: String,
    asset_id: String,
    asset_filename: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum RotationStyle {
    DontRotate,
    LeftRight,
    AllAround,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum VariableValue {
    Boolean(bool),
    Number(f32),
    Text(String),
}
