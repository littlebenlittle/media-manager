use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Media = HashMap<String, MediaItem>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub format: String,
    pub title: String,
    pub url: String,
}

// #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
// pub enum MediaFormat {
//     Webm,
//     Ogg,
//     Mp4,
//     Unknown,
// }

// impl From<&str> for MediaFormat {
//     fn from(value: &str) -> Self {
//         match value.to_lowercase().as_str() {
//             "webm" => Self::Webm,
//             "ogg" => Self::Ogg,
//             "mp4" => Self::Mp4,
//             _ => Self::Unknown,
//         }
//     }
// }
