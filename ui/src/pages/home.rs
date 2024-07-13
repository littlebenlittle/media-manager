use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div id="media-manager-description" class="content">
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

            {
                let github_repo = "https://github.com/littlebenlittle/media-manager";
                #[cfg(feature = "demo")]
                view! {
                    <h3>"Demo Mode"</h3>
                    <p>
                        "You are viewing the app in demo mode. This means that there \
                         isn't an API server and all requests to the API return fake \
                         responses. If you want to experiment with the full version,  \
                         you can clone the source code from the "
                        <a href=github_repo target="_blank">
                            "GitHub repo"
                        </a> " and follow the instructions in the README."
                    </p>
                }
            }

        </div>
    }
}
