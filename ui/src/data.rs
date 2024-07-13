use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Media = HashMap<String, MediaItem>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub format: String,
    pub title: String,
    pub url: String,
}
