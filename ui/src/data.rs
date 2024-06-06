use std::collections::BTreeMap;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub shortname: String,
    pub format: String,
    pub url: String,
    // pub timestamp: i32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Media {
    pub id: String,
    pub title: String,
    pub shortname: String,
    pub format: String,
    pub path: String,
}

impl Media {
    #[inline]
    pub fn url(&self) -> String {
        crate::client::get_origin() + "/media/" + &self.path
    }

    pub fn update(&mut self, path: &str, val: &str) {
        match path.as_ref() {
            "title" => self.title = val.to_string(),
            "path" => self.path = val.to_string(),
            "filetype" => self.format = val.to_string(),
            "shortname" => self.shortname = val.to_string(),
            p => panic!("no such field: {}", p),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MediaLibrary(BTreeMap<String, RwSignal<Media>>);

impl MediaLibrary {
    pub fn get_media(&self, id: impl AsRef<str>) -> Option<RwSignal<Media>> {
        let id = id.as_ref().to_string();
        self.0
            .iter()
            .find_map(|(mid, m)| if *mid == id { Some(m.clone()) } else { None })
    }

    pub fn media<'a>(&'a self) -> impl Iterator<Item = RwSignal<Media>> + 'a {
        self.0.iter().map(|(_, m)| m.clone())
    }

    pub fn insert(&mut self, media: Media) -> anyhow::Result<()> {
        if self.0.contains_key(&media.id) {
            anyhow::bail!("id conflict")
        }
        let id = media.id.clone();
        let sig = create_rw_signal(media);
        self.0.insert(id, sig);
        Ok(())
    }

    // pub fn update(&mut self, id: &str, path: &str, val: &str) {
    //     if let Some(media) = self.0.get_mut(id) {
    //         media.update(path.as_ref(), val.as_ref())
    //     }
    // }
}
