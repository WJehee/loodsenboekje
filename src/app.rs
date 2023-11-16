use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    model::{entry::{Entry, get_entries, AddEntry, DeleteEntry}, user::Register},
    auth::{Login, current_user},
};
use chrono::Datelike;

#[component]
pub fn App() -> impl IntoView {
    let login = create_server_action::<Login>();
    let register = create_server_action::<Register>();

    let add_entry = create_server_action::<AddEntry>();
    
    // TODO: fix page refresh unload
    let user = create_resource(
        move || {
            (
                login.version().get(),
                register.version().get(),
                // logout.version().get(),
            )
        },
        move |_| current_user(),
    );
    provide_meta_context();

    view! {
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@next/css/pico.min.css"/>
        <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png"/>
        <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png"/>
        <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png"/>
        <link rel="manifest" href="/site.webmanifest"/>
        <Title text="Loodsen Boekje"/>
        <div class="container">
            <nav>
                <ul>
                    <li><a href="/"><strong>Loodsen Boekje</strong></a></li>
                </ul>
                <ul>
                    { move || match user.get() {
                        Some(Ok(Some(user))) => view! { <li>Ingelogd als {user} </li> }.into_view(),
                        _ => view! {}.into_view()
                    }}
                </ul>
                <ul>
                    { move || match user.get() {
                        Some(Ok(Some(_))) => view! {<li><a href="/logout">Log uit</a></li>}.into_view(),
                        _ => view! {
                            <li><a href="/login">Log in</a></li>
                            <li><a href="/register">Nieuw account</a></li>
                        }.into_view()
                    }}
                </ul>
            </nav>
            <Router fallback=|| view! { <h1>Router error</h1> }.into_view()>
                <main>
                    <Routes>
                        <Route path="" view=move || view! {
                            <AddEntryForm add_entry/>
                            <SearchBar add_entry/>
                        }/>
                        <Route path="/login" view=move || view! {
                            <LoginPage login/>
                        }/>
                        <Route path="/register" view=move || view! {
                            <RegisterPage register/>
                        }/>
                    </Routes>
                </main>
            </Router>
        </div>
        <footer class="container">
            <hr/>
            View the <a href="https://github.com/WJehee/loodsenboekje.com">Source code</a>
        </footer>
    }
}

#[component]
fn AddEntryForm(
    add_entry: Action<AddEntry, Result<i64, ServerFnError>>
) -> impl IntoView {
    view! {
        <details>
            <summary role="button" class="outline">Voeg een biertje toe</summary>
            <ActionForm action=add_entry>
                <label for="how">
                    Hoe/wat
                    <input type="text" name="how"/>
                </label>
                <label for="who">
                    Wie (indien meer dan 1, voeg kommas toe)
                    <input type="text" name="who" placeholder="Opa Dorus" required/>
                </label>
                <button type="submit" role="button">Voeg toe</button>
            </ActionForm>
        </details>
    }
}

#[component]
fn SearchBar(
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
        <h1>{search}</h1>
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
                Err(_e) => view! {Error loading entries}.into_view(),
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
            // TODO: add actual usernames separated by commas
            <td>Opa dorus</td>
            <td>{format!(
                "{:02}-{:02}-{:04}",
                &entry.created.day(),
                &entry.created.month(),
                &entry.created.year(),
            )}</td>
            // TODO: only show deletion if authorized
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

#[component]
fn LoginPage(login: Action<Login, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <main class="container">
            <h2>Login</h2>
            <ActionForm action=login>
                <div class="grid">
                    <label for="username">
                        Gebruikersnaam
                        <input type="text" name="username" placeholder="Gebruikersnaam" required/>
                    </label>
                    <label for="password">
                        Wachtwoord
                        <input type="password" name="password" placeholder="Wachtwoord" required/>
                    </label>
                </div>
                <button type="submit">Inloggen</button>
            </ActionForm>
        </main>
    }
}

#[component]
fn RegisterPage(register: Action<Register, Result<i64, ServerFnError>>) -> impl IntoView {
    view! {
        <main class="container">
            <h2>Registreer een nieuw account</h2>
            <ActionForm action=register>
                <div class="grid">
                    <label for="username">
                        Gebruikersnaam
                        <input type="text" id="username" name="username" placeholder="Gebruikersnaam" required/>
                    </label>
                    <label for="password">
                        Wachtwoord
                        <input type="password" id="password" name="password" placeholder="Wachtwoord" required/>
                    </label>
                </div>
                <label for="creation password">
                    Registratie Wachtwoord 
                    <input type="password" id="creation_password" name="creation_password" placeholder="Registratie Wachtwoord" required/>
                </label>
                <button type="submit">Aanmelden</button>
            </ActionForm>
        </main>
    }
}

