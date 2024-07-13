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
                <input type="search" name="q" value=search/>
                <input type="submit"/>
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
                                    children=|(_, m)| view! { <li>{m.summary()}</li> }
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
    let media = use_context::<RwSignal<Media>>().unwrap();
    let m = move || {
        media
            .get()
            .get(&id())
            .map(|m| m.clone().into_view())
            .unwrap_or(view! { <p>"Media Not Found"</p> }.into_view())
    };
    view! {
        <div id="media-editor">
            <h3>"Media Editor"</h3>
            {m}
        </div>
    }
}
