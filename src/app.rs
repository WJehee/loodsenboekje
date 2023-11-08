use leptos::*;
use leptos_meta::*;

use crate::model::entry::{Entry, get_entries};
use chrono::Datelike;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@1/css/pico.min.css"/>
        <Title text="Loodsen Boekje"/>
        <div class="container">
            <NavBar/>
            <MainPage/>
            // <Router fallback=|| view! { <h1>Error</h1> }.into_view()>
            //     <main>
            //         <Routes>=
            //             <Route path="" view=MainPage/>
            //             <Route path="/login" view=LoginPage/>
            //         </Routes>
            //     </main>
            // </Router>
        </div>
        <footer class="container">
            <hr/>
            View the <a href="https://github.com/WJehee/loodsenboekje.com">Source code</a>
        </footer>
    }
}

#[component]
fn MainPage() -> impl IntoView {
    view! {
        <AddEntryForm/>
        <SearchBar/>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <nav>
            <ul>
                <li><strong>Loodsen Boekje</strong></li>
            </ul>
            <ul>
                // TODO: if user is authenticated, show username
                <li>Ingelogd als PLACEHOLDER</li>
            </ul>
            <ul>
                <li><a href="/">Lijst</a></li>
                // TODO: if user is authenticated show this
                <li><a href="/logout">Log uit</a></li>
                // Else show this
                <li><a href="/login">Log in</a></li>
            </ul>
        </nav>
    }
}

#[component]
fn AddEntryForm() -> impl IntoView {
    view! {
        // TODO: show this form if user is authenticated, maybe one level higher?
        <details>
            <summary role="button" class="outline">Voeg een biertje toe</summary>
            <form>
                <label for="how">Hoe/wat</label>
                <input type="text" name="how"></input>
                <label for="who"></label>
                Wie (indien meer dan 1, voeg kommas toe)
                <input type="text" name="who" placeholder="Opa Dorus" required></input>
                <button type="submit" role="button">Voeg toe</button>
            </form>
        </details>
    }
}

#[component]
fn SearchBar() -> impl IntoView {
    // TODO: filter (fuzzy) the list of entries based on the search string
    let search_query = create_rw_signal("".to_string());
    view! {
        <form>
            <input
                type="search"
                placeholder="Bier opener"
                on:input=move |ev| {
                    search_query.set(event_target_value(&ev))
                }
            ></input>
        </form>
        <AllEntries/>
    }
}

#[component]
fn AllEntries() -> impl IntoView {
    let entries: RwSignal<Vec<Entry>> = create_rw_signal(vec![]);
    let entry_resource = create_resource(entries, |_| get_entries());

    view! {
        <kbd>x resultaten</kbd>
        <table>
            <thead>
                <tr>
                    <th scope="col">#</th>
                    <th scope="col">Hoe/wat</th>
                    <th scope="col">Wie</th>
                    <th scope="col">Datum</th>
                </tr>
            </thead>
            <tbody>
            <Transition fallback=move || view! {Loading...}>
                {move || entry_resource.get().map(|entries| match entries {
                    Err(_e) => view! {Error loading...}.into_view(),
                    Ok(entries) => view! {
                        <For
                            each=move || entries.clone()
                            key=|entry| entry.id
                            let:entry
                        >
                            <EntryRow entry/>
                        </For>
                    }
                })}
            </Transition>
            </tbody>
            </table>
    }
}

#[component]
fn EntryRow(entry: Entry) -> impl IntoView {
    let editing = create_rw_signal(false);
    view! {
        <tr>
            <td scope="row">{ entry.id}</td>
            <Show
                when=move || { editing.get() }
                // When not editing
                fallback=move || view! {
                    <td>{ &entry.how }</td>
                    <td>Opa dorus</td>
                    <td>{format!(
                        "{:02}-{:02}-{:04}",
                        &entry.created.day(),
                        &entry.created.month(),
                        &entry.created.year(),
                    )}</td>
                }
            >
                // When editing
                // TODO: make sure each input field matches with the api
                <td><input type="text"/></td>
                <td><input type="text"/></td>
                <td><input type="text"/></td>
            </Show>
            // TODO: only allow deletion and editing if authenticated
            <td>
                <a
                    href="#"
                    on:click=move |_| {
                        editing.update(|v| *v = !*v)
                    }
                >
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-edit-3"><path d="M12 20h9"></path><path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"></path></svg>
                </a>
                <a
                    href="#"
                    on:click=move |_| { todo!() }
                >
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-trash-2"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path><line x1="10" y1="11" x2="10" y2="17"></line><line x1="14" y1="11" x2="14" y2="17"></line></svg>
                </a>
                <a
                    href="#"
                    on:click=move |_| { todo!() }
                >
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-check"><polyline points="20 6 9 17 4 12"></polyline></svg>
                </a>
            </td>
        </tr>
    }
}

#[component]
fn LoginPage() -> impl IntoView {
    todo!();
}

