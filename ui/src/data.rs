use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Videos = HashMap<String, Video>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub format: String,
    pub title: String,
    pub url: String,
}

pub type Images = HashMap<String, Image>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub format: String,
    pub title: String,
    pub url: String,
}
