use std::collections::HashMap;

use leptos::*;
use serde::{Deserialize, Serialize};

pub type Media = HashMap<String, MediaItem>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub format: MediaFormat,
    pub title: String,
}

impl IntoView for MediaItem {
    fn into_view(self) -> View {
        view! { <p>"Media Item"</p> }.into_view()
    }
}

impl MediaItem {
    pub fn summary(&self) -> MediaItemSummary {
        MediaItemSummary{
            title: self.title.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItemSummary {
    title: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum MediaFormat {
    Webm,
    Ogg,
    Mp4,
    Unknown,
}

impl IntoView for MediaItemSummary {
    fn into_view(self) -> View {
        view! { <p>{self.title}</p> }.into_view()
    }
}
