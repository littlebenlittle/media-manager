use std::collections::HashMap;

use leptos::*;

#[component]
pub fn NotificationTray<F>(message: F) -> impl IntoView
where
    F: Fn() -> Option<View> + Copy + 'static,
{
    let notifs = create_rw_signal(HashMap::<String, View>::new());
    create_effect(move |_| {
        if let Some(item) = message() {
            let id = uuid::Uuid::new_v4().to_string();
            notifs.update(|notifs| {
                notifs.insert(id.clone(), item);
            });
            let timeout_fn = leptos_use::use_timeout_fn(
                move |_| {
                    notifs.update(|notifs| {
                        notifs.remove(&id);
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
                each=move || notifs.get()
                key=|(id, _)| id.clone()
                children=|(_, notif)| view! { <div>{notif}</div> }
            />
        </div>
    }
}
