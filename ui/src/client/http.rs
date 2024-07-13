use gloo_net::{http::Request, websocket::futures::WebSocket};
use js_sys::wasm_bindgen::JsValue;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::fmt;
use wasm_bindgen_futures::JsFuture;

use crate::{data::Media, log};

#[inline]
fn origin() -> String {
    // web_sys::window()
    //     .expect("window")
    //     .location()
    //     .origin()
    //     .expect("window.location.origin")
    // vv DEV vv
    let location = web_sys::window().expect("window").location();
    let protocol = location.protocol().expect("window.location.protocol");
    let hostname = location.hostname().expect("window.location.hostname");
    let base_url = protocol + "//" + &hostname + ":8090";
    base_url
    // ^^ DEV ^^
}

pub async fn get_media() -> Media {
    let response = gloo_net::http::Request::get(&format!("{}/media", origin()))
        // .query([("q", &query)])
        .send()
        .await;
    if response.is_err() {
        // browser already logs the request error details
        return Default::default();
    }
    match response.unwrap().json::<Media>().await {
        Ok(v) => v,
        Err(e) => {
            log!("{}", e);
            Default::default()
        }
    }
}
