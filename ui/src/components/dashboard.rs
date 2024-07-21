use leptos::*;
use leptos_router::*;

use crate::{components::ClickToEdit, data::MediaItem, log};

#[cfg(web_sys_unstable_apis)]
use crate::components::CopyButton;

#[component]
pub fn Selector<F>(path: String, filter: F) -> impl IntoView
where
    F: Fn(String, &MediaItem) -> bool + Copy + 'static,
{
    let query = use_query_map();
    let search = move || query().get("q").cloned().unwrap_or_default();
    let media = use_context::<RwSignal<Vec<RwSignal<MediaItem>>>>().unwrap();
    view! {
        <Form method="GET" action="." class="search-form">
            <label>
                "Search:"
                <input type="search" name="q" value=search oninput="this.form.requestSubmit()"/>
            </label>
        </Form>
        <ul>
            <For
                each=move || {
                    media
                        .get()
                        .into_iter()
                        .filter(|m| filter(search(), &&m.get()))
                        .collect::<Vec<_>>()
                }

                key=|item| item.get().id
                children=move |item| {
                    let title = move || item.get().title;
                    view! {
                        <a
                            title=title
                            href={
                                let path = path.clone();
                                move || crate::path(
                                    &format!(
                                        "{}/{}{}",
                                        path,
                                        item.get_untracked().id,
                                        query().to_query_string(),
                                    ),
                                )
                            }
                        >

                            <li>{title}</li>
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
        <form id="upload-form" on:submit=onsubmit>
            <input type="file" multiple on:change=onchange/>
            <input node_ref=file_input class="submit" type="submit" value="Upload"/>
        </form>
    }
}

#[component]
pub fn Editor<F, R, IV>(filter: F, render: R) -> impl IntoView
where
    F: Fn(&&MediaItem) -> bool + Copy + 'static,
    R: Fn(MediaItem) -> IV + Copy + 'static,
    IV: IntoView,
{
    // let media = use_context::<Resource<(), Vec<MediaItem>>>().unwrap();
    let media = use_context::<RwSignal<Vec<RwSignal<MediaItem>>>>().unwrap();
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").unwrap().clone());
    let item = move || {
        media.get().iter().find_map(|item| {
            let item = item.get_untracked();
            if item.id == id() && filter(&&item) {
                Some(item)
            } else {
                None
            }
        })
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
    let update =
        use_context::<Action<(String, String, String), Option<(String, String, String)>>>()
            .unwrap();
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
                <td>
                    <span class="media-url">
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
