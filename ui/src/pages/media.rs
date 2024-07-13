use leptos::*;
use leptos_router::*;

use crate::{
    components::ClickToEdit,
    data::{Media, MediaItem},
    log,
};

#[component]
pub fn MediaSelector() -> impl IntoView {
    let media = use_context::<Resource<(), Media>>().unwrap();
    let query = use_query_map();
    let search = move || query().get("q").cloned().unwrap_or_default();
    view! {
        <Form method="GET" action="">
            <input type="search" name="q" value=search oninput="this.form.requestSubmit()"/>
        </Form>
        <Transition fallback=|| view! { <p>"Loading..."</p> }>
            <ul>

                {media
                    .get()
                    .map(|media| {
                        let filtered_media = move || {
                            media
                                .clone()
                                .into_iter()
                                .filter(move |(_, m)| {
                                    m.title.to_lowercase().contains(&search().to_lowercase())
                                })
                        };
                        view! {
                            <For
                                each=filtered_media
                                key=|(id, _)| id.clone()
                                children=move |(id, m)| {
                                    view! {
                                        <a href=move || crate::path(
                                            &format!("media/{}{}", id, query().to_query_string()),
                                        )>
                                            <li>{m.title}</li>
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
    #[cfg(web_sys_unstable_apis)]
    let leptos_use::UseClipboardReturn {
        is_supported,
        copy,
        copied,
        ..
    } = leptos_use::use_clipboard();
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
                <td class="media-url">
                    {#[cfg(web_sys_unstable_apis)]
                    {
                        let url = media.url.clone();
                        view! {
                            <Show when=is_supported fallback=|| view! { <span></span> }>
                                <button on:click={
                                    let copy = copy.clone();
                                    let url = url.clone();
                                    move |_| {
                                        copy(&url);
                                    }
                                }>
                                    <Show when=copied fallback=|| "Copy">
                                        "Copied!"
                                    </Show>
                                </button>
                            </Show>
                        }
                    }}
                    <span class="url-text">{media.url.clone()}</span>
                </td>
            </tr>
        </table>
    }
}
