use crate::{components::LoremIpsum, log, client};
use leptos::*;
use leptos_use::{
    core::ConnectionReadyState, use_document, use_event_source_with_options, utils::JsonCodec,
    UseEventSourceOptions, UseEventSourceReturn,
};
use serde::{Deserialize, Serialize};
use web_sys::{Document, Node};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Job {
    id: String,
    name: String,
    created: String,
    completed: Option<String>,
    progress: Option<u8>,
}

#[component]
pub fn JobsDashboard() -> impl IntoView {
    let jobs = create_rw_signal(Vec::<Job>::new());
    let UseEventSourceReturn { data, .. } = use_event_source_with_options::<Job, JsonCodec>(
        format!("{}/api/jobs", client::origin()).as_str(),
        UseEventSourceOptions::default(),
    );
    create_effect(move |_| {
        if let Some(job) = data.get() {
            jobs.update(|jobs| jobs.push(job));
        }
    });
    view! {
        <div id="jobs-dashboard">
            <h2>"Jobs"</h2>
            {move || {
                if jobs.get().len() == 0 {
                    view! { <p>"Waiting for job data..."</p> }.into_view()
                } else {
                    view! {
                        <ul id="jobs-list">
                            {move || {
                                jobs.get()
                                    .into_iter()
                                    .map(|job| {
                                        view! {
                                            <li>
                                                <JobInfo job/>
                                            </li>
                                        }
                                    })
                                    .collect_view()
                            }}

                        </ul>
                    }
                        .into_view()
                }
            }}

        </div>
    }
}

#[component]
fn JobInfo(job: Job) -> impl IntoView {
    let (show_logs, set_show_logs) = create_signal(false);
    let toggle_show_logs = move |_| set_show_logs.set(!show_logs.get());
    view! {
        <h3>{job.name}</h3>
        <p>"Started: " <time datetime="2024-01-01T16:00:00">"10:17am"</time></p>
        <p>"Completed: -"</p>
        <progress max="100" value="30">
            "30%"
        </progress>
        <button on:click=toggle_show_logs>
            {move || {
                if show_logs.get() {
                    view! { "Hide Logs" }.into_view()
                } else {
                    view! { "Show Logs" }.into_view()
                }
            }}

        </button>
        <div class="collapsible" class:show=move || show_logs.get()>
            <LogMessages job_id="123abc".to_owned() show_logs/>
        </div>
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LogMessage(String);

impl Into<Node> for LogMessage {
    fn into(self) -> Node {
        let pre = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("pre")
            .unwrap();
        pre.set_inner_html(&self.0);
        return pre.into();
    }
}

#[component]
pub fn LogMessages(job_id: String, show_logs: ReadSignal<bool>) -> impl IntoView {
    let UseEventSourceReturn {
        data,
        ready_state,
        open,
        close,
        ..
    } = use_event_source_with_options::<LogMessage, JsonCodec>(
        format!("{}/api/job/{}/logs", client::origin(), job_id).as_str(),
        UseEventSourceOptions::default().immediate(false),
    );
    let node_ref = create_node_ref::<html::Pre>();
    create_effect(move |_| {
        if let Some(message) = data.get() {
            let node = message.into();
            if let Err(e) = node_ref.get().unwrap().append_child(&node) {
                log!(e.as_string().unwrap())
            }
        }
    });
    create_effect(move |_| {
        if show_logs.get() {
            open();
        } else {
            match ready_state.get() {
                ConnectionReadyState::Open => close(),
                _ => {}
            }
        }
    });
    view! {
        {move || match ready_state.get() {
            ConnectionReadyState::Connecting => view! { <pre>"Connecting..."</pre> }.into_view(),
            ConnectionReadyState::Closing => {
                view! { <pre>"Connection Closing..."</pre> }.into_view()
            }
            ConnectionReadyState::Closed => view! { <pre>"Connection Closed"</pre> }.into_view(),
            ConnectionReadyState::Open => node_ref.into_view(),
        }}
    }
}
