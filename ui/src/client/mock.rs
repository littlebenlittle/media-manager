//! Generate fake data for faster debugging cycles.

use crate::{
    data::{Video, Videos},
    log,
};
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref VIDEOS: Mutex<Option<Videos>> = Mutex::new(None);
}

fn init_videos() -> Option<Videos> {
    let mut m = Videos::new();
    for i in 0..5 {
        m.insert(
            i.to_string(),
            Video {
                title: format!("Big Buck Bunny {}", i),
                format: "webm".to_string(),
                // shortname: format!("NASA {:02}", i),
                url: "https://dl6.webmfiles.org/big-buck-bunny_trailer.webm".to_owned(),
            },
        );
    }
    return Some(m);
}

pub async fn get_videos() -> Videos {
    let mut videos = VIDEOS.lock().unwrap();
    if videos.is_none() {
        *videos = init_videos()
    }
    return videos.clone().unwrap();
}

pub async fn update_video(id: String, field: &str, value: &str) {
    let mut videos = VIDEOS.lock().unwrap();
    if videos.is_none() {
        *videos = init_videos()
    }
    let v = videos.as_mut().unwrap();
    if let Some(item) = v.get_mut(&id) {
        match field {
            "title" => item.title = value.to_string(),
            "format" => item.format = value.to_string(),
            _ => log!("unknown field for Video: {}", field),
        }
    } else {
        log!("no video item with id: {}", id)
    }
}
