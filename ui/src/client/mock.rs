//! Generate fake data for faster debugging cycles.

use crate::{log, data::{Media, MediaItem}};
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref MEDIA: Mutex<Option<Media>> = Mutex::new(None);
}

fn init_media() -> Option<Media> {
    let mut m = Media::new();
    for i in 0..5 {
        m.insert(
                i.to_string(),
                MediaItem {
                    title: format!("Big Buck Bunny {}", i),
                    format: "webm".to_string(),
                    // shortname: format!("NASA {:02}", i),
                    url: "https://dl6.webmfiles.org/big-buck-bunny_trailer.webm".to_owned(),
                },
            );
    }
    return Some(m)
}

pub async fn get_media() -> Media {
    let mut media = MEDIA.lock().unwrap();
    if media.is_none() {
        *media = init_media()
    }
    return media.clone().unwrap();
}

pub async fn update_media(id: String, field: &str, value: &str) {
    let mut media = MEDIA.lock().unwrap();
    if media.is_none() {
        *media = init_media()
    }
    let m = media.as_mut().unwrap();
    if let Some(item) = m.get_mut(&id) {
        match field {
            "title" => item.title = value.to_string(),
            "format" => item.format = value.to_string(),
            _ => log!("unknown field for MediaItem: {}", field),
        }
    } else {
        log!("no media item with id: {}", id)
    }
}
