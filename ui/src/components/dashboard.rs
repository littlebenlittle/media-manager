use std::collections::HashMap;

use leptos::*;
use leptos_router::*;

use crate::{
    collection::{MediaCollection, MediaEvent},
    components::ClickToEdit,
    data::MediaItem,
    log,
    transport::{ReqSSETransport, Transport},
};

#[cfg(web_sys_unstable_apis)]
use crate::components::CopyButton;

#[component]
pub fn Selector<F>(path: String, filter: F) -> impl IntoView
where
    F: Fn(String, &MediaItem) -> bool + Copy + 'static,
{
    let query = use_query_map();
    let search = move || query().get("q").cloned().unwrap_or_default();
    let media = use_context::<Signal<MediaCollection>>().unwrap();
    view! {
        <Form method="GET" action="." class="search">
            <label>
                "Search:"
                <input type="search" name="q" value=search oninput="this.form.requestSubmit()"/>
            </label>
        </Form>
        <ul class="selector">
            <For
                each=move || {
                    let mut media = media
                        .get()
                        .into_iter()
                        .filter(|(_, m)| filter(search(), &&m))
                        .collect::<Vec<_>>();
                    media.sort_by(|(_, a), (_, b)| a.title.cmp(&b.title));
                    media
                }

                key=|(id, _)| id.clone()
                children=move |(id, item)| {
                    view! {
                        <a
                            title=item.title.clone()
                            href={
                                let path = path.clone();
                                move || crate::path(
                                    &format!("{}/{}{}", path, id, query().to_query_string()),
                                )
                            }
                        >

                            <li>{item.title}</li>
                        </a>
                    }
                }
            />

        </ul>
        <UploadForm/>
    }
}

#[component]
fn UploadForm() -> impl IntoView {
    let file_input = create_node_ref::<html::Input>();
    let upload = create_action(|file: &web_sys::File| {
        let file = file.clone();
        async move { crate::client::upload_file(file).await }
    });
    let files = create_rw_signal(Option::<web_sys::FileList>::None);
    let onchange = move |e: ev::Event| {
        let tgt = event_target::<web_sys::HtmlInputElement>(&e);
        let file_list = tgt.files().unwrap();
        files.set(Some(file_list.into()));
    };
    let onsubmit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if let Some(files) = files.get() {
            for i in 0..files.length() {
                upload.dispatch(files.get(i).unwrap().clone())
            }
        }
    };
    view! {
        <form class="upload" on:submit=onsubmit>
            <input type="file" multiple on:change=onchange/>
            <input node_ref=file_input class="submit" type="submit" value="Upload"/>
        </form>
    }
}

#[component]
pub fn Editor<R, IV>(render: R) -> impl IntoView
where
    R: Fn(String) -> IV + Copy + 'static,
    IV: IntoView,
{
    let media = use_context::<Signal<MediaCollection>>().unwrap();
    let params = use_params_map();
    let item = move || {
        let id = params.with(|p| p.get("id").unwrap().clone());
        media.with(|m| m.get(&id.into()).cloned())
    };
    let url = create_memo(move |_| item().map(|i| i.url));
    view! {
        <div class="view">
            <Transition fallback=|| {
                view! { <p>"Loading Video"</p> }
            }>{move || { url().map(move |url| render(url).into_view()) }}</Transition>
        </div>
        <div class="detail">
            <Transition fallback=|| {
                view! { <p>"Loading Video"</p> }
            }>
                {move || {
                    item()
                        .map(|item| {
                            view! { <DetailTable item=item/> }
                        })
                }}

            </Transition>
        </div>
    }
}

#[allow(unexpected_cfgs)]
#[component]
fn DetailTable(item: MediaItem) -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").unwrap().clone().into());
    let update = use_context::<WriteSignal<MediaEvent>>().unwrap();
    view! {
        <table>
            <tr>
                <td>"title"</td>
                <td>

                    {view! {
                        <ClickToEdit
                            value=item.title.clone()
                            onset=move |value| {
                                update
                                    .set(MediaEvent::Update {
                                        id: id(),
                                        field: "title".to_string(),
                                        value,
                                    })
                            }
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
                            onset=move |value| {
                                update
                                    .set(MediaEvent::Update {
                                        id: id(),
                                        field: "format".to_string(),
                                        value,
                                    })
                            }
                        />
                    }
                        .into_view()}

                </td>
            </tr>

            <tr>
                <td>"url"</td>
                <td>
                    <span class="media-url">
                        <a download=item.download_name() href=item.url.clone()>
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
                    </span>
                </td>
            </tr>

        </table>
    }
}
