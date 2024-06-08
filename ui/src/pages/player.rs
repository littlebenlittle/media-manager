use crate::Context;
use crate::{client, data::ID, log};
use leptos::*;
use leptos_router::*;

// components
mod controls;
mod info;
mod selector;

use controls::Controls;
use info::Info;
use selector::MediaSelector;

#[component]
pub fn DashboardNoId() -> impl IntoView {
    view! {
        <div id="player">
            <MediaSelector/>
            <MediaUploadForm/>
        </div>
    }
}

#[component]
pub fn Dashboard() -> impl IntoView {
    let ctx = use_context::<Context>().unwrap();
    let params = use_params_map();
    create_effect(move |_| {
        ctx.set_selected_id
            .set(params.with(|p| p.get("id").cloned().map(|s| ID::from(s))))
    });

    let url = create_memo(move |_| ctx.selected_media.get().map(|m| m.url));

    view! {
        <div id="player">
            <MediaSelector/>
            <MediaUploadForm/>
            {move || match url.get() {
                None => view! {}.into_view(),
                Some(url) => {
                    view! {
                        <div id="video-view-area">
                            <Video url=url.clone()/>
                            {move || {
                                ctx.selected_media.get();
                                view! { <Info/> }
                            }}

                        </div>
                        <Controls url/>
                    }
                        .into_view()
                }
            }}

            {move || {
                if ctx.selected_media.get().is_none() {
                    view! { <p>"Not Found"</p> }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}

        </div>
    }
}

#[component]
pub fn MediaUploadForm() -> impl IntoView {
    let files = create_rw_signal(Option::<web_sys::FileList>::None);
    let upload_action = create_action(|files: &web_sys::FileList| {
        let files = files.clone();
        async move {
            let mut i = 0;
            while i < files.length() {
                let file = files.get(i).unwrap();
                if let Err(e) = client::upload(&file).await {
                    log!(e)
                }
                i += 1;
            }
        }
    });
    let upload = move |e: ev::SubmitEvent| {
        e.prevent_default();
        if let Some(files) = files.get() {
            upload_action.dispatch(files)
        }
    };
    let input_submit_ref = create_node_ref::<html::Input>();
    let onchange = move |e: ev::Event| {
        let tgt = event_target::<web_sys::HtmlInputElement>(&e);
        let file_list = tgt.files().unwrap();
        if file_list.length() == 0 {
            input_submit_ref
                .get()
                .unwrap()
                .toggle_attribute("disabled")
                .unwrap();
            files.set(None)
        } else {
            input_submit_ref
                .get()
                .unwrap()
                .remove_attribute("disabled")
                .unwrap();
            files.set(Some(file_list))
        }
    };
    view! {
        <form id="upload-form" method="POST" on:submit=upload>
            <input type="file" multiple accept="video/*" on:change=onchange/>
            <input type="submit" value="submit" node_ref=input_submit_ref/>
        </form>
    }
}

#[component]
fn Video(url: String) -> impl IntoView {
    view! {
        <div id="video">
            <video controls>
                <source src=url/>
            </video>
        </div>
    }
}
