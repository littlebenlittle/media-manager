use crate::data::{Media, MediaLibrary};
use crate::{client, log};
use leptos::*;
use leptos_router::*;
use leptos_use::on_click_outside;
use serde_json::json;

#[component]
pub fn Player() -> impl IntoView {
    let upload_action = create_action(|files: &web_sys::FileList| {
        let files = files.clone();
        async move {
            let mut i = 0;
            let client = client::Client::default();
            while i < files.length() {
                let file = files.get(i).unwrap();
                if let Err(e) = client.upload(&file).await {
                    log!(e)
                }
                i += 1;
            }
        }
    });
    let files = create_rw_signal(Option::<web_sys::FileList>::None);
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
        <div id="player">
            <MediaSelector/>
            <form id="upload-form" method="POST" on:submit=upload>
                <input type="file" multiple accept="video/*" on:change=onchange/>
                <input type="submit" value="submit" node_ref=input_submit_ref/>
            </form>
            <Outlet/>
        </div>
    }
}

#[component]
pub fn MediaSelector() -> impl IntoView {
    let media_library = use_context::<RwSignal<MediaLibrary>>().unwrap();
    let init_media_library = use_context::<Resource<(), Option<MediaLibrary>>>().unwrap();
    view! {
        <div id="media-selector">
            {move || match init_media_library.get() {
                None => {
                    view! {
                        <p>{"Media Library Failed to Load"}</p>
                        <button on:click=move |_| init_media_library.refetch()>"Reload"</button>
                    }
                        .into_view()
                }
                Some(_) => {
                    view! {
                        <ul>
                            {media_library
                                .get()
                                .media()
                                .map(|media| {
                                    let media = media.get();
                                    view! {
                                        <A href=format!("{}", &media.id)>
                                            <li>{&media.shortname}</li>
                                        </A>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    }
                        .into_view()
                }
            }}

        </div>
    }
}

#[component]
pub fn VideoDashboard() -> impl IntoView {
    let media_library = use_context::<RwSignal<MediaLibrary>>().unwrap();
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap());
    view! {
        {move || {
            let id = id();
            match media_library.get().get_media(&id) {
                None => view! { <p>"Media not found: " {id}</p> }.into_view(),
                Some(media) => {
                    let url = create_memo(move |_| media.get().url());
                    let format = create_memo(move |_| media.get().format);
                    view! {
                        <Video url format/>
                        <Info media/>
                        <ControlPanel id url/>
                    }
                        .into_view()
                }
            }
        }}
    }
}

#[component]
fn Video(url: Memo<String>, format: Memo<String>) -> impl IntoView {
    view! {
        <div id="video">
            {move || {
                if ["ogg", "mp4", "webm"].contains(&format().as_str()) {
                    view! {
                        <video controls>
                            <source src=move || url.get()/>
                        </video>
                    }
                        .into_view()
                } else {
                    view! {
                        <div id="unsupported-video">
                            <p>{"Unsupported Video Format"}</p>
                        </div>
                    }
                        .into_view()
                }
            }}

        </div>
    }
}

#[component]
fn ControlPanel(id: String, url: Memo<String>) -> impl IntoView {
    view! {
        <div id="video-control-panel">
            <div id="video-download-button">
                <a href=move || url.get() download>
                    <input type="submit" value="Download"/>
                </a>
            </div>
            <ConvertForm id/>
        </div>
    }
}

#[component]
fn ConvertForm(id: String) -> impl IntoView {
    let convert = create_action(|req: &serde_json::Value| {
        let req = req.clone();
        async {
            if let Err(err) = client::Client::default().convert(req).await {
                log!(err)
            }
        }
    });
    let webm: NodeRef<html::Input> = create_node_ref();
    let ogg: NodeRef<html::Input> = create_node_ref();
    let mp4: NodeRef<html::Input> = create_node_ref();
    let hardsub: NodeRef<html::Input> = create_node_ref();
    let overwrite: NodeRef<html::Input> = create_node_ref();
    let onsubmit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let format = if webm().unwrap().checked() {
            "webm"
        } else if ogg().unwrap().checked() {
            "ogg"
        } else if mp4().unwrap().checked() {
            "mp4"
        } else {
            log!("Form error");
            return;
        };
        let req = json!({
            "id": id,
            "format": format,
            "hardsub": hardsub().unwrap().checked(),
            "overwrite": overwrite().unwrap().checked(),
        });
        convert.dispatch(req);
    };
    view! {
        <form id="media-convert-form" on:submit=onsubmit>
            <fieldset id="media-convert-format">
                <legend>"Format"</legend>
                {[(webm, "webm"), (ogg, "ogg"), (mp4, "mp4")]
                    .into_iter()
                    .map(|(node_ref, value)| {
                        view! {
                            <label>
                                <input
                                    type="radio"
                                    name="format"
                                    value=value
                                    node_ref=node_ref
                                    required
                                />
                                <span>{value}</span>
                            </label>
                        }
                    })
                    .collect_view()}
            </fieldset>
            <div id="media-convert-options">
                {[(hardsub, "hardsub"), (overwrite, "overwrite")]
                    .into_iter()
                    .map(|(node_ref, name)| {
                        view! {
                            <label>
                                <input type="checkbox" name=name node_ref=node_ref/>
                                <span>{name}</span>
                            </label>
                        }
                    })
                    .collect_view()}
            </div>
            <div id="media-convert-submit">
                <input type="submit" value="Convert"/>
            </div>
        </form>
    }
}

#[component]
fn Info(media: RwSignal<Media>) -> impl IntoView {
    let title = create_rw_signal(media.get_untracked().title);
    let format = create_rw_signal(media.get_untracked().format);
    let shortname = create_rw_signal(media.get_untracked().shortname);
    // here there should be effects
    create_effect(move |_| {
        media.update(|m| {
            m.title = title.get();
            m.format = format.get();
            m.shortname = shortname.get();
        });
    });
    let update_media = create_action(|media: &Media| {
        let media = media.clone();
        async move {
            if let Err(e) = client::Client::default().update_media(&media).await {
                log!(e)
            }
        }
    });
    create_effect(move |media_was| {
        let media = media.get();
        if let Some(media_was) = media_was {
            if media != media_was {
                update_media.dispatch(media.clone())
            }
        }
        media
    });
    view! {
        <div id="video-info">
            <ClickToEdit sig=title children=move || view! { <h3>{title.get()}</h3> }/>
            <table>
                <tr>
                    <td>"Shortname"</td>
                    <td>
                        <ClickToEdit
                            sig=shortname
                            children=move || view! { <span>{shortname.get()}</span> }
                        />
                    </td>
                </tr>
                <tr>
                    <td>"Format"</td>
                    <td>
                        <ClickToEdit
                            sig=format
                            children=move || view! { <span>{format.get()}</span> }
                        />
                    </td>
                </tr>
            </table>
        </div>
    }
}

#[component]
fn ClickToEdit<F, IV>(sig: RwSignal<String>, children: F) -> impl IntoView
where
    F: 'static + Fn() -> IV,
    IV: IntoView,
{
    let edit = create_rw_signal(false);
    let val = create_rw_signal(sig.get_untracked());
    let node = create_node_ref::<html::Input>();
    create_effect(move |edit_was| {
        let edit = edit.get();
        if edit {
            node.get().unwrap().focus().expect("input.focus");
        } else if edit_was.unwrap_or(false) {
            sig.set(val.get())
        }
        edit
    });
    let _ = on_click_outside(node, move |_| edit.set(false));
    view! {
        {move || {
            if edit.get() {
                view! {
                    <input
                        node_ref=node
                        type="text"
                        value=val.get_untracked()
                        on:input=move |e| {
                            val.set(event_target_value(&e));
                        }

                        on:keydown=move |e| {
                            if e.key() == "Enter" {
                                edit.set(false);
                            }
                        }
                    />
                }
                    .into_view()
            } else {
                view! { <div on:click=move |_| edit.set(true)>{children()}</div> }.into_view()
            }
        }}
    }
}

// #[component]
// fn InfoTitle(title: RwSignal<String>) -> impl IntoView {
//     let edit = create_rw_signal(false);
//     let target = helper(edit, "title".to_string(), title);
//     view! {
//         {move || {
//             if edit.get() {
//                 view! {
//                     <input
//                         node_ref=target
//                         type="text"
//                         name="title"
//                         value=move || title.get()

//                         on:input=move |e| {
//                             title.set(event_target_value(&e));
//                         }

//                         on:keydown=move |e| {
//                             if e.key() == "Enter" {
//                                 edit.set(false);
//                             }
//                         }
//                     />
//                 }
//                     .into_view()
//             } else {
//                 view! {
//                     <div on:click=move |_| edit.set(true)>
//                         <h3>{move || title.get()}</h3>
//                     </div>
//                 }
//                     .into_view()
//             }
//         }}
//     }
// }

// #[component]
// fn InfoShortname(shortname: RwSignal<String>) -> impl IntoView {
//     let edit = create_rw_signal(false);
//     let target = helper(edit, "shortname".to_string(), shortname);
//     view! {
//         {move || {
//             if edit.get() {
//                 view! {
//                     <input
//                         node_ref=target
//                         type="text"
//                         name="shortname"
//                         value=move || shortname.get()

//                         on:input=move |e| {
//                             shortname.set(event_target_value(&e));
//                         }

//                         on:keydown=move |e| {
//                             if e.key() == "Enter" {
//                                 edit.set(false);
//                             }
//                         }
//                     />
//                 }
//                     .into_view()
//             } else {
//                 view! { <span on:click=move |_| edit.set(true)>{move || shortname.get()}</span> }
//                     .into_view()
//             }
//         }}
//     }
// }

// fn helper(edit: RwSignal<bool>, path: String, val: RwSignal<String>) -> NodeRef<html::Input> {
//     let target = create_node_ref::<html::Input>();
//     let media_library = use_context::<RwSignal<MediaLibrary>>().unwrap();
//     let params = use_params_map();
//     let id = move || params.with(|params| params.get("id").cloned().unwrap());
//     let _use = leptos_use::on_click_outside(target, move |_| edit.set(false));
//     let _ef = create_effect(move |edit_was| {
//         let edit = edit.get();
//         if edit {
//             target.get().unwrap().focus().expect("input.focus");
//         } else if edit_was.unwrap_or(false) {
//             media_library.update(|lib| lib.update(&id(), &path, &val.get()))
//         }
//         edit
//     });
//     return target;
// }
