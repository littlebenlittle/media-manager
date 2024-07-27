#![allow(dead_code)]

mod client;
mod collection;
mod components;
mod data;
mod pages;
mod transport;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use collection::{MediaCollection, MediaEvent, ID};
use components::dashboard::{Editor, Selector};
use components::notification_tray::NotificationTray;
use transport::{ReqSSETransport, Transport};

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

fn media<T: Transport>(
    transport: T,
) -> (
    Signal<MediaCollection>,
    Signal<Option<MediaEvent>>,
    Action<MediaEvent, Result<MediaEvent, <T as Transport>::Error>>,
) {
    let (coll, set_coll) = create_signal(MediaCollection::default());
    let event = transport.subscribe();
    create_effect({
        move |_| {
            if let Some(ev) = event.get() {
                set_coll.update(|coll| match coll.handle(ev) {
                    Ok(_) => {}
                    Err(e) => log!("{:?}", e),
                })
            }
        }
    });
    let send_action = create_action(move |ev: &MediaEvent| {
        let ev = ev.clone();
        async move { transport.send(ev).await }
    });
    create_effect({
        let val = send_action.value();
        move |_| match val.get() {
            Some(Ok(ev)) => set_coll.update(|coll| match coll.handle(ev) {
                Err(e) => log!("{:?}", e),
                _ => {}
            }),
            Some(Err(e)) => log!("{:?}", e),
            None => {}
        }
    });
    return (coll.into(), event, send_action);
}

#[component]
pub fn App() -> impl IntoView {
    let (media, remote_event, emit_event) = media(ReqSSETransport::new());
    provide_context(media);
    provide_context(Box::new(remote_event));
    provide_context(Box::new(emit_event));
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
                    match remote_event() {
                        Some(MediaEvent::Create(_, item)) => {
                            Some(
                                view! {
                                    <a href=path(
                                        &(item.format.clone() + "/" + &item.id),
                                    )>{item.title}</a>
                                }
                                    .into_view(),
                            )
                        }
                        _ => None,
                    }
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
