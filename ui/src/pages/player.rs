use crate::components::Loading;
use crate::data::{Media, MediaLibrary};
use crate::{log, doc};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Player() -> impl IntoView {
    view! {
        <div id="player">
            <MediaSelector/>
            <Outlet/>
        </div>
    }
}

#[component]
pub fn MediaSelector() -> impl IntoView {
    let media_library = use_context::<Resource<(), Option<MediaLibrary>>>().unwrap();
    view! {
        <div id="media-selector">
            {move || match media_library.get() {
                None => view! { <Loading what="Media Library".to_string()/> }.into_view(),
                Some(None) => view! { <p>"Media List failed to load"</p> }.into_view(),
                Some(Some(lib)) => {
                    view! {
                        <ul>
                            {lib
                                .media
                                .into_iter()
                                .map(|media| {
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
    let media_library = use_context::<Resource<(), Option<MediaLibrary>>>().unwrap();
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap());
    view! {
        {move || match media_library.get() {
            None => view! { <Loading what="Media Library".to_owned()/> }.into_view(),
            Some(None) => {
                view! {
                    <div>
                        <p>"Media library failed to load"</p>
                        <button on:click=move |_| media_library.refetch()>"Try Again"</button>
                    </div>
                }
                    .into_view()
            }
            Some(Some(lib)) => {
                match lib.get_media(id()) {
                    None => view! { <p>"Media not found: " {id()}</p> }.into_view(),
                    Some(media) => view! { <Video media/> },
                }
            }
        }}
    }
}

#[component]
fn Video(media: Media) -> impl IntoView {
    let url = media.url();
    view! {
        {if ["ogg", "mp4", "webm"].contains(&media.filetype.as_str()) {
            view! {
                <div id="video">
                    <video controls>
                        <source src=&url/>
                    </video>
                    <Info media=media.clone()/>
                </div>
            }
                .into_view()
        } else {
            view! {}.into_view()
        }}

        <ControlPanel media=media.clone()/>
    }
}

#[component]
fn ControlPanel(media: Media) -> impl IntoView {
    view! {
        <div id="video-control-panel">
            <div id="video-download-button">
                <a href=media.url() download>
                    <input type="submit" value="Download"/>
                </a>
            </div>
            <ConvertForm/>
        </div>
    }
}

#[component]
fn ConvertForm() -> impl IntoView {
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
        let form = doc!({
            "format": format,
            "hardsub": hardsub().unwrap().checked(),
            "overwrite": overwrite().unwrap().checked(),
        });
        log!(form.emit_pretty());
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
fn Info(media: Media) -> impl IntoView {
    let title = create_rw_signal(media.title.clone());
    let shortname = create_rw_signal(media.shortname.clone());
    view! {
        <div id="video-info">
            <InfoTitle title/>
            <p>"Filetype: " {media.filetype}</p>
            <p>"Shortname: " <InfoShortname shortname/></p>
        </div>
    }
}

#[component]
fn InfoTitle(title: RwSignal<String>) -> impl IntoView {
    let edit = create_rw_signal(false);
    let target = helper(edit, "title".to_string(), title);
    view! {
        {move || {
            if edit.get() {
                view! {
                    <input
                        node_ref=target
                        type="text"
                        name="title"
                        value=move || title.get()

                        on:input=move |e| {
                            title.set(event_target_value(&e));
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
                view! {
                    <div on:click=move |_| edit.set(true)>
                        <h3>{move || title.get()}</h3>
                    </div>
                }
                    .into_view()
            }
        }}
    }
}

#[component]
fn InfoShortname(shortname: RwSignal<String>) -> impl IntoView {
    let edit = create_rw_signal(false);
    let target = helper(edit, "shortname".to_string(), shortname);
    view! {
        {move || {
            if edit.get() {
                view! {
                    <input
                        node_ref=target
                        type="text"
                        name="shortname"
                        value=move || shortname.get()

                        on:input=move |e| {
                            shortname.set(event_target_value(&e));
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
                view! {
                        <span on:click=move |_| edit.set(true)>{move || shortname.get()}</span>
                }
                    .into_view()
            }
        }}
    }
}

fn helper(edit: RwSignal<bool>, path: String, val: RwSignal<String>) -> NodeRef<html::Input> {
    let target = create_node_ref::<html::Input>();
    let media_library = use_context::<Resource<(), Option<MediaLibrary>>>().unwrap();
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap());
    let _use = leptos_use::on_click_outside(target, move |_| edit.set(false));
    let _ef = create_effect(move |edit_was| {
        let edit = edit.get();
        if edit {
            target.get().unwrap().focus().expect("input.focus");
        } else if edit_was.unwrap_or(false) {
            media_library.update(|lib| {
                lib.as_mut()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .update(&id(), &path, &val.get())
            })
        }
        edit
    });
    return target;
}
