use crate::data::Metadata;
use crate::{log, Context};
use leptos::*;
use leptos_use::on_click_outside;

#[component]
pub fn Info() -> impl IntoView {
    let ctx = use_context::<Context>().unwrap();
    let title = create_rw_signal(ctx.selected_media.get_untracked().unwrap().title);
    let format = create_rw_signal(ctx.selected_media.get_untracked().unwrap().format);
    let shortname = create_rw_signal(ctx.selected_media.get_untracked().unwrap().shortname);
    let url = ctx.selected_media.get_untracked().unwrap().url;
    create_effect({
        let url = url.clone();
        move |x| {
            // these must be accessed outside of the conditional
            // to be registered with the runtime
            let title = title.get();
            let format = format.get();
            let shortname = shortname.get();
            // x prevents effect cycle due to re-render
            if x.is_some() {
                ctx.set_updated_media.set(Some(Metadata {
                    title,
                    format,
                    shortname,
                    url: url.clone(),
                }))
            }
        }
    });
    view! {
        <div id="video-info">
            <ClickToEdit sig=title/>

            <table>
                <tr class="editable">
                    <td>"Shortname"</td>
                    <td>
                        <ClickToEdit sig=shortname/>

                    </td>

                </tr>

                <tr class="editable">
                    <td>"Format"</td>
                    <td>
                        <ClickToEdit sig=format/>

                    </td>

                </tr>

                <tr>
                    <td>"URL"</td>
                    <td>
                        <span>{url}</span>
                    </td>
                </tr>
            </table>
        </div>
    }
}

#[component]
fn ClickToEdit(sig: RwSignal<String>) -> impl IntoView {
    let (edit, set_edit) = create_signal(false);
    let val = create_rw_signal(sig.get_untracked());
    let node = create_node_ref::<html::Input>();
    let _ = on_click_outside(node, move |_| {
        if edit.get_untracked() {
            set_edit(false);
            let val = val.get_untracked();
            if sig.get() != val {
                sig.set(val);
            }
        }
    });
    view! {
        <input
            hidden=move || !edit()
            node_ref=node
            type="text"
            value=val.get_untracked()
            on:input=move |e| {
                val.set(event_target_value(&e));
            }

            on:keydown=move |e| {
                if e.key() == "Enter" {
                    set_edit(false);
                    let val = val.get_untracked();
                    if sig.get() != val {
                        sig.set(val);
                    }
                } else if e.key() == "Escape" {
                    set_edit(false);
                    val.set(sig.get());
                }
            }
        />

        <span
            hidden=move || edit()
            on:click=move |_| {
                set_edit(true);
                node.get().unwrap().select();
            }
        >
            {move || sig.get()}
        </span>
    }
}
