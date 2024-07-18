#![allow(dead_code)]

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod client;
mod components;
mod data;
mod pages;

use data::{Image, Video};

use components::dashboard::{Editor, Selector};

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
    let videos = create_local_resource(|| (), |_| async { crate::client::get_videos().await });
    let images = create_local_resource(|| (), |_| async { crate::client::get_images().await });
    provide_context(videos);
    provide_context(images);
    provide_meta_context();
    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>
        <Title text="Media Manager"/>
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Router base=option_env!("APP_BASE_PATH").unwrap_or_default()>
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
                <Routes base=option_env!("APP_BASE_PATH").unwrap_or_default().to_owned()>
                    <Route path="/" view=pages::Home/>
                    <Route
                        path="videos"
                        view=|| {
                            let videos = use_context::<Resource<(), Vec<Video>>>().unwrap();
                            view! {
                                <div class="dashboard">
                                    <div class="selector">
                                        <Selector
                                            path="videos".to_string()
                                            items=videos
                                            key=|video| video.id.clone()
                                            filter=|search, video| video.title.contains(&search)
                                            title=|video| video.title.clone()
                                        />
                                    </div>
                                    <div class="editor">
                                        <Outlet/>
                                    </div>
                                </div>
                            }
                        }
                    >

                        <Route path="" view=|| view! { <p>"No Video Selected"</p> }/>
                        <Route
                            path=":id"
                            view=move || {
                                let videos = use_context::<Resource<(), Vec<Video>>>().unwrap();
                                let update_video = create_action(|v: &(String, String, String)| {
                                    let (id, field, value) = v.clone();
                                    async move {
                                        client::update_video(id, field, value).await;
                                    }
                                });
                                view! {
                                    <Editor
                                        items=videos
                                        key=|video| video.id.clone()
                                        update=move |id, field, value| {
                                            update_video
                                                .dispatch((
                                                    id.to_string(),
                                                    field.to_string(),
                                                    value.to_string(),
                                                ))
                                        }

                                        fields=|video| {
                                            vec![
                                                ("title".to_string(), video.title.clone(), true),
                                                ("format".to_string(), video.format.clone(), true),
                                                ("url".to_string(), video.url.clone(), false),
                                            ]
                                        }
                                    />
                                }
                            }
                        />

                    </Route>
                    <Route
                        path="images"
                        view=|| {
                            let images = use_context::<Resource<(), Vec<Image>>>().unwrap();
                            view! {
                                <div class="dashboard">
                                    <div class="selector">
                                        <Selector
                                            path="images".to_string()
                                            items=images
                                            key=|image| image.id.clone()
                                            filter=|search, image| image.title.contains(&search)
                                            title=|image| image.title.clone()
                                        />
                                    </div>
                                    <div class="editor">
                                        <Outlet/>
                                    </div>
                                </div>
                            }
                        }
                    >

                        <Route path="" view=|| view! { <p>"No Image Selected"</p> }/>
                        <Route
                            path=":id"
                            view=move || {
                                let images = use_context::<Resource<(), Vec<Image>>>().unwrap();
                                let update_image = create_action(|v: &(String, String, String)| {
                                    let (id, field, value) = v.clone();
                                    async move {
                                        client::update_image(id, field, value).await;
                                    }
                                });
                                view! {
                                    <Editor
                                        items=images
                                        key=|image| image.id.clone()
                                        update=move |id, field, value| {
                                            update_image
                                                .dispatch((
                                                    id.to_string(),
                                                    field.to_string(),
                                                    value.to_string(),
                                                ))
                                        }

                                        fields=|image| {
                                            vec![
                                                ("title".to_string(), image.title.clone(), true),
                                                ("format".to_string(), image.format.clone(), true),
                                                ("url".to_string(), image.url.clone(), false),
                                            ]
                                        }
                                    />
                                }
                            }
                        />

                    </Route>
                    // <Route
                    // path="images"
                    // view=|| {
                    // view! {
                    // <div class="dashboard">
                    // <div class="selector">
                    // <ImageSelector/>
                    // </div>
                    // <div class="editor">
                    // <Outlet/>
                    // </div>
                    // </div>
                    // }
                    // }
                    // >

                    // <Route path="" view=|| view! { <p>"No Image Selected"</p> }/>
                    // <Route path=":id" view=ImageEditor/>

                    // </Route>
                    <Route path="/*" view=pages::NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
