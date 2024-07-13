use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Media = HashMap<String, MediaItem>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub format: MediaFormat,
    pub title: String,
    pub url: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum MediaFormat {
    Webm,
    Ogg,
    Mp4,
    Unknown,
}

impl From<String> for MediaFormat {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "webm" => Self::Webm,
            "ogg" => Self::Ogg,
            "mp4" => Self::Mp4,
            _ => Self::Unknown,
        }
    }
}
