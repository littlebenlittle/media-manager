use leptos::*;
use leptos_router::*;

use crate::{Collection, Data};

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
