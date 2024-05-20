#![allow(dead_code)]
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod client;
mod components;
mod data;
mod pages;

// hack for in-progress crate
mod doc_rs;
use doc_rs::Doc;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::player::{Player, VideoDashboard};

#[macro_export]
macro_rules! log {
    ( $e:expr ) => {
        web_sys::console::log_1(&format!("{} {}: {}", file! {}, line! {}, $e.to_string()).into())
    };
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let media_library = create_resource(
        || (),
        |_: ()| async move {
            match client::Client::default().media_library().await {
                Err(e) => {
                    log!(e);
                    None
                }
                Ok(media_library) => Some(media_library),
            }
        },
    );

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
                        <li><a href="/">"Home"</a></li>
                        <li><a href="/player">"Player"</a></li>
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
                        <Route path="" view=|| view!{
                            <div id="select-media-source-notice">
                                <h3>"<< Select Media Source"</h3>
                            </div>
                        }/>
                    </Route>
                    <Route path="/*" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
