#![allow(dead_code)]

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod client;
mod components;
mod data;
mod pages;

// data types
use data::{MediaCollection, Metadata, SyncResponse, ID};
use leptos_use::{use_timeout_fn, UseTimeoutFnReturn};

// Components
use crate::components::SyncButton;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::jobs::JobsDashboard;
use crate::pages::not_found::NotFound;
use crate::pages::player::{Dashboard as MediaDashboard, DashboardNoId as MediaDashboardNoId};

#[macro_export]
macro_rules! log {
    ( $expr:expr ) => {
        web_sys::console::log_1(
            &format!("{} {}: {}", file! {}, line! {}, $expr.to_string()).into()
        )
    };
    ( $lit:literal $(, $expr:expr)* ) => {
        let msg = format!($lit, $($expr,)*);
        web_sys::console::log_1(
            &format!("{} {}: {}", file! {}, line! {}, msg).into()
        )
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

#[derive(Clone)]
struct Context {
    media: ReadSignal<MediaCollection>,
    selected_media: ReadSignal<Option<Metadata>>,
    set_selected_id: WriteSignal<Option<ID>>,
    set_updated_media: WriteSignal<Option<Metadata>>,
}

#[component]
pub fn App() -> impl IntoView {
    // all of the media the client is currently aware of
    let (media, set_media) = create_signal(MediaCollection::new());
    // the id selected by the user for viewing/editing
    let (selected_id, set_selected_id) = create_signal(Option::<ID>::None);
    // the media that was selected by the user (propogated down)
    let (selected_media, set_selected_media) = create_signal(Option::<Metadata>::None);
    // the media that was most recently submitted by the user (propogated up)
    let (updated_media, set_updated_media) = create_signal(Option::<Metadata>::None);

    // update the selected media when the selected id changes
    create_effect(move |_| {
        if let Some(id) = selected_id() {
            set_selected_media(media.get().get(&id).cloned())
        }
    });

    // update the stored media metadata when updated_media is
    // written to
    create_effect(move |_| {
        if let Some(m) = updated_media() {
            if let Some(id) = selected_id.get_untracked() {
                set_media.update(|ms| {
                    ms.insert(id, m);
                })
            }
        }
    });

    provide_meta_context();
    provide_context(Context {
        media,
        selected_media,
        set_selected_id,
        set_updated_media,
    });

    // Sync Actions

    let sync_local_action = create_action(move |_: &()| {
        let media = media.get_untracked();
        async move {
            match client::sync_local(media).await {
                Err(e) => {
                    log!(e);
                    None
                }
                Ok(res) => Some(res),
            }
        }
    });

    let sync_remote_action = create_action(move |_: &()| {
        let media = media.get_untracked();
        async move {
            match client::sync_remote(media).await {
                Err(e) => {
                    log!(e);
                    None
                }
                Ok(res) => Some(res),
            }
        }
    });

    // Sync Effects

    fn sync(res: SyncResponse, media: &mut MediaCollection) {
        for (id, m) in res.missing {
            media.insert(id, m);
        }
        for id in res.unknown {
            media.remove(&id);
        }
    }

    create_effect(move |_| {
        sync_local_action.version().get();
        if let Some(Some(res)) = sync_local_action.value().get() {
            set_media.update(|media| sync(res, media))
        }
    });

    create_effect(move |_| {
        sync_remote_action.version().get();
        if let Some(Some(res)) = sync_remote_action.value().get() {
            set_media.update(|media| sync(res, media))
        }
    });

    // Trigger Sync Effects

    create_effect({
        let media_memo = create_memo(move |_| media.get());
        move |_| {
            media_memo.get();
            sync_local_action.dispatch(());
        }
    });

    let (sync_pending, set_sync_pending) = create_signal(false);
    create_effect(move |_| {
        if sync_local_action.pending().get() || sync_remote_action.pending().get() {
            set_sync_pending(true);
        } else {
            let UseTimeoutFnReturn { start, .. } = use_timeout_fn(
                move |_| {
                    set_sync_pending(false);
                },
                1000.0,
            );
            start(())
        }
    });

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>
        <Title text="Media Manager"/>
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Router base=std::option_env!("APP_BASE_PATH").unwrap_or("")>
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
                    <div id="heading-ctr">
                        <h1>"Media Manager"</h1>

                        {
                            #[cfg(feature = "demo")]
                            view! { <h2>"Demo Mode"</h2> }
                        }

                    </div>
                    // TODO replace with sync indicators
                    <div id="sync-buttons">
                        <div class="sync-button-ctr">
                            <p>"Local"</p>
                            <SyncButton action=sync_local_action pending=sync_pending/>
                        </div>
                        <div class="sync-button-ctr">
                            <p>"Remote"</p>
                            <SyncButton action=sync_remote_action pending=sync_pending/>
                        </div>
                    </div>
                </div>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route
                        path="/player"
                        view=move || {
                            view! { <Outlet/> }
                        }
                    >

                        <Route path=":id" view=MediaDashboard/>
                        <Route path="" view=MediaDashboardNoId/>

                    </Route>
                    // <Route path="/jobs" view=JobsDashboard/>
                    <Route path="/*" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
