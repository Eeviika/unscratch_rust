use anyhow::{Error, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::scratch::scratch_structs::{ScratchList, ScratchTarget, ScratchValue, ScratchVariable};

#[derive(Deserialize, Serialize, Debug)]
pub struct Sprite {
    #[serde(skip)]
    pub raw_lists: Option<HashMap<String, ScratchList>>,
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

impl TryFrom<&str> for RotationStyle {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "all around" => Ok(Self::AllAround),
            "don't rotate" => Ok(Self::DontRotate),
            "left-right" => Ok(Self::LeftRight),
            _ => bail!("invalid rotation style type: {value}"),
        }
    }
}

impl TryFrom<ScratchTarget> for Sprite {
    type Error = Error;

    fn try_from(target: ScratchTarget) -> Result<Self, Self::Error> {
        let ScratchTarget {
            x,
            y,
            size,
            direction,
            visible,
            draggable,
            rotation_style,
            is_stage,
            name,
            current_costume,
            volume,
            layer_order,
            tempo,
            video_transparency,
            text_to_speech_language,
            video_state,
            variables: scratch_variables,
            costumes: scratch_costumes,
            sounds: scratch_sounds,
            lists,
            ..
        } = target;

        let rotation_style = rotation_style
            .map(|style| RotationStyle::try_from(style.as_str()))
            .transpose()?;

        let general = GeneralData {
            name,
            x,
            y,
            size,
            direction,
            is_stage,
        };

        let looks = LooksData {
            visible,
            draggable,
            current_costume,
            layer: layer_order,
            rotation_style,
        };

        let audio = AudioData {
            volume,
            tts_language: text_to_speech_language,
            tempo,
        };

        let video = VideoData {
            video_transparency,
            video_state,
        };

        let mut variables = HashMap::with_capacity(scratch_variables.len());
        let mut cloud_variables = HashMap::with_capacity(scratch_variables.len());

        for ScratchVariable(name, value, is_cloud) in scratch_variables.into_values() {
            let value = VariableValue::from(value);

            if is_cloud {
                cloud_variables.insert(name, value);
            } else {
                variables.insert(name, value);
            }
        }

        let costumes = scratch_costumes
            .into_iter()
            .map(|costume| {
                let data = CostumeData {
                    bitmap_resolution: costume.bitmap_resolution,
                    md5_hash: costume.md5ext,
                    asset_filename: costume.asset_id,
                    center_x: costume.rotation_center_x,
                    center_y: costume.rotation_center_y,
                };
                (costume.name, data)
            })
            .collect();

        let sounds = scratch_sounds
            .into_iter()
            .map(|sound| {
                let data = SoundData {
                    rate: sound.rate,
                    sample_count: sound.sample_count,
                    md5_hash: sound.md5ext,
                    asset_filename: sound.asset_id,
                };
                (sound.name, data)
            })
            .collect();

        Ok(Self {
            raw_lists: Some(lists),
            general,
            looks,
            audio,
            video,
            variables,
            cloud_variables,
            costumes,
            sounds,
        })
    }
}
