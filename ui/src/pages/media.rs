use leptos::*;
use leptos_router::*;

use crate::data::Media;

#[component]
pub fn MediaSelector() -> impl IntoView {
    let media = use_context::<Resource<(), Media>>().unwrap();
    let query = use_query_map();
    let search = move || query().get("q").cloned().unwrap_or_default();
    view! {
        <div id="media-selector">
            <h3>"Media Selector"</h3>
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
                                                <li>{m.summary()}</li>
                                            </a>
                                        }
                                    }
                                />
                            }
                        })}

                </ul>
            </Transition>
        </div>
    }
}

#[component]
pub fn MediaEditor() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").unwrap().clone());
    let media = use_context::<Resource<(), Media>>().unwrap();
    let m = move || {
        media.get().map(|media| {
            media
                .get(&id())
                .map(|m| m.clone().into_view())
                .unwrap_or(view! { <p>"Media Not Found"</p> }.into_view())
        })
    };
    view! {
        <div id="media-editor">
            <h3>"Media Editor"</h3>
            {m}
        </div>
    }
}
