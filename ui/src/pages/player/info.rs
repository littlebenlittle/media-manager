use std::borrow::BorrowMut;

use crate::data::Metadata;
use crate::{components::ClickToEdit, log, Context};
use leptos::*;
use leptos_use::on_click_outside;

#[component]
pub fn Info() -> impl IntoView {
    let ctx = use_context::<Context>().unwrap();
    let (media, set_media) = create_signal(ctx.selected_media.get_untracked().unwrap());
    create_effect(move |x| {
        let media = media.get();
        if x.is_some() {
            ctx.set_updated_media.set(Some(media))
        }
    });
    view! {
        <div id="video-info">

            {
                #[cfg(feature = "demo")]
                view! {
                    <p>
                        "You can edit some metadata items (title, shortname, format) \
                        by clicking on them."
                    </p>
                }
            }
            <h3>
                <ClickToEdit
                    value=move || media.get().title
                    onchange=move |value| set_media.update(|media| media.title = value)
                />
            </h3> <table>
                <tr class="editable">
                    <td>"Shortname"</td>
                    <td>
                        <ClickToEdit
                            value=move || media.get().shortname
                            onchange=move |value| set_media.update(|media| media.shortname = value)
                        />
                    </td>

                </tr>

                <tr class="editable">
                    <td>"Format"</td>
                    <td>
                        <ClickToEdit
                            value=move || media.get().format
                            onchange=move |value| set_media.update(|media| media.format = value)
                        />
                    </td>

                </tr>

                <tr>
                    <td>"URL"</td>
                    <td>
                        <span>{media.get_untracked().url}</span>
                    </td>
                </tr>
            </table>
        </div>
    }
}
