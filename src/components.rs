use leptos::*;
use leptos_router::*;
use chrono::Datelike;

use crate::model::{entry::{Entry, AddEntry, get_entries, validate_who, DeleteEntry}, user::get_all_users};

#[component]
pub fn AddEntryForm(
    add_entry: Action<AddEntry, Result<i64, ServerFnError>>
) -> impl IntoView {
    view! {
        <details>
            <summary role="button" class="outline">Voeg een biertje toe</summary>
            <UserDataList/>
            <ActionForm action=add_entry>
                <label for="how">
                    Hoe/wat
                    <input type="text" name="how"/>
                </label>
                <label for="who">
                    Wie (indien meer dan 1, voeg kommas toe)
                    <MyInput 
                        input_type="text"
                        input_name="who"
                        input_placeholder="Opa Dorus"
                        error_msg="Alleen letters, kommas en spaties toegestaan"
                        validation_function=validate_who
                        // This only works for the first entry, which is fine for now
                        input_list="userdata"
                    />
                </label>
                <button type="submit" role="button">Voeg toe</button>
            </ActionForm>
        </details>
    }
}

#[component]
pub fn MyInput(
    input_type: &'static str,
    input_name: &'static str,
    input_placeholder: &'static str,
    error_msg: &'static str,
    validation_function: fn(&str) -> bool,
    #[prop(optional)]
    input_list: &'static str,
) -> impl IntoView {
    let invalid = create_rw_signal("");
    let error = create_rw_signal("");
    view! {
        <input 
            type=input_type
            name=input_name
            placeholder=input_placeholder
            aria-invalid=invalid
            required
            autocomplete="off"
            list=input_list
            on:input=move |ev| {
                let data = event_target_value(&ev);
                if validation_function(&data) {
                    invalid.set("false");
                    error.set("");
                } else {
                    invalid.set("true");
                    error.set(error_msg);
                }
            }/>
        <small>{error}</small>
    }
}

#[component]
fn UserDataList() -> impl IntoView {
    let users = create_resource(|| (), |_| async move { get_all_users().await });
    view!{
         <Transition>
            {move || users.get().map(|users| match users{
                Err(e) => view! {<span>{e.to_string()}</span>}.into_view(),
                Ok(users) => view! {
                    <datalist id="userdata">
                    <For
                        each=move || users.clone()
                        key=|user| user.id
                        let:user
                    >
                        <option>{user.name}</option>
                    </For>
                    </datalist>
                }.into_view()
            })}
        </Transition>
    }
}

#[component]
pub fn SearchBar(
    add_entry: Action<AddEntry, Result<i64, ServerFnError>>
) -> impl IntoView {
    let query = use_query_map();
    let search = move || query().get("search").cloned().unwrap_or_default();

    let delete_entry = create_server_action::<DeleteEntry>();

    let entry_resource = create_resource(
        move || {(
            search(),
            add_entry.version().get(),
            delete_entry.version().get(),
        )},
        |(query,  _, _)| get_entries(query)
    );
    view! {
        <Form method="GET" action="">
            <input
                type="search"
                name="search"
                placeholder="Bier opener"
                oninput="this.form.requestSubmit()"
            />
        </Form>
        <AllEntries delete_entry entry_resource/>
    }
}

#[component]
fn AllEntries(
    delete_entry: Action<DeleteEntry, Result<(), ServerFnError>>,
    entry_resource: Resource<(String, usize, usize), Result<Vec<Entry>, ServerFnError>>
) -> impl IntoView {
    view! {
        <Transition>
            {move || entry_resource.get().map(|entries| match entries {
                // TODO: display error more nicely
                Err(e) => view! {<span>{e.to_string()}</span>}.into_view(),
                Ok(entries) => view! {
                    <kbd>{ entries.len() } resultaten</kbd>
                    <table>
                        <thead>
                            <tr>
                                <th scope="col">#</th>
                                <th scope="col">Hoe/wat</th>
                                <th scope="col">Wie</th>
                                <th scope="col">Datum</th>
                                <th scope="col"></th>
                            </tr>
                        </thead>
                        <tbody>
                            <For
                                each=move || entries.clone()
                                key=|entry| entry.id
                                let:entry
                            >
                                <EntryRow entry delete_entry/>
                            </For>
                        </tbody>
                    </table>
                }.into_view()
            })}
        </Transition>
    }
}

#[component]
fn EntryRow(
    entry: Entry,
    delete_entry: Action<DeleteEntry, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        <tr>
            <td scope="row">{ entry.id }</td>
            <td>{ &entry.how }</td>
            <td>Opa Dorus</td>
            // <td>{ &entry.who }</td>
            <td>{format!(
                "{:02}-{:02}-{:04}",
                &entry.created.day(),
                &entry.created.month(),
                &entry.created.year(),
            )}</td>
            <td>
                <ActionForm action=delete_entry>
                    <input type="hidden" name="id" value={entry.id}/>
                    <button type="submit" name="submit" class="outline secondary">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-trash-2"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path><line x1="10" y1="11" x2="10" y2="17"></line><line x1="14" y1="11" x2="14" y2="17"></line></svg>
                    </button>
                </ActionForm>
            </td>
        </tr>
    }
}

