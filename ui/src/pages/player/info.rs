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
            <ClickToEdit
                sig=title
                children=move |tr| {
                    view! { <h3 on:click=move |_| tr.notify()>{title()}</h3> }
                }
            />

            <table>
                <tr class="editable">
                    <td>"Shortname"</td>
                    <ClickToEdit
                        sig=shortname
                        children=move |tr| {
                            view! {
                                <td on:click=move |_| tr.notify()>
                                    <span>{shortname()}</span>
                                </td>
                            }
                        }
                    />

                </tr>

                <tr class="editable">
                    <td>"Format"</td>
                    <ClickToEdit
                        sig=format
                        children=move |tr| {
                            view! {
                                <td on:click=move |_| tr.notify()>
                                    <span>{format()}</span>
                                </td>
                            }
                        }
                    />

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
fn ClickToEdit<F, IV>(sig: RwSignal<String>, children: F) -> impl IntoView
where
    F: 'static + Fn(Trigger) -> IV,
    IV: IntoView,
{
    let (edit, set_edit) = create_signal(false);
    let val = create_rw_signal(sig.get_untracked());
    let trigger = create_trigger();
    let node = create_node_ref::<html::Input>();
    create_effect(move |x| {
        trigger.track();
        if x.is_some() {
            // triggered rather than run by default
            set_edit(true);
            let n = node.get().unwrap();
            // n.focus().expect("input.focus");
            n.select();
        }
    });
    let _ = on_click_outside(node, move |_| {
        set_edit(false);
        sig.set(val.get());
    });
    view! {
        {move || {
            if edit() {
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
                                // order matter; set_edit will be disposed
                                // if sig is triggered first
                                set_edit(false);
                                sig.set(val.get());
                            } else if e.key() == "Escape" {
                                set_edit(false);
                                val.set(sig.get());
                            }
                        }
                    />
                }
                    .into_view()
            } else {
                children(trigger).into_view()
            }
        }}
    }
}
