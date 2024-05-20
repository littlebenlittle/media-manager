use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
    <div id="media-manager-description">
        <h2>"Welcome to Media Manager!"</h2>
        <p>
            "Media Manager helps you take charge of your own media library by \
             providing various tools for cataloguing and transforming your \
             media files."
        </p>
        <p>
            "Check out the navigation on the left by hovering your cursor over \
             that blue-ish line on the far left. In there you'll find the various \
             dashboards you can use to interact with your media libarary."
        </p>
    </div>
    }
}

#[inline(always)]
fn log_error(e: impl Into<error::Error>) {
    // TODO: toaster
    web_sys::console::log_1(&e.into().to_string().into())
}
