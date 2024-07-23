use std::collections::HashMap;

use leptos::*;

use crate::log;

#[component]
pub fn Toaster<F>(message: F) -> impl IntoView
where
    F: Fn() -> Option<View> + Copy + 'static,
{
    let toasts = create_rw_signal(HashMap::<String, View>::new());
    create_effect(move |_| {
        if let Some(item) = message() {
            log!("new media C");
            let id = uuid::Uuid::new_v4().to_string();
            toasts.update(|toasts| {
                toasts.insert(id.clone(), item);
            });
            let timeout_fn = leptos_use::use_timeout_fn(
                move |_| {
                    log!("removing toast...");
                    toasts.update(|toasts| {
                        toasts.remove(&id);
                    });
                },
                5_000.0,
            );
            (timeout_fn.start)(());
        }
    });
    view! {
        <div id="notification-tray">
            <h3>"Notification Tray"</h3>
            <For
                each=move || toasts.get()
                key=|(id, _)| id.clone()
                children=|(_, toast)| view! { <div>{toast}</div> }
            />
        </div>
    }
}
