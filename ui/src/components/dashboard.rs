use leptos::*;
use leptos_router::*;

use crate::{collection::ID, components::ClickToEdit, Collection, Data, Metadata};

#[cfg(web_sys_unstable_apis)]
use crate::components::CopyButton;

#[component]
pub fn QueryForm() -> impl IntoView {
    let query = use_query_map();
    let search = move || query.get().get("q").cloned().unwrap_or_default();
    let media_type = move || query.get().get("t").cloned().unwrap_or_default();
    // TODO this should be derived from the metadata schema
    view! {
        <Form method="GET" action="." class="query-form">
            <label class="search">
                "Search:"
                <input type="search" name="q" value=search oninput="this.form.requestSubmit()"/>
            </label>
            <fieldset class="media-type">
                <legend>"Format:"</legend>
                <label>
                    "Video"
                    <input
                        type="radio"
                        name="t"
                        value="v"
                        checked=move || media_type() == "v"
                        oninput="this.form.requestSubmit()"
                    />
                </label>
                <label>
                    "Image"
                    <input
                        type="radio"
                        name="t"
                        value="i"
                        checked=move || media_type() == "i"
                        oninput="this.form.requestSubmit()"
                    />
                </label>
            </fieldset>
        </Form>
    }
}

#[component]
pub fn Selector<F>(filter: F) -> impl IntoView
where
    // query string and b64-encoded metadata
    F: Fn(ParamsMap, &Metadata) -> bool + Copy + 'static,
{
    let query = use_query_map();
    let metadata = use_context::<Signal<Collection<Metadata>>>().unwrap();
    let params = use_params_map();
    let qid = move || params.with(|p| p.get("id").map(|s| ID::from(s.clone())));
    view! {
        <ul class="selector">
            <For
                each=move || {
                    let mut metadata = metadata
                        .get()
                        .into_iter()
                        .filter(|(_, meta)| filter(query.get(), meta))
                        .collect::<Vec<_>>();
                    metadata.sort_by(|(_, a), (_, b)| a.title().cmp(&b.title()));
                    metadata
                }

                key=|(id, _)| id.clone()
                children=move |(id, meta)| {
                    let href = format!(
                        "http://localhost:8080/media/{}{}",
                        id,
                        query().to_query_string(),
                    );
                    let title = meta.title();
                    view! {
                        <a
                            class:selected=Some(id) == qid()
                            title=title.clone()
                            href=move || crate::path(
                                &format!("{}{}", href, query().to_query_string()),
                            )
                        >

                            <li>{title}</li>
                        </a>
                    }
                }
            />

        </ul>
    }
}

#[component]
pub fn UploadForm() -> impl IntoView {
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
pub fn MediaView() -> impl IntoView {
    let view = || {
        let data = use_context::<Signal<Collection<Data>>>().unwrap();
        let params = use_params_map();
        for (id, data) in data.get() {
            if id == params.with(|p| p.get("id").unwrap().clone()).into() {
                return Some(data.into_view());
            }
        }
        return None;
    };
    view! {
        <div class="view">
            <Transition fallback=|| {
                view! { <p>"Invalid Media ID"</p> }
            }>{view}</Transition>
        </div>
    }
}

#[allow(unexpected_cfgs)]
#[component]
pub fn Detail() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").map(|s| ID::from(s.clone())));
    let data = move || {
        id().map(|id| {
            use_context::<Signal<Collection<Data>>>()
                .unwrap()
                .with(|x| x.get(&id).cloned())
        })
        .flatten()
    };
    let metadata = move || {
        id().map(|id| {
            use_context::<Signal<Collection<Metadata>>>()
                .unwrap()
                .with(|x| x.get(&id).cloned())
        })
        .flatten()
    };
    let update = use_context::<fn(Box<dyn Fn(&mut Metadata)>)>().unwrap();
    view! {
        <div class="detail">
            <Transition fallback=|| {
                view! { <p>"Invalid Media ID"</p> }
            }>
                <table>
                    {move || {
                        metadata()
                            .map(|item| {
                                view! {
                                    <tr>
                                        <td>"title"</td>
                                        <td>
                                            <ClickToEdit
                                                value=item.title()
                                                onset=move |title| update(
                                                    Box::new(move |metadata| metadata.set_title(title.clone())),
                                                )
                                            />

                                        </td>
                                    </tr>
                                    <tr>
                                        <td>"format"</td>
                                        <td>
                                            <ClickToEdit
                                                value=item.format()
                                                onset=move |format| update(
                                                    Box::new(move |metadata| {
                                                        metadata.set_format(format.clone())
                                                    }),
                                                )
                                            />

                                        </td>
                                    </tr>
                                }
                            })
                    }}
                    {move || {
                        data()
                            .map(|data| {
                                view! {
                                    <tr>
                                        <td>"url"</td>
                                        <td>
                                            <span class="media-url">
                                                <a download=data.download_name() href=data.url()>
                                                    <button>"Download"</button>
                                                </a>

                                                {
                                                    #[cfg(web_sys_unstable_apis)]
                                                    view! {
                                                        <span>
                                                            <CopyButton value=data.url()/>
                                                        </span>
                                                    }
                                                }

                                                <span class="url-text" title=data.url()>
                                                    {data.url()}
                                                </span>
                                            </span>
                                        </td>
                                    </tr>
                                }
                            })
                    }}

                </table>

            </Transition>
        </div>
    }
}
