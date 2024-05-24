use crate::data::{Media, MediaLibrary};
use gloo_net::{http::Request, websocket::futures::WebSocket};
use js_sys::wasm_bindgen::JsValue;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use wasm_bindgen_futures::JsFuture;

macro_rules! unwrap_js {
    ($result:expr) => {
        match $result {
            Ok(v) => v,
            Err(e) => anyhow::bail!(e.as_string().unwrap()),
        }
    };
}

pub struct Client {
    origin: String,
}

impl Client {
    pub async fn media_library(&self) -> anyhow::Result<MediaLibrary> {
        let url = format!("{}/api/media_library", self.origin);
        let media_library = Request::get(&url)
            .send()
            .await?
            .json::<MediaLibrary>()
            .await?;
        Ok(media_library)
    }

    pub async fn convert(&self, req: serde_json::Value) -> anyhow::Result<()> {
        let url = format!("{}/api/convert", self.origin);
        let query: Vec<(&str, String)> = ["id", "format", "hardsub", "overwrite"]
            .into_iter()
            .map(|field| (field, req.get(field).unwrap().to_string()))
            .collect();
        Request::get(&url).query(query).send().await?;
        // TODO report network errors
        // TODO get event stream to follow progress
        Ok(())
    }

    // non-resumable wrapper around resumable upload
    pub async fn upload<'a>(&self, file: &'a web_sys::File) -> anyhow::Result<()> {
        let (mut upload, location) = self.new_upload(file).await?;
        self.continue_upload(&mut upload, &location).await
    }

    // Register a new resumable upload using tus protocol
    pub async fn new_upload<'a>(
        &self,
        file: &'a web_sys::File,
    ) -> anyhow::Result<(ResumableUpload<'a>, String)> {
        let upload = unwrap_js!(new_upload(&file, 800_000).await);
        let res = gloo_net::http::Request::post(format!("{}/files", self.origin).as_str())
            .header("Content-Length", "0")
            .header("Upload-Length", upload.size().to_string().as_str())
            .header("Tus-Resumable", "1.0.0")
            // .header("Upload-Metadata", format!("filename {}", base64!(file.name()))
            .header("Content-Type", "application/offset+octet-stream")
            .send()
            .await?;
        if res.status() != 201 {
            anyhow::bail!("expected 201 Created")
        }
        let location = res.headers().get("Location").unwrap();
        return Ok((upload, location));
    }

    pub async fn continue_upload<'a>(
        &self,
        upload: &'a mut ResumableUpload<'a>,
        location: &str,
    ) -> anyhow::Result<()> {
        let chunk_sz = upload.chunk_size();
        let nchunks = upload.nchunks();
        let mut sent_ok = vec![true; nchunks as usize];
        let mut bad_res = Option::<gloo_net::http::Response>::None;
        for (chunk, index) in upload.iter_unsent() {
            let offset = if index < nchunks {
                index * chunk_sz
            } else {
                chunk.size() as i32
            };
            let arr = unwrap_js!(JsFuture::from(chunk.array_buffer()).await);
            let res = gloo_net::http::Request::patch(location)
                .header("Content-Length", chunk_sz.to_string().as_str())
                .header("Upload-Offset", offset.to_string().as_str())
                .header("Content-Type", "application/offset+octet-stream")
                .header("Tus-Resumable", "1.0.0")
                .body(arr)?
                .send()
                .await?;
            if res.status() != 204 {
                bad_res = Some(res);
                break;
            }
            sent_ok[index as usize] = true
        }
        for i in 0..nchunks {
            if sent_ok[i as usize] {}
        }
        if let Some(_) = bad_res {
            anyhow::bail!("bad response")
        }
        Ok(())
    }

    pub async fn update_media(&self, media: &Media) -> anyhow::Result<()> {
        let url = format!("{}/api/media_library/{}", self.origin, media.id);
        let res = Request::put(&url).json(media)?.send().await?;
        if res.status() != 202 {
            anyhow::bail!("update was not accepted")
        }
        Ok(())
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            origin: get_origin(),
        }
    }
}

#[inline]
pub fn get_origin() -> String {
    web_sys::window()
        .expect("window")
        .location()
        .origin()
        .expect("window.location.origin")
    // DEV
    // let location = web_sys::window().expect("window").location();
    // let protocol = location.protocol().expect("window.location.protocol");
    // let hostname = location.hostname().expect("window.location.hostname");
    // let base_url = protocol + "//" + &hostname + ":8090";
    // base_url
}
struct ChunkIter<'a> {
    file: &'a web_sys::Blob,
    file_sz: i32,
    chunk_sz: i32,
    i: i32,
}

impl<'a> Iterator for ChunkIter<'a> {
    type Item = web_sys::Blob;
    fn next(&mut self) -> Option<Self::Item> {
        let start = self.chunk_sz * self.i;
        if start > self.file_sz {
            return None;
        }
        self.i += 1;
        let end = (self.chunk_sz * self.i).min(self.file_sz);
        Some(self.file.slice_with_i32_and_i32(start, end).unwrap())
    }
}

impl<'a> ChunkIter<'a> {
    pub fn new(file: &'a web_sys::File, chunk_sz: i32) -> Self {
        Self {
            file,
            file_sz: file.size() as i32,
            chunk_sz,
            i: 0,
        }
    }
}

// TODO BYO digest function
// TODO extract to crate
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ResumableUploadData {
    hash: [[u8; 32]; 8],
    chunk_sz: i32,
    sent: Vec<bool>,
}

impl ResumableUploadData {
    pub async fn new<'a>(file: &'a web_sys::File, chunk_sz: i32) -> Result<Self, JsValue> {
        let nchunks = (file.size() as i32) % chunk_sz + 1;
        let sent = vec![false; nchunks as usize];
        Ok(Self {
            hash: Self::hash_parts(file).await?,
            chunk_sz,
            sent,
        })
    }

    pub async fn enliven<'a>(self, file: &'a web_sys::File) -> anyhow::Result<ResumableUpload<'a>> {
        let hash = unwrap_js!(Self::hash_parts(file).await);
        if hash != self.hash {
            anyhow::bail!("hashes don't match")
        }
        Ok(ResumableUpload { data: self, file })
    }

    async fn hash_parts(file: &web_sys::File) -> Result<[[u8; 32]; 8], JsValue> {
        let mut hasher = sha2::Sha256::default();
        for chunk in ChunkIter::new(file, 80_000) {
            let v = blob_to_vec(&chunk).await?;
            hasher.update(v)
        }
        let mut parts = [[0u8; 32]; 8];
        let mut i = 0;
        for part in hasher.finalize().chunks_exact(32) {
            parts[i] = part.try_into().unwrap();
            i += 1;
        }
        Ok(parts)
    }
}

pub struct ResumableUpload<'a> {
    data: ResumableUploadData,
    file: &'a web_sys::File,
}

pub async fn new_upload<'a>(
    file: &'a web_sys::File,
    chunk_sz: i32,
) -> Result<ResumableUpload<'a>, JsValue> {
    Ok(ResumableUpload {
        data: ResumableUploadData::new(file, chunk_sz).await?,
        file,
    })
}

impl<'a> ResumableUpload<'a> {
    fn iter_unsent(&'a self) -> impl Iterator<Item = (web_sys::Blob, i32)> + 'a {
        let iter = ChunkIter::new(self.file, self.data.chunk_sz);
        iter.zip(self.data.sent.iter())
            .zip(0..self.data.sent.len())
            .filter_map(move |((chunk, is_sent), index)| {
                if !is_sent {
                    Some((chunk, index as i32))
                } else {
                    None
                }
            })
    }

    pub fn sent_ok(&mut self, index: i32) {
        self.data.sent[index as usize] = true
    }

    #[inline(always)]
    fn chunk_size(&self) -> i32 {
        self.data.chunk_sz
    }

    #[inline(always)]
    fn nchunks(&self) -> i32 {
        self.data.sent.len() as i32
    }

    #[inline(always)]
    pub fn as_data(&self) -> ResumableUploadData {
        self.data.clone()
    }

    #[inline(always)]
    pub fn size(&self) -> i32 {
        self.file.size() as i32
    }
}

async fn blob_to_vec(blob: &web_sys::Blob) -> Result<Vec<u8>, js_sys::wasm_bindgen::JsValue> {
    let array_buffer = JsFuture::from(blob.array_buffer()).await?;
    let uint8_array = js_sys::Uint8Array::new(&array_buffer);
    Ok(uint8_array.to_vec())
}

// Unused functions below... for now

fn new_ws(ws_path: &str) -> anyhow::Result<WebSocket> {
    let location = web_sys::window().unwrap().location();
    let protocol = location.protocol().unwrap();
    let hostname = location.hostname().unwrap();
    let ws_proto = match protocol.as_str() {
        "http:" => "ws",
        "https:" => "wss",
        p => panic!("unhandled protocol: {}", p),
    };
    let url = format!("{}://{}/api/{}", ws_proto, hostname, ws_path);
    Ok(WebSocket::open(&url)?)
}
