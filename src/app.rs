use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@1/css/pico.min.css"/>
        <Title text="Loodsen Boekje"/>
        <div class="container">
            <NavBar/>
            <AddEntryForm/>
            <SearchBar/>

            <Router fallback=|| view! { <h1>Error</h1> }.into_view()>
                <main>
                    <Routes>=
                        <Route path="" view=HomePage/>
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
fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
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
            // {% if current_user.is_authenticated %}
                <li>Ingelogd als PLACEHOLDER</li>
            </ul>
            // {% endif %}
            <ul>
                <li><a href="/">Lijst</a></li>
                // {% if current_user.is_authenticated %}
                <li><a href="/logout">Log uit</a></li>
                // {% else %}
                <li><a href="/login">Log in</a></li>
                // {% endif %}
            </ul>
        </nav>
    }
}

#[component]
fn AddEntryForm() -> impl IntoView {
    view! {
        // {% if current_user.is_authenticated %}
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
        // {% endif %}
    }
}

#[component]
fn SearchBar() -> impl IntoView {
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
    // TODO: get entries from database
    let entries = create_rw_signal(vec![0, 1, 2]);
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
            <For
                each=move || entries.get()
                key=|entry| *entry
                let:entry
            >
                <EntryRow/>
            </For>
            </tbody>
        </table>
    }
}

#[component]
fn EntryRow() -> impl IntoView {
    view! {
        <tr>
            <td>Hello world</td>
            // <td scope='row'>{{ entry.id}}</td>
            // <td>
            //     <template v-if="!entry.editing">{{ entry.how}}</template>
            //     <input v-else type="text" v-model="entry.how">
            // </td>
            // <td>
            //     <template v-if="!entry.editing">{{ entry.who.join(', ') }}</template>
            //     <input v-else type="text" v-model="entry.who">
            // </td>
            // <td>
            //     <template v-if="!entry.editing">{{ entry.created }}</template>
            //     <input v-else type="date" v-model="entry.created">
            // </td>
            // // <!-- only allow deletion and editing if authenticated -->
            // // {% if current_user.is_authenticated %}
            // <td>
            //     <a href="#" v-if="!entry.editing" @click="editEntry(entry)">
            //         <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-edit-3"><path d="M12 20h9"></path><path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"></path></svg>
            //     </a>
            //     <div v-else style="display: flex; justify-content: space-between;">
            //         <a href="#" @click="cancelEdit(entry)">
            //             <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-x"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
            //         </a>
            //         <a href="#" @click="deleteEntry(entry.id)">
            //             <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-trash-2"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path><line x1="10" y1="11" x2="10" y2="17"></line><line x1="14" y1="11" x2="14" y2="17"></line></svg>
            //         </a>
            //         <a href="#" @click="saveEntry(entry)">
            //             <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-check"><polyline points="20 6 9 17 4 12"></polyline></svg>
            //         </a>
            //     </div>
            // </td>
            // {% endif %}
        </tr>
    }
}

