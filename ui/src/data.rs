use leptos::*;
use serde::{Deserialize, Serialize};

pub trait Media: Clone + IntoView {
    fn url(&self) -> String;
    fn title(&self) -> String;
    fn format(&self) -> String;
    fn key(&self) -> String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub id: String,
    pub format: String,
    pub title: String,
    pub url: String,
}

impl Media for Video {
    fn format(&self) -> String {
        self.format.clone()
    }
    fn key(&self) -> String {
        self.id.clone()
    }
    fn title(&self) -> String {
        self.title.clone()
    }
    fn url(&self) -> String {
        self.url.clone()
    }
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

impl Media for Image {
    fn format(&self) -> String {
        self.format.clone()
    }
    fn key(&self) -> String {
        self.id.clone()
    }
    fn title(&self) -> String {
        self.title.clone()
    }
    fn url(&self) -> String {
        self.url.clone()
    }
}
