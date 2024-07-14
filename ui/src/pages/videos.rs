use leptos::*;
use leptos_router::*;

use crate::{
    components::ClickToEdit,
    data::{Video, Videos},
    log,
};

#[cfg(web_sys_unstable_apis)]
use crate::components::CopyButton;

#[component]
pub fn VideoSelector() -> impl IntoView {
    let videos = use_context::<Resource<(), Videos>>().unwrap();
    let query = use_query_map();
    let search = move || query().get("q").cloned().unwrap_or_default();
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned());
    let filter_sort = move |videos: Videos| {
        let mut videos = videos
            .clone()
            .into_iter()
            .filter(move |(_, m)| m.title.to_lowercase().contains(&search().to_lowercase()))
            .collect::<Vec<_>>();
        videos.sort_by(|(_, a), (_, b)| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
        videos
    };
    view! {
        <Form method="GET" action="">
            <input type="search" name="q" value=search oninput="this.form.requestSubmit()"/>
        </Form>
        <Transition fallback=|| view! { <p>"Loading..."</p> }>
            <ul>

                {videos
                    .get()
                    .map(|videos| {
                        view! {
                            <For
                                each=move || filter_sort(videos.clone())
                                key=|(mid, _)| mid.clone()
                                children=move |(mid, m)| {
                                    view! {
                                        <a
                                            title=m.title.clone()
                                            href={
                                                let mid = mid.clone();
                                                move || crate::path(
                                                    &format!("videos/{}{}", mid, query().to_query_string()),
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
pub fn VideoEditor() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").unwrap().clone());
    let videos = use_context::<Resource<(), Videos>>().unwrap();
    let v = move || {
        videos
            .get()
            .map(|videos| videos.get(&id()).cloned())
            .flatten()
    };
    let update_video = create_action(move |(field, value): &(String, String)| {
        let (field, value) = (field.clone(), value.clone());
        let id = id();
        async move {
            crate::client::update_video(id, &field, &value).await;
            videos.refetch();
        }
    });
    let url = create_memo(move |_| v().map(|m| m.url));
    view! {
        <div class="video-ctr">
            <Transition fallback=|| {
                view! { <p>"Loading Video"</p> }
            }>
                {move || {
                    url()
                        .map(|url| {
                            view! { <video controls src=url></video> }
                        })
                }}

            </Transition>
        </div>
        <Transition fallback=|| {
            view! { <p>"Loading Video"</p> }
        }>
            {move || {
                v()
                    .map(|v| {
                        view! {
                            <div id="detail">
                                <DetailTable
                                    video=v
                                    update_video=move |field, value| {
                                        update_video.dispatch((field, value))
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
fn DetailTable<Cb>(video: Video, update_video: Cb) -> impl IntoView
where
    Cb: 'static + Copy + Fn(String, String),
{
    let download_name = {
        if let Some(pos) = video.title.rfind(".") {
            if &video.title[pos + 1..] == video.format {
                video.title.clone()
            } else {
                video.title.clone() + "." + &video.format
            }
        } else {
            video.title.clone() + "." + &video.format
        }
    };
    view! {
        <table>
            <tr>
                <td>"Title"</td>
                <td>
                    <ClickToEdit
                        value=video.title
                        onset=move |value| update_video("title".to_string(), value)
                    />
                </td>
            </tr>
            <tr>
                <td>"Format"</td>
                <td>
                    <ClickToEdit
                        value=video.format
                        onset=move |value| update_video("format".to_string(), value)
                    />
                </td>
            </tr>
            <tr>
                <td>"URL"</td>
                <td>
                    <span class="video-url">

                        <a download=download_name href=video.url.clone()>
                            <button>"Download"</button>
                        </a>

                        {
                            #[cfg(web_sys_unstable_apis)]
                            view! {
                                <span>
                                    <CopyButton value=video.url.clone()/>
                                </span>
                            }
                        }

                        <span class="url-text" title=video.url.clone()>
                            {video.url.clone()}
                        </span>
                    </span>
                </td>
            </tr>
        </table>
    }
}
