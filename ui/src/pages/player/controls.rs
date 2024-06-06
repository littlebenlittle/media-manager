use crate::{client, log, store::ID};
use leptos::*;
use leptos_router::*;
use serde_json::json;


#[component]
pub fn Controls(url: String) -> impl IntoView {
    let params = use_params_map();
    let id = params
        .with_untracked(|p| p.get("id").cloned().map(|s| ID::from(s)))
        .unwrap();
    view! {
        <div id="video-control-panel">
            <DownloadButton url/>
            <ConvertForm id/>
        </div>
    }
}

#[component]
fn DownloadButton(url: String) -> impl IntoView {
    view! {
        <div id="video-download-button">
            <a href=url>
                <input type="submit" value="Download"/>
            </a>
        </div>
    }
}

#[component]
fn ConvertForm(id: ID) -> impl IntoView {
    let convert = create_action(|req: &serde_json::Value| {
        let req = req.clone();
        async {
            if let Err(err) = client::convert(req).await {
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
