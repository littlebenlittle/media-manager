use leptos::*;

#[component]
pub fn Loading(#[prop(optional)] what: Option<String>) -> impl IntoView {
    view! { <p>"Loading" {
        if let Some(what) = what {
            view!{ " "{what} }.into_view()
        } else {
            view!{}.into_view()
        }
    } "..."</p> }
}
