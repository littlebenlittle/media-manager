//! Generate fake data for faster debugging cycles.

use std::collections::HashMap;
use crate::data::Metadata;
use crate::store::ID;

/// Fake items that are "missing" from the in-memory metadata
/// store.
pub fn missing(n: usize) -> HashMap<ID, Metadata> {
    let mut missing = HashMap::new();
    for i in 0..n {
        missing.insert(
            ID::from(i.to_string()),
            Metadata {
                title: "blah".to_owned(),
                format: "blah blah".to_owned(),
                shortname: "blah".to_owned(),
                url: "media/blah".to_owned(),
            },
        );
    }
    return missing;
}

/// Fake IDs that were supposedly present but don't correspond to
/// media on the server.
pub fn unknown(n: usize) -> Vec<ID> {
    let mut unknown = Vec::new();
    for i in 0..n {
        unknown.push(ID::from(i.to_string()));
    }
    return unknown;
}
