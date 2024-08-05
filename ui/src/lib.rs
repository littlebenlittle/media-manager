#![allow(dead_code)]

mod client;
mod collection;
mod components;
mod pages;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use collection::{Collection, ID};
use components::{Detail, NotificationTray, QueryForm, Selector, UploadForm};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum StoreMsg {
    Exists { data: String, metadata: String },
    Forget { data: String },
    Errors(Vec<String>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum StoreReq {
    Exists { data: Data, metadata: Metadata },
    Forget { data: Data },
    Sync,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
struct Data(String);

impl Data {
    pub fn url(&self) -> String {
        todo!()
    }
    pub fn download_name(&self) -> String {
        todo!()
    }
}

impl IntoView for Data {
    fn into_view(self) -> View {
        view! { <p>"Data"</p> }.into_view()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
struct Metadata(String);

impl Metadata {
    pub fn title(&self) -> String {
        todo!()
    }
    pub fn with_title(&self, title: String) -> Self {
        todo!()
    }
    pub fn set_title(&mut self, title: String) {
        todo!()
    }
    pub fn format(&self) -> String {
        todo!()
    }
    pub fn with_format(&self, format: String) -> Self {
        todo!()
    }
    pub fn set_format(&mut self, format: String) {
        todo!()
    }
    pub fn matches(&self, query: ParamsMap) -> bool {
        todo!()
    }
}

#[component]
pub fn App() -> impl IntoView {
    // provide_context(data);
    // provide_context(metadata);
    // provide_context(Box::new(|id: ID, update: Box<dyn Fn(&mut Metadata)>| media));
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
                // <NotificationTray message=move || {
                // match new_media.get() {
                // Some(metadata) => {
                // Some(
                // view! { <a href=metadata.local_href()>{metadata.title()}</a> }
                // .into_view(),
                // )
                // }
                // _ => None,
                // }
                // }/>
                <Routes base=option_env!("APP_BASE_PATH").unwrap_or_default().to_owned()>
                    <Route path="/" view=pages::Home/>
                    <Route
                        path="library"
                        view=|| {
                            view! {
                                <div class="dashboard">
                                    <Outlet/>
                                </div>
                            }
                        }
                    >

                        <Route
                            path=":id"
                            view=move || {
                                view! {
                                    <QueryForm/>
                                    <Selector/>
                                    <UploadForm/>
                                    <Transition fallback=|| {
                                        view! { <p>"No Media Selected"</p> }
                                    }>
                                        {use_params_map()
                                            .with(|p| {
                                                p.get("id")
                                                    .map(|s| {
                                                        view! { <Dashboard id=ID::from(s.clone())/> }
                                                    })
                                            })}

                                    </Transition>
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

#[component]
pub fn Dashboard(id: ID) -> impl IntoView {
    let store = use_context::<Signal<Collection<(Data, Metadata)>>>().unwrap();
    let x = move || {
        store.with(|store| {
            store
                .get(&id)
                .map(|(d, m)| (id.clone(), d.clone(), m.clone()))
        })
    };
    view! {
        <Transition fallback=|| {
            view! { "Invalid Media ID" }
        }>
            {x()
                .map(|(id, data, metadata)| {
                    view! {
                        <div class="view">{data.clone().into_view()}</div>
                        <Detail
                            data=data
                            metadata=metadata
                            update=move |metadata| {
                                let id = id.clone();
                                spawn_local(async move {
                                    log!("set metadata: {} {:?}", id, metadata);
                                })
                            }
                        />
                    }
                })}

        </Transition>
    }
}
