use leptos::*;
use leptos_router::*;

use crate::components::ClickToEdit;
use crate::data::{Image, Images};

#[cfg(web_sys_unstable_apis)]
use crate::components::CopyButton;

#[component]
pub fn ImageSelector() -> impl IntoView {
    let images = use_context::<Resource<(), Images>>().unwrap();
    let query = use_query_map();
    let search = move || query().get("q").cloned().unwrap_or_default();
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned());
    let filter_sort = move |images: Images| {
        let mut images = images
            .clone()
            .into_iter()
            .filter(move |(_, m)| m.title.to_lowercase().contains(&search().to_lowercase()))
            .collect::<Vec<_>>();
        images.sort_by(|(_, a), (_, b)| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
        images
    };
    view! {
        <Form method="GET" action="">
            <input type="search" name="q" value=search oninput="this.form.requestSubmit()"/>
        </Form>
        <Transition fallback=|| view! { <p>"Loading..."</p> }>
            <ul>

                {images
                    .get()
                    .map(|images| {
                        view! {
                            <For
                                each=move || filter_sort(images.clone())
                                key=|(mid, _)| mid.clone()
                                children=move |(mid, m)| {
                                    view! {
                                        <a
                                            title=m.title.clone()
                                            href={
                                                let mid = mid.clone();
                                                move || crate::path(
                                                    &format!("images/{}{}", mid, query().to_query_string()),
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
pub fn ImageEditor() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").unwrap().clone());
    let images = use_context::<Resource<(), Images>>().unwrap();
    let i = move || {
        images
            .get()
            .map(|images| images.get(&id()).cloned())
            .flatten()
    };
    let update_image = create_action(move |(field, value): &(String, String)| {
        let (field, value) = (field.clone(), value.clone());
        let id = id();
        async move {
            crate::client::update_image(id, &field, &value).await;
            images.refetch();
        }
    });
    let url = create_memo(move |_| i().map(|m| m.url));
    view! {
        <div class="view">
            <Transition fallback=|| {
                view! { <p>"Loading Image"</p> }
            }>
                {move || {
                    url()
                        .map(|url| {
                            view! { <img src=url/> }
                        })
                }}

            </Transition>
        </div>
        <Transition fallback=|| {
            view! { <p>"Loading Video"</p> }
        }>
            {move || {
                i()
                    .map(|i| {
                        view! {
                            <div id="detail">
                                <DetailTable
                                    image=i
                                    update_image=move |field, value| {
                                        update_image.dispatch((field, value))
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
fn DetailTable<Cb>(image: Image, update_image: Cb) -> impl IntoView
where
    Cb: 'static + Copy + Fn(String, String),
{
    let download_name = {
        if let Some(pos) = image.title.rfind(".") {
            if &image.title[pos + 1..] == image.format {
                image.title.clone()
            } else {
                image.title.clone() + "." + &image.format
            }
        } else {
            image.title.clone() + "." + &image.format
        }
    };
    view! {
        <table>
            <tr>
                <td>"Title"</td>
                <td>
                    <ClickToEdit
                        value=image.title
                        onset=move |value| update_image("title".to_string(), value)
                    />
                </td>
            </tr>
            <tr>
                <td>"Format"</td>
                <td>
                    <ClickToEdit
                        value=image.format
                        onset=move |value| update_image("format".to_string(), value)
                    />
                </td>
            </tr>
            <tr>
                <td>"URL"</td>
                <td>
                    <span class="url">

                        <a download=download_name href=image.url.clone()>
                            <button>"Download"</button>
                        </a>

                        {
                            #[cfg(web_sys_unstable_apis)]
                            view! {
                                <span>
                                    <CopyButton value=image.url.clone()/>
                                </span>
                            }
                        }

                        <span class="url-text" title=image.url.clone()>
                            {image.url.clone()}
                        </span>
                    </span>
                </td>
            </tr>
        </table>
    }
}
