use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Sprite {
    pub general: GeneralData,
    pub looks: LooksData,
    pub audio: AudioData,
    pub video: VideoData,
    pub variables: HashMap<String, VariableValue>,
    pub costumes: HashMap<String, CostumeData>,
    pub sounds: HashMap<String, SoundData>,
}

pub struct GeneralData {
    name: String,
    x: f32,
    y: f32,
    size: f32,
    direction: f32,
    is_stage: bool,
}

pub struct LooksData {
    visible: bool,
    draggable: bool,
    rotation_style: RotationStyle,
    current_costume: u32,
    layer: u32,
}

pub struct AudioData {
    volume: f32,
    tts_language: Option<String>,
    tempo: Option<u32>,
}

pub struct VideoData {
    video_transparency: Option<String>,
    video_state: Option<String>,
}

pub struct CostumeData {
    bitmap_resolution: Option<u32>,
    md5_hash: String,
    asset_id: String,
    asset_filename: String,
    center_x: f32,
    center_y: f32,
}

pub struct SoundData {
    rate: u32,
    sample_count: u32,
    md5_hash: String,
    asset_id: String,
    asset_filename: String,
}

pub enum RotationStyle {
    DontRotate,
    LeftRight,
    AllAround,
}

pub enum VariableValue {
    Boolean(bool),
    Number(f32),
    Text(String),
}
