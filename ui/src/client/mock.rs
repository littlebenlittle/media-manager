//! Generate fake data for faster debugging cycles.

use crate::data::{Collection, MediaCollection, Metadata, SyncResponse, ID};
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

pub async fn sync_remote(_media: MediaCollection) -> anyhow::Result<SyncResponse> {
    Ok(crate::data::SyncResponse {
        missing: missing(12),
        unknown: vec![],
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

pub async fn todo_list() -> Result<Collection<String>, String> {
    // TODO in a non-mock, should check if remote is available
    // and only pull from cache if not. If remote is indeed
    // available, it should write content to storage after it
    // has passed data back to caller, e.g. spawn a new thread.
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let mut items = Collection::new();
    for i in 0..storage.length().unwrap() {
        // TODO need a mutex around storage or this may panic
        let key = storage.key(i).unwrap().unwrap();
        if let Some(id) = key.strip_prefix("todo/") {
            let val = storage.get_item(&key).unwrap().unwrap();
            items.insert(id.into(), serde_json::from_str(&val).unwrap());
        }
    }
    return Ok(items);
}

pub async fn add_todo_item(text: String) -> Result<ID, String> {
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let id = uuid::Uuid::new_v4().to_string().into();
    let key = format!("todo/{}", id);
    storage.set_item(&key, &text).unwrap();
    // TODO in non-mock, spawn a task to write back to remote
    return Ok(id);
}

pub async fn edit_todo_item(id: ID, text: String) -> Result<(), String> {
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let key = format!("todo/{}", id);
    storage.set_item(&key, &text).unwrap();
    Ok(())
}
