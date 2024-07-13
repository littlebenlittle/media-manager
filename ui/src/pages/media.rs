use leptos::*;
use leptos_router::*;

use crate::{components::ClickToEdit, data::Media, log};

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
    let update_media = create_action(move |(field, value): &(&str, String)| {
        let (field, value) = (field.to_string(), value.clone());
        let id = id();
        async move {
            crate::client::update_media(id, &field, value).await;
            media.refetch();
        }
    });
    let url = create_memo(move |_| m().map(|m| m.url));
    // let url = move || m().map(|m| m.url);
    create_effect(move |_| log!("{:?}", url()));
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
                                <table>
                                    <tr>
                                        <td>"Title"</td>
                                        <td>
                                            <ClickToEdit
                                                value=m.title
                                                onset=move |value| update_media.dispatch(("title", value))
                                            />
                                        </td>
                                    </tr>
                                </table>
                            </div>
                        }
                    })
            }}

        </Transition>
    }
}
