use crate::data::{Doc, Job, MediaLibrary, MediaLibraryResponse};
use futures::{Stream, TryStreamExt};
use gloo_net::{
    eventsource::{futures::EventSource, EventSourceError},
    http::Request,
};
use std::future::Future;

pub struct Client {
    origin: String,
}

impl Client {
    pub async fn media_library(&self) -> anyhow::Result<MediaLibrary> {
        let url = format!("{}/media_library", self.origin);
        let media_library = Request::get(&url)
            .send()
            .await?
            .json::<MediaLibrary>()
            .await?;
        // let media_library = MediaLibrary::try_from(Doc::parse(&body)?)?;
        Ok(media_library)
    }

    pub async fn media_library_event_stream(
        &self,
        event: &str,
    ) -> anyhow::Result<impl Stream<Item = Result<anyhow::Result<Doc>, EventSourceError>>> {
        let url = format!("{}/api/media_library/events", self.origin);
        let body = Request::get(&url)
            .send()
            .await?
            .body()
            .unwrap()
            .as_string()
            .unwrap();
        let response = MediaLibraryResponse::try_from(Doc::parse(&body)?)?;
        let mut src = EventSource::new(&response.event_url).unwrap();
        let event_stream = src.subscribe(event).unwrap().and_then(|(_event, message)| {
            futures::future::ok(Doc::parse(message.as_string().unwrap().as_str()))
        });
        Ok(event_stream)
    }

    pub fn job_library(&self) -> impl Future<Output = Vec<Job>> {
        async { todo!() }
    }
}

impl Default for Client {
    fn default() -> Self {
        // let location = web_sys::window().expect("window").location();
        // let protocol = location.protocol().expect("window.location.protocol");
        // let host = location.host().expect("window.location.host");
        // let base_url = if protocol == "https" {
        //     format!("wss://{}", host)
        // } else {
        //     format!("ws://{}", host)
        // };
        Self {
            origin: get_api_path(),
        }
    }
}

#[inline]
fn get_api_path() -> String {
    web_sys::window()
        .expect("window")
        .location()
        .origin()
        .expect("window.location.origin")
        + "/api"
    // let location = web_sys::window().expect("window").location();
    // let protocol = location.protocol().expect("window.location.protocol");
    // let hostname = location.hostname().expect("window.location.hostname");
    // let base_url = protocol + "//" + &hostname + ":8090" + "/api";
    // base_url
}
