use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaItem {
    pub id: String,
    pub url: String,
    pub title: String,
    pub format: String,
}

impl MediaItem {
    pub fn kind(&self) -> &'static str {
        match self.format.as_str() {
            "mkv" | "mp4" | "ogg" | "webm" => "video",
            "jpeg" | "jpg" | "png" | "webp" => "image",
            _ => "unknown",
        }
    }

    pub fn update(&mut self, field: String, value: String) {
        match field.as_str() {
            "title" => self.title = value,
            "format" => self.format = value,
            _ => {}
        }
    }

    pub fn download_name(&self) -> String {
        if let Some(pos) = self.title.rfind(".") {
            if &self.title[pos..] == &self.format {
                return self.title.clone();
            }
        }
        format!("{}.{}", self.title, self.format)
    }
}
