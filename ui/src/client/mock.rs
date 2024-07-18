//! Generate fake data for faster debugging cycles.

use crate::{
    data::{Image, Images, Video, Videos},
    log,
};
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref VIDEOS: Mutex<Option<Vec<Video>>> = Mutex::new(None);
    static ref IMAGES: Mutex<Option<Vec<Image>>> = Mutex::new(None);
}

fn init_videos() -> Option<Vec<Video>> {
    let mut m = Vec::new();
    for i in 0..5 {
        m.push(Video {
            id: i.to_string(),
            title: format!("Big Buck Bunny {}", i),
            format: "webm".to_string(),
            url: "https://dl6.webmfiles.org/big-buck-bunny_trailer.webm".to_owned(),
        });
    }
    return Some(m);
}

pub async fn get_videos() -> Vec<Video> {
    let mut videos = VIDEOS.lock().unwrap();
    if videos.is_none() {
        *videos = init_videos()
    }
    return videos.clone().unwrap();
}

pub async fn update_video(id: String, field: String, value: String) {
    let mut videos = VIDEOS.lock().unwrap();
    if videos.is_none() {
        *videos = init_videos()
    }
    let v = videos.as_mut().unwrap();
    for item in v.iter_mut() {
        if item.id == id {
            match field.as_str() {
                "title" => item.title = value.to_string(),
                "format" => item.format = value.to_string(),
                _ => log!("unknown field for Video: {}", field),
            }
        }
    }
}

fn init_images() -> Option<Vec<Image>> {
    let mut m = Vec::new();
    for i in 1..6 {
        m.push(Image {
            id: i.to_string(),
            title: format!("Blah {}", i),
            format: "webp".to_string(),
            url: format!("https://www.gstatic.com/webp/gallery/{}.webp", i),
        });
    }
    return Some(m);
}

pub async fn get_images() -> Vec<Image> {
    let mut images = IMAGES.lock().unwrap();
    if images.is_none() {
        *images = init_images()
    }
    return images.clone().unwrap();
}

pub async fn update_image(id: String, field: String, value: String) {
    let mut images = IMAGES.lock().unwrap();
    if images.is_none() {
        *images = init_images()
    }
    let i = images.as_mut().unwrap();
    for item in i.iter_mut() {
        if item.id == id {
            match field.as_str() {
                "title" => item.title = value.to_string(),
                "format" => item.format = value.to_string(),
                _ => log!("unknown field for Video: {}", field),
            }
        }
    }
}
