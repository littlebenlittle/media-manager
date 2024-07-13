use leptos::*;
use leptos_router::*;

use crate::{
    components::ClickToEdit,
    data::{Media, MediaItem},
    log,
};

#[cfg(web_sys_unstable_apis)]
use crate::components::CopyButton;

#[component]
pub fn MediaSelector() -> impl IntoView {
    let media = use_context::<Resource<(), Media>>().unwrap();
    let query = use_query_map();
    let search = move || query().get("q").cloned().unwrap_or_default();
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned());
    let filter_sort = move |media: Media| {
        let mut media = media
            .clone()
            .into_iter()
            .filter(move |(_, m)| m.title.to_lowercase().contains(&search().to_lowercase()))
            .collect::<Vec<_>>();
        media.sort_by(|(_, a), (_, b)| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
        media
    };
    view! {
        <Form method="GET" action="">
            <input type="search" name="q" value=search oninput="this.form.requestSubmit()"/>
        </Form>
        <Transition fallback=|| view! { <p>"Loading..."</p> }>
            <ul>

                {media
                    .get()
                    .map(|media| {
                        view! {
                            <For
                                each=move || filter_sort(media.clone())
                                key=|(mid, _)| mid.clone()
                                children=move |(mid, m)| {
                                    view! {
                                        <a
                                            title=m.title.clone()
                                            href={
                                                let mid = mid.clone();
                                                move || crate::path(
                                                    &format!("media/{}{}", mid, query().to_query_string()),
                                                )
                                            }
                                        >

                                            // TODO why is this class not reacing to changes in id?
                                            <li class:selected=move || {
                                                Some(mid.clone()) == id()
                                            }>{m.title}</li>
                                        </a>
                                    }
                                }
                            />
                        }
                    })}

            </ul>
        </Transition>
    }
}

#[component]
pub fn MediaEditor() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").unwrap().clone());
    let media = use_context::<Resource<(), Media>>().unwrap();
    let m = move || media.get().map(|media| media.get(&id()).cloned()).flatten();
    let update_media = create_action(move |(field, value): &(String, String)| {
        let (field, value) = (field.clone(), value.clone());
        let id = id();
        async move {
            crate::client::update_media(id, &field, &value).await;
            media.refetch();
        }
    });
    let url = create_memo(move |_| m().map(|m| m.url));
    view! {
        <div class="video-ctr">
            <Transition fallback=|| {
                view! { <p>"Loading Media"</p> }
            }>
                {move || {
                    url()
                        .map(|url| {
                            view! { <video controls src=url></video> }
                        })
                }}

            </Transition>
        </div>
        <Transition fallback=|| {
            view! { <p>"Loading Media"</p> }
        }>
            {move || {
                m()
                    .map(|m| {
                        view! {
                            <div id="detail">
                                <DetailTable
                                    media=m
                                    update_media=move |field, value| {
                                        update_media.dispatch((field, value))
                                    }
                                />

                            </div>
                        }
                    })
            }}

        </Transition>
    }
}

#[allow(unexpected_cfgs)]
#[component]
fn DetailTable<Cb>(media: MediaItem, update_media: Cb) -> impl IntoView
where
    Cb: 'static + Copy + Fn(String, String),
{
    view! {
        <table>
            <tr>
                <td>"Title"</td>
                <td>
                    <ClickToEdit
                        value=media.title
                        onset=move |value| update_media("title".to_string(), value)
                    />
                </td>
            </tr>
            <tr>
                <td>"Format"</td>
                <td>
                    <ClickToEdit
                        value=media.format
                        onset=move |value| update_media("format".to_string(), value)
                    />
                </td>
            </tr>
            <tr>
                <td>"URL"</td>
                <td>
                    <span class="media-url">

                        <a download href=media.url.clone()>
                            <button>"Download"</button>
                        </a>

                        {
                            #[cfg(web_sys_unstable_apis)]
                            view! {
                                <span>
                                    <CopyButton value=media.url.clone()/>
                                </span>
                            }
                        }

                        <span class="url-text" title=media.url.clone()>
                            {media.url.clone()}
                        </span>
                    </span>
                </td>
            </tr>
        </table>
    }
}
