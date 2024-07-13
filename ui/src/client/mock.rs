//! Generate fake data for faster debugging cycles.

use crate::{data::{Media, MediaFormat, MediaItem}, log};

/// Fake items that are "missing" from the in-memory metadata
/// store.
pub async fn get_media() -> Media {
    let mut media = Media::new();
    for i in 0..5 {
        media.insert(
            i.to_string(),
            MediaItem {
                title: format!("NASA Video {}", i),
                format: MediaFormat::Webm,
                // shortname: format!("NASA {:02}", i),
                // url: "https://file-examples.com/storage/fe4e1227086659fa1a24064/2020/03/file_example_WEBM_480_900KB.webm".to_owned(),
            },
        );
    }
    return media
}
