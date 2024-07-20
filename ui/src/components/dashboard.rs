use leptos::*;
use leptos_router::*;

use crate::{components::ClickToEdit, data::MediaItem};

#[cfg(web_sys_unstable_apis)]
use crate::components::CopyButton;

#[component]
pub fn Selector<F>(path: String, filter: F) -> impl IntoView
where
    F: Fn(String, &MediaItem) -> bool + Copy + 'static,
{
    let query = use_query_map();
    let search = move || query().get("q").cloned().unwrap_or_default();
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned());
    let media = use_context::<Resource<(), Vec<MediaItem>>>().unwrap();
    let items = move || {
        media.with(|m| {
            m.clone().map(|m| {
                m.into_iter()
                    .filter(|m| filter(search(), m))
                    .collect::<Vec<_>>()
            })
        })
    };
    view! {
        <Form method="GET" action="">
            <input type="search" name="q" value=search oninput="this.form.requestSubmit()"/>
        </Form>
        <Transition fallback=|| {
            view! { "Loading..." }
        }>

            {items()
                .map(|items| {
                    view! {
                        <ul>
                            <For
                                each=move || items.clone()
                                key=|item| item.id.clone()
                                children={
                                    let path = path.clone();
                                    move |item| {
                                        view! {
                                            <a
                                                title=item.title.clone()
                                                href={
                                                    let id = item.id.clone();
                                                    let path = path.clone();
                                                    move || crate::path(
                                                        &format!("{}/{}{}", path, id, query().to_query_string()),
                                                    )
                                                }
                                            >

                                                // TODO why is this class not reacing to changes in id?
                                                <li class:selected={
                                                    let iid = item.id.clone();
                                                    move || Some(iid.clone()) == id()
                                                }>{item.title.clone()}</li>
                                            </a>
                                        }
                                    }
                                }
                            />

                        </ul>
                    }
                })}

        </Transition>
    }
}

#[component]
pub fn Editor<F, R, IV>(filter: F, render: R) -> impl IntoView
where
    F: Fn(&&MediaItem) -> bool + Copy + 'static,
    R: Fn(MediaItem) -> IV + Copy + 'static,
    IV: IntoView,
{
    let media = use_context::<Resource<(), Vec<MediaItem>>>().unwrap();
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").unwrap().clone());
    let item = move || {
        media
            .get()
            .map(|items| {
                items
                    .iter()
                    .find(|item| item.id == id() && filter(item))
                    .cloned()
            })
            .flatten()
    };
    view! {
        <div class="view">
            <Transition fallback=|| {
                view! { <p>"Loading Video"</p> }
            }>{move || { item().map(move |item| render(item).into_view()) }}</Transition>
        </div>
        <Transition fallback=|| {
            view! { <p>"Loading Video"</p> }
        }>
            {move || {
                item()
                    .map(|item| {
                        view! {
                            <div class="detail">
                                <DetailTable item=item/>
                            </div>
                        }
                    })
            }}

        </Transition>
    }
}

#[allow(unexpected_cfgs)]
#[component]
fn DetailTable(item: MediaItem) -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").unwrap().clone());
    let update = use_context::<Action<(String, String, String), ()>>().unwrap();
    view! {
        <table>
            <tr>
                <td>"title"</td>
                <td>

                    {view! {
                        <ClickToEdit
                            value=item.title.clone()
                            onset=move |value| update.dispatch((id(), "title".to_string(), value))
                        />
                    }
                        .into_view()}

                </td>
            </tr>
            <tr>
                <td>"format"</td>
                <td>

                    {view! {
                        <ClickToEdit
                            value=item.format.clone()
                            onset=move |value| update.dispatch((id(), "format".to_string(), value))
                        />
                    }
                        .into_view()}

                </td>
            </tr>

            <tr>
                <td>"url"</td>
                <td class="media-url">
                    <a download=download_name(&item) href=item.url.clone()>
                        <button>"Download"</button>
                    </a>

                    {
                        #[cfg(web_sys_unstable_apis)]
                        view! {
                            <span>
                                <CopyButton value=item.url.clone()/>
                            </span>
                        }
                    }

                    <span class="url-text" title=item.url.clone()>
                        {item.url.clone()}
                    </span>
                </td>
            </tr>

        </table>
    }
}

fn download_name(item: &MediaItem) -> String {
    if let Some(pos) = item.title.rfind(".") {
        if &item.title[pos..] == &item.format {
            return item.title.clone();
        }
    }
    format!("{}.{}", item.title, item.format)
}
