use core::time;
use std::thread;

use leptos::*;
use leptos_use::{use_timeout_fn, UseTimeoutFnReturn};

#[component]
pub fn Loading(#[prop(optional)] what: Option<String>) -> impl IntoView {
    view! {
        <p>
            "Loading"
            {if let Some(what) = what {
                view! {
                    " "
                    {what}
                }
                    .into_view()
            } else {
                view! {}.into_view()
            }}
            "..."
        </p>
    }
}

#[component]
pub fn LoremIpsum() -> impl IntoView {
    view! {
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam pretium lobortis dui non facilisis. Vestibulum quis malesuada nisl. Ut at nibh vitae magna rutrum ultrices in quis ante. Duis velit massa, semper ultricies pharetra eget, vulputate sit amet eros. Aenean purus elit, lacinia ac ligula vel, suscipit porta augue. Etiam id orci suscipit, varius sem quis, aliquam arcu. Suspendisse vitae feugiat lectus. Suspendisse tortor sapien, tristique vel elit quis, iaculis consectetur lorem. Etiam vel nunc ac lacus vulputate interdum. Proin sed dui vitae lectus imperdiet aliquam aliquet pharetra tellus. Aliquam ullamcorper ex tellus, tincidunt lacinia lectus volutpat nec. Aenean blandit, urna vel ultricies hendrerit, odio purus convallis libero, ac tristique nibh mi ut tellus. Aliquam sagittis sapien in nibh cursus consequat. Praesent ornare neque ipsum, sed scelerisque nisl vulputate nec. Cras gravida sapien magna, vestibulum maximus velit aliquet pulvinar. \
        Etiam non nisl quis leo mollis eleifend molestie maximus risus. Donec sit amet lectus venenatis, finibus lectus vel, laoreet nulla. Donec eget fringilla sapien, non suscipit nibh. Integer at porta eros. Vestibulum urna nisi, hendrerit ac fringilla vel, eleifend sit amet eros. Donec quis nulla urna. Nulla at pharetra felis. Duis aliquam laoreet suscipit. Vestibulum eget rhoncus lectus, nec eleifend lacus. Integer nec libero aliquet, interdum lacus ut, rutrum augue. Sed magna risus, tincidunt non porttitor in, ornare vitae dolor. Mauris malesuada felis tortor, vitae tempor augue ultrices sed. Praesent placerat augue vel luctus ullamcorper. Sed luctus massa nec sapien consequat tempor. Fusce mattis gravida malesuada. \
        Suspendisse molestie nulla scelerisque tellus tempor hendrerit. Nam interdum, eros id scelerisque feugiat, orci lectus viverra lectus, et finibus nibh tellus vitae erat. Fusce ipsum leo, varius ac maximus congue, gravida vel felis. Nulla facilisi. Fusce iaculis ut ipsum vel consectetur. Aenean arcu augue, viverra non felis eu, varius tempor sapien. Pellentesque a ipsum eros. Vestibulum risus arcu, porttitor ac neque eu, pharetra sodales ligula. Nunc tincidunt bibendum leo, at ultricies augue ornare sit amet. Sed eu aliquet diam. Curabitur ut lectus urna. Nullam hendrerit lacinia nibh et pharetra. Aliquam ac bibendum lectus. Nunc et ex ac nisl tristique aliquet. \
        Quisque nec purus enim. Etiam tempus mattis ligula sit amet viverra. Sed rhoncus leo est, vel commodo nunc malesuada et. Morbi tortor dui, pulvinar vel ultricies porttitor, porttitor ut libero. Mauris vel dui elementum risus congue sagittis. Suspendisse potenti. Fusce gravida molestie felis eu rhoncus. Ut quis orci quis augue tempus tempor ac et est. Nam dapibus velit et nibh viverra volutpat. Suspendisse et vulputate dolor, interdum mollis lorem. Interdum et malesuada fames ac ante ipsum primis in faucibus. Sed tincidunt nisl a enim hendrerit rhoncus. \
        Quisque egestas nisl quis arcu rutrum semper. Nulla tincidunt iaculis tellus ac ultricies. In nunc libero, efficitur imperdiet tortor eu, commodo luctus lectus. Donec mollis dolor turpis, eu sodales lorem posuere ut. Nulla vel ante ut arcu faucibus euismod. Maecenas id mi nisi. Donec at lectus sed orci aliquet aliquam. Nulla semper porttitor magna, sed tristique massa dictum quis. Suspendisse bibendum porttitor purus. Sed ac sapien nec massa euismod semper. Vestibulum sodales neque augue, ac egestas eros suscipit finibus. Morbi eget quam non elit viverra condimentum sed vitae orci. Quisque at massa in neque tincidunt tincidunt sit amet sed sapien. Suspendisse feugiat mollis egestas. Duis in velit faucibus, tristique enim at, aliquam augue."
    }
}

#[component]
pub fn SyncButton<T: 'static>(action: Action<(), T>, pending: ReadSignal<bool>) -> impl IntoView {
    view! {
        <button
            on:click=move |_| action.dispatch(())
            prop:disabled=move || pending.get()
            class:disabled-button=move || pending.get()
        >
            {move || if pending.get() { "Syncing..." } else { "Sync" }}
        </button>
    }
}
