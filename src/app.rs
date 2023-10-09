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
                <li>Ingelogd als</li>
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
                <input type="how" id="how" name="how"></input>
                <label for="who"></label>
                Wie (indien meer dan 1, voeg kommas toe)
                <input type="text" id="who" name="who" placeholder="Opa Dorus" v-model="who" required></input>
                <button type="submit" role="button">Voeg toe</button>
            </form>
        </details>
        // {% endif %}
    }
}

#[component]
fn SearchBar() -> impl IntoView {
    view! {
        <form>
            <input type="search" name="search" placeholder="Bier opener"></input>
        </form>
    }
}

