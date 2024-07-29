use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::data::MediaItem;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct ID(String);

impl From<String> for ID {
    fn from(value: String) -> Self {
        ID(value)
    }
}

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
            Sync(map) => {
                self.items = map
                    .into_iter()
                    .map(|item| (item.id.clone().into(), item))
                    .collect()
            }
            Create(id, item) => {
                self.items.insert(id, item);
            }
            Update { id, field, value } => {
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

    pub fn get(&self, id: &ID) -> Option<&MediaItem> {
        self.items.get(id)
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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum MediaEvent {
    Sync(Vec<MediaItem>),
    Create(ID, MediaItem),
    Update {
        id: ID,
        field: String,
        value: String,
    },
    Forget(ID),
    Null,
}

impl Default for MediaEvent {
    fn default() -> Self {
        Self::Null
    }
}
