//! Generate fake data for faster debugging cycles.

use crate::{
    data::{Image, Images, Video, Videos},
    log,
};
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref VIDEOS: Mutex<Option<Videos>> = Mutex::new(None);
    static ref IMAGES: Mutex<Option<Images>> = Mutex::new(None);
}

fn init_videos() -> Option<Videos> {
    let mut m = Videos::new();
    for i in 0..5 {
        m.insert(
            i.to_string(),
            Video {
                title: format!("Big Buck Bunny {}", i),
                format: "webm".to_string(),
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
        log!("no video with id: {}", id)
    }
}

fn init_images() -> Option<Images> {
    let mut m = Images::new();
    for i in 1..6 {
        m.insert(
            i.to_string(),
            Image {
                title: format!("Blah {}", i),
                format: "webp".to_string(),
                url: format!("https://www.gstatic.com/webp/gallery/{}.webp", i),
            },
        );
    }
    return Some(m);
}

pub async fn get_images() -> Images {
    let mut images = IMAGES.lock().unwrap();
    if images.is_none() {
        *images = init_images()
    }
    return images.clone().unwrap();
}

pub async fn update_image(id: String, field: &str, value: &str) {
    let mut images = IMAGES.lock().unwrap();
    if images.is_none() {
        *images = init_images()
    }
    let i = images.as_mut().unwrap();
    if let Some(item) = i.get_mut(&id) {
        match field {
            "title" => item.title = value.to_string(),
            "format" => item.format = value.to_string(),
            _ => log!("unknown field for Video: {}", field),
        }
    } else {
        log!("no image with id: {}", id)
    }
}
