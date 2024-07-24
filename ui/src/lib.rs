#![allow(dead_code)]

use std::collections::HashMap;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod client;
mod components;
mod data;
mod pages;

use data::{MediaItem, MediaUpdate};

use components::dashboard::{Editor, Selector};
use components::notification_tray::NotificationTray;

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
    let (media, set_media) = create_signal(HashMap::<String, MediaItem>::new());
    let get_media_action = create_action(|_: &()| async move { client::get_media().await });
    create_effect({
        let val = get_media_action.value();
        move |_| {
            if let Some(items) = val.get() {
                for item in items {
                    set_media.update(|m| {
                        m.insert(item.id.clone(), item);
                    })
                }
            }
        }
    });
    get_media_action.dispatch(());
    let update_item_action = create_action(|update: &MediaUpdate| {
        let u = update.clone();
        async move {
            match client::update_media(u.id.clone(), u.field.clone(), u.value.clone()).await {
                Ok(true) => Some(u),
                _ => None,
            }
        }
    });
    create_effect({
        let val = update_item_action.value();
        move |_| {
            if let Some(u) = val.get().flatten() {
                set_media.update(|m| {
                    if let Some(item) = m.get_mut(&u.id) {
                        item.update(u.field, u.value)
                    }
                })
            }
        }
    });
    let new_media_source = client::new_media();
    let (new_media, set_new_media) = create_signal(None::<(String, MediaItem)>);
    create_effect(move |_| {
        if let Some(item) = new_media_source.get() {
            let id = item.id.clone();
            set_media.update(|m| {
                m.insert(id.clone(), item.clone());
            });
            set_new_media.set(Some((id, item)))
        }
    });
    provide_context(update_item_action);
    provide_context(media);
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
                            <a href=path("video")>"Videos"</a>
                        </li>
                        <li>
                            <a href=path("image")>"Images"</a>
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
                <NotificationTray message=move || {
                    new_media
                        .get()
                        .map(|(id, item)| {
                            view! {
                                <a href=path(
                                    &format!("{}/{}", item.kind(), id),
                                )>"New Media! " {item.title}</a>
                            }
                                .into_view()
                        })
                }/>
                <Routes base=option_env!("APP_BASE_PATH").unwrap_or_default().to_owned()>
                    <Route path="/" view=pages::Home/>
                    <Route
                        path="video"
                        view=|| {
                            view! {
                                <div class="dashboard">
                                    <Selector
                                        path="video".to_string()
                                        filter=|search, item| {
                                            item.title.to_lowercase().contains(&search.to_lowercase())
                                                && item.kind() == "video"
                                        }
                                    />

                                    <Outlet/>
                                </div>
                            }
                        }
                    >

                        <Route path="" view=|| view! {}/>
                        <Route
                            path=":id"
                            view=move || {
                                view! {
                                    <Editor render=|url| {
                                        view! {
                                            <video controls>
                                                <source src=url/>
                                            </video>
                                        }
                                    }/>
                                }
                            }
                        />

                    </Route>
                    <Route
                        path="image"
                        view=|| {
                            view! {
                                <div class="dashboard">
                                    <Selector
                                        path="image".to_string()
                                        filter=|search, item| {
                                            item.title.to_lowercase().contains(&search.to_lowercase())
                                                && item.kind() == "image"
                                        }
                                    />

                                    <Outlet/>
                                </div>
                            }
                        }
                    >

                        <Route path="" view=|| view! {}/>
                        <Route
                            path=":id"
                            view=move || {
                                view! {
                                    <Editor render=|url| {
                                        view! { <img src=url/> }
                                    }/>
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
