//use anyhow::{bail, Result};
use leptos::*;
use serde::{Deserialize, Serialize};

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
    <ErrorBoundary fallback=|errors| {
        view! {
            <h1>"Uh oh! Something went wrong!"</h1>
            <p>"Errors: "</p>
            <ul>
                {move || {
                    errors
                        .get()
                        .into_iter()
                        .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                        .collect_view()
                }}

            </ul>
        }
    }>
        <Main />
    </ErrorBoundary>
    }
}

#[component]
fn Main() -> impl IntoView {
    view! {
    <div class="column">
        <div class="row">
            <MkvControls />
            <WebmControls />
        </div>
    </div>
    }
}

#[component]
fn WebmControls() -> impl IntoView {
    let webm_list = create_media_list_resouce("webm");
    let (selected_webm, set_selected_webm) = create_signal(Option::<String>::None);
    view! {
    <div class="row">
        <div class="column">
            <h3>"WEBM Files"</h3>
            <button on:click=move |_| webm_list.refetch()>"Reload"</button>
        {
            move || match webm_list.get() {
                None => view!{ <p>"Loading..."</p> }.into_view(),
                Some(media) => view!{
                    <MediaSelector filetype="webm" media set_selected_media=set_selected_webm/>
                }.into_view()
            }
        }
        </div>
        <div class="column">
        {
            move || match selected_webm.get() {
                None => view!{ <p>"Select a Media Source"</p> }.into_view(),
                Some(url) => view!{
                    <video controls max-width="512">
                        <source src={url}/>
                    </video>
                    }.into_view(),
            }
        }
        </div>
    </div>
    }
}

#[component]
fn MkvControls() -> impl IntoView {
    let mkv_list = create_media_list_resouce("mkv");
    let convert = create_action(|filename: &String| {
        let filename = filename.to_owned();
        async move {
            if let Err(e) = convert_media(&filename).await {
                log(e.to_string())
            }
        }
    });
    let (selected_mkv, set_selected_mkv) = create_signal(Option::<String>::None);
    view! {
    <div class="row">
        <div class="column">
            <h3>"MKV Files"</h3>
            <button on:click=move |_| mkv_list.refetch()>"Reload"</button>
        {
            move || match mkv_list.get() {
                None => view!{ <p>"Loading..."</p> }.into_view(),
                Some(media) => view!{
                    <MediaSelector filetype="mkv" media set_selected_media=set_selected_mkv/>
                }.into_view()
            }
        }
        </div>
        <div class="column">
        {
            move || match selected_mkv.get() {
                None => view!{ <p>"Select Media"</p> }.into_view(),
                Some(media) => {
                    view!{
                        <div class="column">
                            <a href=&media>"Download"</a>
                            <button on:click=move |_| convert.dispatch(media.clone())>
                                "Convert"
                            </button>
                        </div>
                    }.into_view()
                }
            }
        }
        </div>
    </div>
    }
}

#[component]
fn MediaSelector(
    filetype: &'static str,
    media: Vec<Media>,
    set_selected_media: WriteSignal<Option<String>>,
) -> impl IntoView {
    view! {
    <select size="25" on:change=move |e| set_selected_media.update(|v| *v = Some(event_target_value(&e)))>
    {
        media.into_iter().map(|m: Media| view! {
            <option value={format!("http://{}/media/{}/{}", get_host(), filetype, &m.title)}>{format!("{}", m.title)}</option>
        })
        .collect_view()
    }
    </select>
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Media {
    title: String,
}

async fn fetch_media_list(filetype: &str) -> Result<Vec<Media>, error::Error> {
    let client = reqwest_wasm::Client::default();
    let url = format!("http://{}/api/{}", get_host(), filetype);
    let response = client.get(url).send().await?;
    match response.error_for_status() {
        Ok(res) => Ok(res.json::<Vec<Media>>().await?),
        Err(e) => Err(e.into()),
    }
}

async fn convert_media(filename: &str) -> Result<(), error::Error> {
    let client = reqwest_wasm::Client::default();
    let url = format!("http://{}/api/convert/{}", get_host(), filename);
    let response = client.get(url).send().await?;
    match response.error_for_status() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

fn log_error(e: error::Error) {
    // TODO: toaster
    web_sys::console::log_1(&e.to_string().into())
}

fn log(msg: impl Into<String>) {
    // TODO: toaster
    web_sys::console::log_1(&msg.into().into())
}

fn create_media_list_resouce(filetype: &'static str) -> Resource<(), Vec<Media>> {
    create_resource(
        || (),
        move |_| async move {
            match fetch_media_list(filetype).await {
                Ok(list) => list,
                Err(e) => {
                    log_error(e);
                    Vec::new()
                }
            }
        },
    )
}

fn get_host() -> String {
    let window = web_sys::window().expect("couldn't get window");
    window
        .location()
        .host()
        .expect("could not get location.host")
}
