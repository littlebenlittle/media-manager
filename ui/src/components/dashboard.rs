use leptos::*;
use leptos_router::*;

use crate::{components::ClickToEdit, data::Media};

#[cfg(web_sys_unstable_apis)]
use crate::components::CopyButton;

#[component]
pub fn Selector<T, F>(items: Resource<(), Vec<T>>, path: String, filter: F) -> impl IntoView
where
    T: Media + 'static,
    F: Fn(String, &T) -> bool + Copy + 'static,
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

                                key=|item| item.key()
                                children={
                                    let path = path.clone();
                                    move |item| {
                                        view! {
                                            <a
                                                title=item.title()
                                                href={
                                                    let key = item.key();
                                                    let path = path.clone();
                                                    move || crate::path(
                                                        &format!("{}/{}{}", path, key, query().to_query_string()),
                                                    )
                                                }
                                            >

                                                // TODO why is this class not reacing to changes in id?
                                                <li class:selected={
                                                    let key = item.key();
                                                    move || Some(key.clone()) == id()
                                                }>{item.title()}</li>
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
pub fn Editor<T, UF, FF>(items: Resource<(), Vec<T>>, update: UF, fields: FF) -> impl IntoView
where
    T: Media + 'static,
    UF: Fn(&str, &str, &str) + Copy + 'static,
    FF: Fn(&T) -> Vec<(String, String, bool)> + Copy + 'static,
{
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").unwrap().clone());
    let item = move || {
        items
            .get()
            .map(|items| items.iter().find(|item| item.key() == id()).cloned())
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
    T: Media,
    // get the fields of T: name, init value, editable (more? copyable? links?)
    FF: Fn(&T) -> Vec<(String, String, bool)>,
    UF: Fn(&str, &str, &str) + Copy + 'static,
{
    let fields = fields(&item);
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").unwrap().clone());
    view! {
        <div class="media-url">
            <a download=download_name(&item) href=item.url()>
                <button>"Download"</button>
            </a>

            {
                #[cfg(web_sys_unstable_apis)]
                view! {
                    <span>
                        <CopyButton value=item.url()/>
                    </span>
                }
            }

            <span class="url-text" title=item.url()>
                {item.url()}
            </span>
        </div>
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

fn download_name<T>(item: &T) -> String
where
    T: Media,
{
    let title = item.title();
    let format = item.format();
    if let Some(pos) = title.rfind(".") {
        if &title[pos..] == &format {
            return title.clone();
        }
    }
    format!("{}.{}", title, format)
}
