#![allow(dead_code)]

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod client;
mod components;
mod data;
mod pages;

use pages::videos::VideoEditor;
use pages::videos::VideoSelector;

#[macro_export]
macro_rules! log {
    ( $expr:expr ) => {
        {
            web_sys::console::log_1(
                &format!("{} {}: {}", file! {}, line! {}, $expr.to_string()).into()
            );
        }
    };
    ( $lit:literal $(, $expr:expr)* ) => {
        {
            let msg = format!($lit, $($expr,)*);
            web_sys::console::log_1(
                &format!("{} {}: {}", file! {}, line! {}, msg).into()
            );
        }
    };
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
    provide_context(videos);
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
                            view! {
                                <div id="video-dashboard">
                                    <div id="video-selector">
                                        <VideoSelector/>
                                    </div>
                                    <div id="video-editor">
                                        <Outlet/>
                                    </div>
                                </div>
                            }
                        }
                    >

                        <Route path="" view=|| view! { <p>"No Video Selected"</p> }/>
                        <Route path=":id" view=VideoEditor/>

                    </Route>
                    <Route path="/*" view=pages::NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
