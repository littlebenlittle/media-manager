use std::collections::HashMap;

use crate::{
    client,
    components::ClickToEdit,
    data::{Collection, ID},
};
use leptos::*;

#[component]
pub fn TodoList() -> impl IntoView {
    let todo_list: Resource<(), Result<Collection<String>, String>> =
        create_resource(|| {}, |_| async { crate::client::todo_list().await });
    let add_todo = create_action(|text: &String| {
        let text = text.clone();
        async move { client::add_todo_item(text).await }
    });
    view! {
        <div id="todo-list">
            <Suspense fallback=|| view! { <p>"Loading TODO Items..."</p> }>
                <ul>
                    <li>
                        <ClickToEdit
                            value=|| "New TODO...".to_owned()
                            onchange=move |value| add_todo.dispatch(value)
                        />
                    </li>
                    {todo_list
                        .get()
                        .map(|result| match result {
                            Err(e) => view! { <li>{e.to_string()}</li> }.into_view(),
                            Ok(items) => {
                                view! {
                                    <For
                                        each=move || items.clone()
                                        key=|(id, _)| id.clone()
                                        children=|(id, text)| {
                                            view! { <TodoItem id text/> }
                                        }
                                    />
                                }
                                    .into_view()
                            }
                        })}

                </ul>
            </Suspense>
        </div>
    }
}

#[component]
fn TodoItem(id: ID, text: String) -> impl IntoView {
    let edit_todo = create_action(|(id, text): &(ID, String)| {
        let text = text.clone();
        let id = id.clone();
        async move { client::edit_todo_item(id, text).await }
    });
    view! {
        <ClickToEdit
            value=move || text.clone()
            onchange=move |value| edit_todo.dispatch((id.clone(), value))
        />
    }
}
