#[cfg(not(feature = "demo"))]
mod http;
#[cfg(not(feature = "demo"))]
pub use http::*;

#[cfg(feature = "demo")]
mod mock;
#[cfg(feature = "demo")]
pub use mock::*;

use crate::{
    data::{SyncResponse, ID},
    log, MediaCollection,
};
use std::collections::HashMap;

pub async fn sync_local(media: MediaCollection) -> anyhow::Result<SyncResponse> {
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let mut missing = HashMap::new();
    // TODO this is not efficient, but not worth optimizing yet
    if media.len() > 0 {
        // empty storage
        // localStorage **updates its keys** when an item
        // is removed, so count backwards
        for i in (0..storage.length().unwrap()).rev() {
            let key = storage.key(i).unwrap().unwrap();
            if key.starts_with("media/") {
                storage.remove_item(&key).unwrap();
            }
        }
        // populate storage from memory
        for (id, val) in media.iter() {
            let key = format!("media/{}", id);
            let sval = serde_json::to_string(&val).unwrap();
            storage.set_item(&key, &sval).unwrap();
        }
    } else {
        // populate memory from storage
        for i in 0..storage.length().unwrap() {
            let key = storage.key(i).unwrap().unwrap();
            if let Some(id) = key.strip_prefix("media/").map(|k| ID::from(k)) {
                if media.get(&id).is_none() {
                    let storage_val = storage.get_item(&key).unwrap().unwrap();
                    let val = serde_json::from_str(&storage_val).unwrap();
                    missing.insert(id, val);
                }
            }
        }
    }
    return Ok(SyncResponse {
        missing,
        unknown: vec![],
    });
}
