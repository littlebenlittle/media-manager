use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub id: String,
    pub title: String,
    pub shortname: String,
    pub filetype: String,
    pub path: String,
}

impl Media {
    pub fn url(&self) -> String {
        let location = web_sys::window().expect("window").location();
        let protocol = location.protocol().expect("window.location.protocol");
        let hostname = location.hostname().expect("window.location.hostname");
        protocol + "//" + &hostname + ":8090" + &self.path
    }

    pub fn update(&mut self, path: &str, val: &str) {
        match path.as_ref() {
            "title" => self.title = val.to_string(),
            "path" => self.path = val.to_string(),
            "filetype" => self.filetype = val.to_string(),
            "shortname" => self.shortname = val.to_string(),
            p => panic!("no such field: {}", p),
        }
    }
}

impl TryFrom<Doc> for Media {
    type Error = anyhow::Error;
    fn try_from(_value: Doc) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Job {}

// TODO placeholder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Doc {}

impl Doc {
    pub fn parse(_s: &str) -> anyhow::Result<Self> {
        todo!()
    }
    pub fn sat(&self, _query: impl Into<Query>) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Query {}

impl Query {
    pub fn parse(_s: &str) -> anyhow::Result<Self> {
        Ok(Self {})
    }
}

impl<T: AsRef<str>> From<T> for Query {
    fn from(_value: T) -> Self {
        return Self {};
    }
}

pub struct MediaLibraryResponse {
    pub fetch_url: String,
    pub event_url: String,
}

impl TryFrom<Doc> for MediaLibraryResponse {
    type Error = anyhow::Error;
    fn try_from(_value: Doc) -> anyhow::Result<Self> {
        anyhow::bail!("doc missing field `stream_url`")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MediaLibrary {
    pub media: Vec<Media>,
}

impl MediaLibrary {
    pub fn get_media(&self, id: impl AsRef<str>) -> Option<Media> {
        let id = id.as_ref().to_string();
        self.media.iter().find(|m| m.id == id).map(|m| m.clone())
    }

    pub fn update(&mut self, id: &str, path: &str, val: &str) {
        let id = id.as_ref();
        for media in self.media.iter_mut() {
            if media.id == id {
                media.update(path.as_ref(), val.as_ref())
            }
        }
    }
}

impl TryFrom<Doc> for MediaLibrary {
    type Error = anyhow::Error;
    fn try_from(_value: Doc) -> Result<Self, Self::Error> {
        todo!()
    }
}
