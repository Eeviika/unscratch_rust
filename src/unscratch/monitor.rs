use anyhow::{Error, Ok, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::scratch::scratch_structs::ScratchMonitor;

#[derive(Serialize, Deserialize)]
pub enum MonitorMode {
    Default,
    Large,
    Slider,
    List,
}

#[derive(Serialize, Deserialize)]
pub struct Monitor {
    #[serde(skip, default)]
    id: Option<String>,
    general: GeneralData,
    looks: LooksData,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    slider: Option<SliderData>,
}

#[derive(Serialize, Deserialize)]
pub struct GeneralData {
    is_discrete: bool,
    mode: MonitorMode,
    sprite_name: Option<String>,
    opcode: String,
    params: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct LooksData {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub visible: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SliderData {
    pub min: f32,
    pub max: f32,
}

impl TryFrom<&str> for MonitorMode {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "default" => Ok(MonitorMode::Default),
            "large" => Ok(MonitorMode::Large),
            "slider" => Ok(MonitorMode::Slider),
            "list" => Ok(MonitorMode::List),
            _ => bail!("Invalid value: {value}"),
        }
    }
}

impl TryFrom<ScratchMonitor> for Monitor {
    type Error = Error;

    fn try_from(value: ScratchMonitor) -> Result<Self, Self::Error> {
        let ScratchMonitor {
            id,
            mode,
            opcode,
            params,
            sprite_name,
            value,
            width,
            height,
            x,
            y,
            visible,
            slider_min,
            slider_max,
            is_discrete,
        } = value;

        let id = Some(id);
        let mode = MonitorMode::try_from(mode.as_str())?;

        let general = GeneralData {
            is_discrete,
            sprite_name,
            opcode,
            params,
            mode,
        };

        let looks = LooksData {
            x,
            y,
            width,
            height,
            visible,
        };

        let mut slider: Option<SliderData> = None;

        if slider_min.is_some() && slider_max.is_some() {
            let min = slider_min.unwrap();
            let max = slider_max.unwrap();

            slider = Some(SliderData { min, max });
        }

        Ok(Monitor {
            id,
            general,
            looks,
            slider,
        })
    }
}
