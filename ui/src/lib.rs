#![allow(dead_code)]

use std::collections::HashMap;

use futures::{channel::mpsc::channel, SinkExt};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod client;
mod components;
mod data;
mod pages;

use data::MediaItem;

use components::dashboard::{Editor, Selector};
use components::toaster::Toaster;

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => (web_sys::console::log_1(
        &leptos::wasm_bindgen::JsValue::from(
            format!("{} {}: {}",
                file! {},
                line! {},
                format_args!($($t)*).to_string()
            )
        )
    ));
}

#[macro_export]
macro_rules! unwrap_js {
    ($result:expr) => {
        match $result {
            Ok(v) => v,
            Err(e) => anyhow::bail!(e.as_string().unwrap()),
        }
    };
}

/// Return the relative path from `APP_BASE_PATH`
pub(crate) fn path(p: &str) -> String {
    if let Some(base) = option_env!("APP_BASE_PATH") {
        if p == "" {
            format!("/{}", base)
        } else {
            format!("/{}/{}", base, p)
        }
    } else {
        format!("/{}", p)
    }
}

#[component]
pub fn App() -> impl IntoView {
    let media = create_media();
    let client_update = create_update_action(media);
    let server_update = client::media_update();
    let server_update =
        move || server_update.with(|u| u.clone().map(|u| view! { <p>{u}</p> }.into_view()));
    provide_context(media);
    provide_context(client_update);
    provide_meta_context();
    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>
        <Title text="Media Manager"/>
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Router>
            <div id="nav-container">
                <nav>
                    <ul>
                        <li>
                            <a href=path("")>"Home"</a>
                        </li>
                        <li>
                            <a href=path("videos")>"Videos"</a>
                        </li>
                        <li>
                            <a href=path("images")>"Images"</a>
                        </li>
                    </ul>
                </nav>
            </div>
            <main>
                <div id="main-heading">
                    <div id="heading-ctr">
                        <h1>"Media Manager"</h1>

                        {
                            #[cfg(feature = "demo")]
                            view! { <h2>"Demo Mode"</h2> }
                        }

                    </div>
                </div>
                <Toaster message=server_update/>
                <Routes base=option_env!("APP_BASE_PATH").unwrap_or_default().to_owned()>
                    <Route path="/" view=pages::Home/>
                    <Route
                        path="videos"
                        view=|| {
                            view! {
                                <div class="dashboard">
                                    <Selector
                                        path="videos".to_string()
                                        filter=|search, item| {
                                            item.title.to_lowercase().contains(&search.to_lowercase())
                                                && video_filter(&item)
                                        }
                                    />

                                    <Outlet/>
                                </div>
                            }
                        }
                    >

                        <Route path="" view=|| view! { <p>"No Video Selected"</p> }/>
                        <Route
                            path=":id"
                            view=move || {
                                view! {
                                    <Editor
                                        filter=video_filter
                                        render=|item| {
                                            view! {
                                                <video controls>
                                                    <source src=item.url/>
                                                </video>
                                            }
                                        }
                                    />
                                }
                            }
                        />

                    </Route>
                    <Route
                        path="images"
                        view=|| {
                            view! {
                                <div class="dashboard">
                                    <div class="selector">
                                        <Selector
                                            path="images".to_string()
                                            filter=|search, item| {
                                                item.title.to_lowercase().contains(&search.to_lowercase())
                                                    && image_filter(&item)
                                            }
                                        />

                                    </div>
                                    <Outlet/>
                                </div>
                            }
                        }
                    >

                        <Route path="" view=|| view! { <p>"No Image Selected"</p> }/>
                        <Route
                            path=":id"
                            view=move || {
                                view! {
                                    <Editor
                                        filter=image_filter
                                        render=|item| {
                                            view! { <img src=item.url/> }
                                        }
                                    />
                                }
                            }
                        />

                    </Route>
                    <Route path="/*" view=pages::NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

fn video_filter(m: &&MediaItem) -> bool {
    match m.format.as_str() {
        "mp4" | "webm" | "mkv" | "ogg" => true,
        _ => false,
    }
}

fn image_filter(m: &&MediaItem) -> bool {
    match m.format.as_str() {
        "jpg" | "jpeg" | "png" | "webp" => true,
        _ => false,
    }
}

#[derive(Clone)]
struct MediaUpdate {
    id: String,
    field: String,
    value: String,
}

fn create_update_action(
    media: RwSignal<Vec<RwSignal<MediaItem>>>,
) -> Action<MediaUpdate, Option<MediaUpdate>> {
    let update = create_action(move |v: &MediaUpdate| {
        let v = v.clone();
        async move {
            match client::update_media(v.id.clone(), v.field.clone(), v.value.clone()).await {
                Ok(true) => Some(v),
                _ => None,
            }
        }
    });
    let update_value = update.value();
    create_effect(move |_| {
        if let Some(v) = update_value.get().flatten() {
            if let Some(item) = media
                .get_untracked()
                .into_iter()
                .find(|item| item.get_untracked().id == v.id)
            {
                item.update(move |item| match v.field.as_str() {
                    "title" => item.title = v.value,
                    "format" => item.format = v.value,
                    f => log!("unknown field: {}", f),
                })
            }
        }
    });
    return update;
}

fn create_media() -> RwSignal<Vec<RwSignal<MediaItem>>> {
    let media = create_rw_signal(Vec::<RwSignal<MediaItem>>::new());
    let server_media = create_local_resource(|| (), |_| async { crate::client::get_media().await });
    create_effect(move |_| {
        if let Some(m) = server_media.get() {
            media.set(m.into_iter().map(|item| create_rw_signal(item)).collect())
        }
    });
    return media;
}
