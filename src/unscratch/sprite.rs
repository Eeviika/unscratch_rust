use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::scratch::scratch_structs::{ScratchTarget, ScratchValue, ScratchVariable};

#[derive(Deserialize, Serialize, Debug)]
pub struct Sprite {
    pub general: GeneralData,
    pub looks: LooksData,
    pub audio: AudioData,
    pub video: VideoData,
    pub variables: HashMap<String, VariableValue>,
    pub cloud_variables: HashMap<String, VariableValue>,
    pub costumes: HashMap<String, CostumeData>,
    pub sounds: HashMap<String, SoundData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GeneralData {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<f32>,
    is_stage: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LooksData {
    visible: bool,
    draggable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation_style: Option<RotationStyle>,
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
    video_transparency: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_state: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CostumeData {
    #[serde(skip_serializing_if = "Option::is_none")]
    bitmap_resolution: Option<u32>,
    md5_hash: String,
    asset_filename: String,
    center_x: f32,
    center_y: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SoundData {
    rate: u32,
    sample_count: u32,
    md5_hash: String,
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
    Number(f64),
    Text(String),
}

impl From<ScratchValue> for VariableValue {
    fn from(value: ScratchValue) -> Self {
        match value {
            ScratchValue::Boolean(boolean) => Self::Boolean(boolean),
            ScratchValue::Text(text) => Self::Text(text),
            ScratchValue::Number(num) => Self::Number(num),
        }
    }
}

impl From<&str> for RotationStyle {
    fn from(value: &str) -> Self {
        match value {
            "all around" => Self::AllAround,
            "don't rotate" => Self::DontRotate,
            "left-right" => Self::LeftRight,
            _ => panic!("invalid rotation style type"),
        }
    }
}

impl From<ScratchTarget> for Sprite {
    fn from(target: ScratchTarget) -> Self {
        let rotation_style: Option<RotationStyle> = match target.rotation_style {
            None => None,
            Some(style) => Some(RotationStyle::from(&*style)),
        };

        let general = GeneralData {
            name: target.name,
            x: target.x,
            y: target.y,
            size: target.size,
            direction: target.direction,
            is_stage: target.is_stage,
        };

        let looks = LooksData {
            visible: target.visible,
            draggable: target.draggable,
            current_costume: target.current_costume,
            layer: target.layer_order,
            rotation_style: rotation_style,
        };

        let audio = AudioData {
            volume: target.volume,
            tts_language: target.text_to_speech_language,
            tempo: target.tempo,
        };

        let video = VideoData {
            video_transparency: target.video_transparency,
            video_state: target.video_state,
        };

        let mut variables: HashMap<String, VariableValue> = HashMap::new();
        let mut cloud_variables: HashMap<String, VariableValue> = HashMap::new();

        for ScratchVariable(name, value, is_cloud) in target.variables.into_values() {
            let value = VariableValue::from(value);

            if is_cloud {
                cloud_variables.insert(name, value);
            } else {
                variables.insert(name, value);
            }
        }

        let mut costumes: HashMap<String, CostumeData> = HashMap::new();
        let mut sounds: HashMap<String, SoundData> = HashMap::new();

        for costume in target.costumes {
            let data = CostumeData {
                bitmap_resolution: costume.bitmap_resolution,
                md5_hash: costume.md5ext,
                asset_filename: costume.asset_id,
                center_x: costume.rotation_center_x,
                center_y: costume.rotation_center_y,
            };
            costumes.insert(costume.name, data);
        }

        for sound in target.sounds {
            let data = SoundData {
                rate: sound.rate,
                sample_count: sound.sample_count,
                md5_hash: sound.md5ext,
                asset_filename: sound.asset_id,
            };
            sounds.insert(sound.name, data);
        }

        Self {
            general,
            looks,
            audio,
            video,
            variables,
            cloud_variables,
            costumes,
            sounds,
        }
    }
}
