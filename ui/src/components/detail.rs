use std::rc::Rc;

use leptos::*;
use leptos_router::*;

use crate::{collection::ID, components::ClickToEdit, Collection, Data, Metadata};

#[cfg(web_sys_unstable_apis)]
use crate::components::CopyButton;

#[allow(unexpected_cfgs)]
#[component]
pub fn Detail<UF>(data: Data, metadata: Metadata, update: UF) -> impl IntoView
where
    UF: Fn(Metadata) + Clone + 'static,
{
    let metadata = Rc::new(metadata);
    view! {
        <table class="detail">
            <tr>
                <td>"title"</td>
                <td>
                    <ClickToEdit
                        value=metadata.title()
                        onset={
                            let metadata = metadata.clone();
                            let update = update.clone();
                            move |title| update(metadata.with_title(title.clone()))
                        }
                    />

                </td>
            </tr>
            <tr>
                <td>"format"</td>
                <td>
                    <ClickToEdit
                        value=metadata.format()
                        onset={
                            let metadata = metadata.clone();
                            let update = update.clone();
                            move |format| update(metadata.with_format(format.clone()))
                        }
                    />

                </td>
            </tr>
            <tr>
                <td>"url"</td>
                <td>
                    <span class="media-url">
                        <a download=data.download_name() href=data.url()>
                            <button>"Download"</button>
                        </a>

                        {
                            #[cfg(web_sys_unstable_apis)]
                            view! {
                                <span>
                                    <CopyButton value=data.url()/>
                                </span>
                            }
                        }

                        <span class="url-text" title=data.url()>
                            {data.url()}
                        </span>
                    </span>
                </td>
            </tr>
        </table>
    }
}
