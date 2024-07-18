use leptos::*;
use leptos_router::*;
use std::{cmp::Ordering, collections::HashMap, hash::Hash};

use crate::{
    components::ClickToEdit,
    data::{Video, Videos},
    log,
};

#[cfg(web_sys_unstable_apis)]
use crate::components::CopyButton;

// TODO There should be a trait for `T` to implement to avoid all these
// closure arguments. For now leaving them explicit because it's easier
// to reason about locatlly.

#[component]
pub fn Selector<T, K, F, KF, TF>(
    items: Resource<(), Vec<T>>,
    path: String,
    filter: F,
    key: KF,
    title: TF,
) -> impl IntoView
where
    T: Clone + 'static,
    K: Eq + Hash + ToString + 'static,
    KF: Fn(&T) -> K + Copy + 'static,
    F: Fn(String, &T) -> bool + Copy + 'static,
    TF: Fn(&T) -> String + Copy + 'static,
{
    let query = use_query_map();
    let search = move || query().get("q").cloned().unwrap_or_default();
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned());
    view! {
        <Form method="GET" action="">
            <input type="search" name="q" value=search oninput="this.form.requestSubmit()"/>
        </Form>
        <Transition fallback=|| view! { <p>"Loading..."</p> }>
            <ul>

                {items()
                    .map(|items| {
                        view! {
                            <For
                                each=move || {
                                    items.clone().into_iter().filter(move |t| filter(search(), t))
                                }

                                key=key
                                children={
                                    let path = path.clone();
                                    move |t| {
                                        let key = key(&t).to_string();
                                        let title = title(&t);
                                        view! {
                                            <a
                                                title=title.clone()
                                                href={
                                                    let key = key.clone();
                                                    let path = path.clone();
                                                    move || crate::path(
                                                        &format!("{}/{}{}", path, key, query().to_query_string()),
                                                    )
                                                }
                                            >

                                                // TODO why is this class not reacing to changes in id?
                                                <li class:selected={
                                                    let key = key.clone();
                                                    move || Some(key.clone()) == id()
                                                }>{title}</li>
                                            </a>
                                        }
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
pub fn Editor<T, K, KF, UF, FF>(
    items: Resource<(), Vec<T>>,
    key: KF,
    update: UF,
    fields: FF,
) -> impl IntoView
where
    T: Clone + IntoView + 'static,
    K: Eq + Hash + ToString + 'static,
    KF: Fn(&T) -> K + Copy + 'static,
    UF: Fn(&str, &str, &str) + Copy + 'static,
    FF: Fn(&T) -> Vec<(String, String, bool)> + Copy + 'static,
{
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").unwrap().clone());
    let item = move || {
        items
            .get()
            .map(|items| items.iter().find(|t| key(t).to_string() == id()).cloned())
            .flatten()
    };
    view! {
        <div class="view">
            <Transition fallback=|| {
                view! { <p>"Loading Video"</p> }
            }>{move || { item().map(|item| item.into_view()) }}</Transition>
        </div>
        <Transition fallback=|| {
            view! { <p>"Loading Video"</p> }
        }>
            {move || {
                item()
                    .map(|item| {
                        view! {
                            <div class="detail">
                                <DetailTable item=item update=update fields=fields/>
                            </div>
                        }
                    })
            }}

        </Transition>
    }
}

#[allow(unexpected_cfgs)]
#[component]
fn DetailTable<T, UF, FF>(item: T, update: UF, fields: FF) -> impl IntoView
where
    // get the fields of T: name, init value, editable (more? copyable? links?)
    FF: Fn(&T) -> Vec<(String, String, bool)>,
    UF: Fn(&str, &str, &str) + Copy + 'static,
{
    let fields = fields(&item);
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").unwrap().clone());
    view! {
        <table>
            <For
                each=move || fields.clone()
                key=|(field, _, _)| field.clone()
                children=move |(field, init_val, edit)| {
                    view! {
                        <tr>
                            <td>{field.clone()}</td>
                            <td>
                                {if edit {
                                    let field = field.clone();
                                    view! {
                                        <ClickToEdit
                                            value=init_val
                                            onset=move |value| update(&id(), &field, &value)
                                        />
                                    }
                                        .into_view()
                                } else {
                                    view! { <span>{init_val}</span> }.into_view()
                                }}

                            </td>
                        </tr>
                    }
                }
            />

        </table>
    }
}

fn download_name<T, TF, FF>(item: &T, title: TF, format: FF) -> String
where
    TF: Fn(&T) -> String,
    FF: Fn(&T) -> String,
{
    let title = title(item);
    let format = format(item);
    if let Some(pos) = title.rfind(".") {
        if &title[pos..] == &format {
            return title.clone();
        }
    }
    format!("{}.{}", title, format)
}
