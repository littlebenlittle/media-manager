use crate::Context;
use crate::{data::ID, log, base_path};
use leptos::*;
use leptos_router::*;

#[component]
pub fn MediaSelector() -> impl IntoView {
    let ctx = use_context::<Context>().unwrap();
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned().map(|s| ID::from(s)));
    let is_eq = move |i: &ID| id().map(|id| id == *i).unwrap_or(false);
    let sorted_media = move || {
        let mut list: Vec<_> = ctx.media.get().into_iter().collect();
        list.sort_by(|(_, a), (_, b)| a.shortname.to_lowercase().cmp(&b.shortname.to_lowercase()));
        list
    };
    view! {
        <div id="media-selector">
            {move || {
                let media = ctx.media.get();
                if media.len() == 0 {
                    view! {
                        <p>
                            "Looks like there isn't any media to show. Try clicking \
                                        the Remote sync button in the upper right."
                        </p>
                    }
                        .into_view()
                } else {
                    view! {}.into_view()
                }
            }}
            <ul>
                <For
                    each=sorted_media
                    key=move |(i, m)| {
                        use std::hash::{Hash, Hasher, DefaultHasher};
                        let mut s = DefaultHasher::new();
                        (i, m, is_eq(&i)).hash(&mut s);
                        s.finish()
                    }

                    children=move |(i, m)| {
                        view! {
                            <a href=base_path(&format!("/player/{}", i))>
                                <li class:selected=is_eq(&i)>
                                    <p>{m.shortname}</p>
                                    <br/>
                                    <p>{i.to_string()}</p>
                                </li>
                            </a>
                        }
                    }
                />
            </ul>
        </div>
    }
}
