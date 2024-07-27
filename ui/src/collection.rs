use core::fmt;
use std::collections::HashMap;

use js_sys::IntoIter;
use serde::{Deserialize, Serialize};

use crate::data::MediaItem;

#[derive(PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct ID(String);

impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <String as std::fmt::Display>::fmt(&self.0, f)
    }
}

#[derive(Clone)]
pub struct MediaCollection {
    items: HashMap<ID, MediaItem>,
}

impl MediaCollection {
    pub fn handle(&mut self, ev: MediaEvent) -> anyhow::Result<()> {
        use MediaEvent::*;
        match ev {
            Create(id, item) => {
                self.items.insert(id, item);
            }
            Update(id, field, value) => {
                if let Some(item) = self.items.get_mut(&id) {
                    item.update(field, value)
                }
            }
            Forget(id) => {
                self.items.remove(&id);
            }
            Null => {}
        }
        Ok(())
    }
}

impl IntoIterator for MediaCollection {
    type Item = <HashMap<ID, MediaItem> as IntoIterator>::Item;
    type IntoIter = <HashMap<ID, MediaItem> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl Default for MediaCollection {
    fn default() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum MediaEvent {
    Create(ID, MediaItem),
    Update(ID, String, String),
    Forget(ID),
    Null,
}

impl Default for MediaEvent {
    fn default() -> Self {
        Self::Null
    }
}
