//! Generate fake data for faster debugging cycles.

use crate::data::{MediaCollection, Metadata, SyncResponse, ID};
use std::collections::HashMap;

/// Fake items that are "missing" from the in-memory metadata
/// store.
fn missing(n: usize) -> HashMap<ID, Metadata> {
    let mut missing = HashMap::new();
    for i in 0..n {
        missing.insert(
            ID::from(i.to_string()),
            Metadata {
                title: "NASA Video".to_owned(),
                format: "webm".to_owned(),
                shortname: format!("NASA {:02}", i),
                url: "https://file-examples.com/storage/fe4e1227086659fa1a24064/2020/03/file_example_WEBM_480_900KB.webm".to_owned(),
            },
        );
    }
    return missing;
}

/// Fake IDs that were supposedly present but don't correspond to
/// media on the server.
fn unknown(n: usize) -> Vec<ID> {
    let mut unknown = Vec::new();
    for i in 0..n {
        unknown.push(ID::from(i.to_string()));
    }
    return unknown;
}

pub async fn sync_local(_media: MediaCollection) -> anyhow::Result<SyncResponse> {
    Ok(crate::data::SyncResponse {
        missing: missing(12),
        unknown: vec![],
    })
}

pub async fn sync_remote(_media: MediaCollection) -> anyhow::Result<SyncResponse> {
    Ok(crate::data::SyncResponse {
        missing: missing(3),
        unknown: unknown(2),
    })
}

// non-resumable wrapper around resumable upload
pub async fn upload<'a>(_file: &'a web_sys::File) -> anyhow::Result<()> {
    anyhow::bail!("uploads not available for mock client");
}

#[inline]
pub fn origin() -> String {
    web_sys::window()
        .expect("window")
        .location()
        .origin()
        .expect("window.location.origin")
}

pub async fn convert(req: serde_json::Value) -> anyhow::Result<()> {
    anyhow::bail!("convert not available for mock client");
}