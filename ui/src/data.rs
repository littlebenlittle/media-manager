use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Videos = HashMap<String, Video>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub id: String,
    pub format: String,
    pub title: String,
    pub url: String,
}

impl IntoView for Video {
    fn into_view(self) -> leptos::View {
        view! {
            <video>
                <source src=self.url/>
            </video>
        }
        .into_view()
    }
}

pub type Images = HashMap<String, Image>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub format: String,
    pub title: String,
    pub url: String,
}

impl IntoView for Image {
    fn into_view(self) -> leptos::View {
        view! { <img src=self.url/> }.into_view()
    }
}
