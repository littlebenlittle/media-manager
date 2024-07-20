use std::collections::HashMap;

use leptos::*;
use serde::{Deserialize, Serialize};

// impl IntoView for Video {
//     fn into_view(self) -> leptos::View {
//         view! {
//             <video>
//                 <source src=self.url/>
//             </video>
//         }
//         .into_view()
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub id: String,
    pub url: String,
    pub title: String,
    pub format: String,
}

pub struct Video(pub MediaItem);
pub struct Image(pub MediaItem);

// impl IntoView for Image {
//     fn into_view(self) -> leptos::View {
//         view! { <img src=self.url/> }.into_view()
//     }
// }
