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
