use crate::{
    data::{Image, Video},
    log,
};

#[inline]
fn origin() -> String {
    if let Some(url) = option_env!("API_BASE_URL") {
        return url.to_owned();
    } else {
        web_sys::window()
            .expect("window")
            .location()
            .origin()
            .expect("window.location.origin")
    }
}

pub async fn get_videos() -> Vec<Video> {
    let response = gloo_net::http::Request::get(&format!("{}/api/videos", origin()))
        .send()
        .await;
    if response.is_err() {
        // browser already logs the error details
        return Default::default();
    }
    match response.unwrap().json::<_>().await {
        Ok(v) => v,
        Err(e) => {
            log!("{}", e);
            Default::default()
        }
    }
}

pub async fn update_video(id: String, field: String, value: String) {
    if gloo_net::http::Request::put(&format!("{}/api/videos/{}", origin(), id))
        .query([("f", field), ("v", value)])
        .send()
        .await
        .is_err()
    {
        // browser already logs errors
    };
}

pub async fn get_images() -> Vec<Image> {
    let response = gloo_net::http::Request::get(&format!("{}/api/images", origin()))
        .send()
        .await;
    if response.is_err() {
        // browser already logs the error details
        return Default::default();
    }
    match response.unwrap().json::<_>().await {
        Ok(v) => v,
        Err(e) => {
            log!("{}", e);
            Default::default()
        }
    }
}

pub async fn update_image(id: String, field: String, value: String) {
    if gloo_net::http::Request::put(&format!("{}/api/images/{}", origin(), id))
        .query([("f", field), ("v", value)])
        .send()
        .await
        .is_err()
    {
        // browser already logs errors
    };
}
