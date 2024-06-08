use leptos::*;

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    let spa_wiki = "https://en.wikipedia.org/wiki/Single-page_application";
    view! {
        <div id="not-found">
            <h3>"Not Found"</h3>
            <br/>
            <h4>
                "Check out the navigation menu by hovering your cursor \
                 over the blue-ish purple-ish bar on the left."
            </h4>
            <br/>
            <h5>"But this URL worked before!"</h5>
            <p>
                "This app is a " <a href=spa_wiki>"Single Page Applicaton"</a> " \
                or SPA. This means that routing (how your browser decides what \
                to do with the URL) is determined by code running on this page \
                rather than by making a web request to a remote server."
            </p>
            <p>
                "This would normally be fine, but this site is also hosted on Github \
                Pages, which doesn't currently provide support for a SPA's routing \
                mechanism. That means you may need to navigate to the page using \
                a " <em>"link in the app"</em> " rather than by following an external \
                link."
            </p>
            <p>
                "If you clicked something inside the app and got to this page, that's \
                definitely a bug!"
            </p>
        </div>
    }
}
