//! Generate fake data for faster debugging cycles.

use crate::{data::MediaItem, log};
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref MEDIA: Mutex<Option<Vec<MediaItem>>> = Mutex::new(None);
}

fn init_media() -> Option<Vec<MediaItem>> {
    let mut m = Vec::new();
    for i in 1..6 {
        m.push(MediaItem {
            id: i.to_string(),
            title: format!("Blah {}", i),
            format: "webp".to_string(),
            url: format!("https://www.gstatic.com/webp/gallery/{}.webp", i),
        });
    }
    for i in 0..5 {
        m.push(MediaItem {
            id: (7 + i).to_string(),
            title: format!("Big Buck Bunny {}", i),
            format: "webm".to_string(),
            url: "https://dl6.webmfiles.org/big-buck-bunny_trailer.webm".to_owned(),
        });
    }
    return Some(m);
}

pub async fn get_media() -> Vec<MediaItem> {
    let mut media = MEDIA.lock().unwrap();
    if media.is_none() {
        *media = init_media()
    }
    return media.clone().unwrap();
}

pub async fn update_media(id: String, field: String, value: String) -> anyhow::Result<bool> {
    let mut media = MEDIA.lock().unwrap();
    if media.is_none() {
        *media = init_media()
    }
    let v = media.as_mut().unwrap();
    for item in v.iter_mut() {
        if item.id == id {
            match field.as_str() {
                "title" => item.title = value.to_string(),
                "format" => item.format = value.to_string(),
                _ => log!("unknown field for Video: {}", field),
            }
        }
    }
    Ok(true)
}
