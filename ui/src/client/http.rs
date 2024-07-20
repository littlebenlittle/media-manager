use crate::{data::MediaItem, log};

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

pub async fn get_media() -> Vec<MediaItem> {
    let response = gloo_net::http::Request::get(&format!("{}/api/media", origin()))
        .send()
        .await;
    if response.is_err() {
        // browser already logs the error details
        return Default::default();
    }
    match response.unwrap().json::<Vec<MediaItem>>().await {
        Ok(v) => v,
        Err(e) => {
            log!("{}", e);
            Default::default()
        }
    }
}

pub async fn update_media(id: String, field: String, value: String) -> anyhow::Result<bool> {
    Ok(
        gloo_net::http::Request::patch(&format!("{}/api/media/{}", origin(), id))
            .query([("f", field), ("v", value)])
            .send()
            .await?
            .status()
            != 200,
    )
}

pub async fn upload_file(file: web_sys::File) {
    let (mut upload, loc) =
        match tus_web::new_upload(&file, &format!("{}/files", origin()), 8_000_000).await {
            Ok(u) => u,
            Err(e) => {
                log!("{}", e);
                return;
            }
        };
    match tus_web::continue_upload(&mut upload, &loc).await {
        Ok(()) => {}
        Err(e) => {
            log!("{}", e);
            return;
        }
    };
}
