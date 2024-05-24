#![allow(dead_code)]
use std::borrow::BorrowMut;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod client;
mod components;
mod data;
mod pages;

use leptos_use::utils::JsonCodec;
use leptos_use::{
    use_event_source_with_options, UseEventSourceOptions, UseEventSourceReturn,
};

// data types
use data::{Media, MediaLibrary};

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::player::{Player, VideoDashboard};

#[macro_export]
macro_rules! log {
    ( $expr:expr ) => {
        web_sys::console::log_1(
            &format!("{} {}: {}", file! {}, line! {}, $expr.to_string()).into()
        )
    };
    ( $lit:literal $(, $expr:expr)* ) => {
        let msg = format!($lit, $($expr)*);
        web_sys::console::log_1(
            &format!("{} {}: {}", file! {}, line! {}, msg).into()
        )
    };
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let new_media = {
        let UseEventSourceReturn { data, .. } = use_event_source_with_options::<Media, JsonCodec>(
            format!("{}/api/media_library/events", client::get_origin()).as_str(),
            UseEventSourceOptions::default()
                .immediate(true)
                .named_events(["new".to_owned()]),
        );
        data
    };

    let media_library = create_rw_signal(MediaLibrary::default());
    let init_media_library = create_local_resource(
        || (),
        move |_: ()| async move {
            match client::Client::default().media_library().await {
                Err(e) => {
                    log!(e);
                    None
                }
                Ok(lib) => {
                    log!("media library loaded");
                    Some(lib)
                }
            }
        },
    );

    create_effect(move |_| {
        if let Some(Some(ib)) = init_media_library.get() {
            media_library.set(ib);
        }
    });

    create_effect(move |_| {
        if let Some(new_media) = new_media.get() {
            media_library.update(move |lib| {
                if let Err(e) = lib.insert(new_media) {
                    log!(e)
                }
            })
        }
    });

    provide_context(init_media_library);
    provide_context(media_library);

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
                            <a href="/">"Home"</a>
                        </li>
                        <li>
                            <a href="/player">"Player"</a>
                        </li>
                    </ul>
                </nav>
            </div>
            <main>
                <div id="main-heading">
                    <h1>"Media Manager"</h1>
                </div>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/player" view=Player>
                        <Route path=":id" view=VideoDashboard/>
                        <Route
                            path=""
                            view=|| {
                                view! {
                                    <div id="select-media-source-notice">
                                        <h3>"<< Select Media Source"</h3>
                                    </div>
                                }
                            }
                        />

                    </Route>
                    <Route path="/*" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
