use leptos::*;
use leptos_router::*;

#[component]
pub fn QueryForm() -> impl IntoView {
    let query = use_query_map();
    let search = move || query.get().get("q").cloned().unwrap_or_default();
    let media_type = move || query.get().get("t").cloned().unwrap_or_default();
    // TODO this should be derived from the metadata schema
    view! {
        <Form method="GET" action="." class="query">
            <label class="search">
                "Search:"
                <input type="search" name="q" value=search oninput="this.form.requestSubmit()"/>
            </label>
            <fieldset class="media-type">
                <legend>"Format:"</legend>
                <label>
                    "Video"
                    <input
                        type="radio"
                        name="t"
                        value="v"
                        checked=move || media_type() == "v"
                        oninput="this.form.requestSubmit()"
                    />
                </label>
                <label>
                    "Image"
                    <input
                        type="radio"
                        name="t"
                        value="i"
                        checked=move || media_type() == "i"
                        oninput="this.form.requestSubmit()"
                    />
                </label>
            </fieldset>
        </Form>
    }
}
