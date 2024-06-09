use std::collections::{BTreeMap, HashMap};

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
    // #[inline]
    // pub fn url(&self) -> String {
    //     crate::client::get_origin() + "/media/" + &self.path
    // }

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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SyncResponse {
    pub missing: HashMap<ID, Metadata>,
    pub unknown: Vec<ID>,
    // may include updated if multiple clients are ever supported
}

/// IDs are base64 encoded sha256 hashes.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
pub struct ID(String);

impl ID {
    pub fn gen(v: &str) -> Self {
        use base64::prelude::*;
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(v);
        let result = hasher.finalize();
        let id = BASE64_STANDARD.encode(result);
        return Self(id);
    }

    pub fn as_str<'a>(&'a self) -> &'a str {
        &self.0
    }
}

impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ID {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<ID> for String {
    fn from(value: ID) -> String {
        value.0
    }
}

impl<'a> From<&'a ID> for &'a str {
    fn from(value: &'a ID) -> &'a str {
        &value.0
    }
}

impl From<&str> for ID {
    fn from(value: &str) -> Self {
        ID(value.to_owned())
    }
}

impl leptos::IntoView for &ID {
    fn into_view(self) -> leptos::View {
        (&self.0).into_view()
    }
}

impl leptos::IntoView for ID {
    fn into_view(self) -> leptos::View {
        (&self.0).into_view()
    }
}

/// Just a `HashMap`, but may eventually need more
/// sophistication.
pub type MediaCollection = HashMap<ID, Metadata>;
pub type Collection<T: Serialize + for <'de> Deserialize<'de>> = HashMap<ID, T>;

