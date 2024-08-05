use leptos::*;
use leptos_router::*;

use crate::{collection::ID, components::ClickToEdit, Collection, Data, Metadata};

#[component]
pub fn Selector() -> impl IntoView {
    let selected_id = move || use_params_map().with(|p| p.get("id").map(|s| ID::from(s.clone())));
    let store = use_context::<Signal<Collection<(Data, Metadata)>>>().unwrap();
    let filtered_metadata = move || {
        store.with(|store| {
            store
                .clone()
                .into_iter()
                .filter_map(move |(id, (_, m))| {
                    if m.matches(use_query_map().get()) {
                        Some((id.clone(), m.clone()))
                    } else {
                        None
                    }
                })
                .collect::<Collection<_>>()
        })
    };
    view! {
        <ul class="selector">
            <For
                each=filtered_metadata
                key=|(id, _)| id.clone()
                children=move |(id, meta)| {
                    let href = format!(
                        "http://localhost:8080/media/{}{}",
                        id,
                        use_query_map().get().to_query_string(),
                    );
                    let title = meta.title();
                    view! {
                        <a class:selected=Some(id) == selected_id() title=title.clone() href=href>
                            <li>{title}</li>
                        </a>
                    }
                }
            />

        </ul>
    }
}
