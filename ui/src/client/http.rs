use crate::{data::Media, log};

#[inline]
fn origin() -> String {
    web_sys::window()
        .expect("window")
        .location()
        .origin()
        .expect("window.location.origin")
    // vv DEV vv
    // let location = web_sys::window().expect("window").location();
    // let protocol = location.protocol().expect("window.location.protocol");
    // let hostname = location.hostname().expect("window.location.hostname");
    // let base_url = protocol + "//" + &hostname + ":8090";
    // base_url
    // ^^ DEV ^^
}

pub async fn get_media() -> Media {
    let response = gloo_net::http::Request::get(&format!("{}/api/media", origin()))
        .send()
        .await;
    if response.is_err() {
        // browser already logs the error details
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

pub async fn update_media(id: String, field: &str, value: &str) {
    if gloo_net::http::Request::put(&format!("{}/api/media/{}", origin(), id))
        .query([("f", field), ("v", value)])
        .send()
        .await
        .is_err()
    {
        // browser already logs errors
    };
}
