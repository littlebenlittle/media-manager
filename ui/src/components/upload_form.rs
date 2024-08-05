use leptos::*;

#[component]
pub fn UploadForm() -> impl IntoView {
    let file_input = create_node_ref::<html::Input>();
    let upload = create_action(|file: &web_sys::File| {
        let file = file.clone();
        async move { crate::client::upload_file(file).await }
    });
    let files = create_rw_signal(Option::<web_sys::FileList>::None);
    let onchange = move |e: ev::Event| {
        let tgt = event_target::<web_sys::HtmlInputElement>(&e);
        let file_list = tgt.files().unwrap();
        files.set(Some(file_list.into()));
    };
    let onsubmit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if let Some(files) = files.get() {
            for i in 0..files.length() {
                upload.dispatch(files.get(i).unwrap().clone())
            }
        }
    };
    view! {
        <form class="upload" on:submit=onsubmit>
            <input type="file" multiple on:change=onchange/>
            <input node_ref=file_input class="submit" type="submit" value="Upload"/>
        </form>
    }
}
