use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    components::{MyInput, SearchBar, AddEntryForm},
    model::{
        entry::AddEntry,
        user::{Register, validate_username, validate_password, get_all_users}
    },
    auth::{Login, Logout, current_user}
};

#[component]
pub fn App() -> impl IntoView {
    let login = create_server_action::<Login>();
    let register = create_server_action::<Register>();
    let logout = create_server_action::<Logout>();
    let add_entry = create_server_action::<AddEntry>();
    
    let user = create_resource(
        move || {(
            login.version().get(),
            register.version().get(),
            logout.version().get(),
        )},
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
                <Transition
                    fallback=move || view!{<span>Loading...</span>}
                >
                { move || user.get().map(|user| match user {
                    Ok(Some(user)) => view! {
                        <ul>
                            <li>Ingelogd als {user}</li>
                        </ul>
                        <ul>
                            <li><a href="/leaderboard">Leaderboard</a></li>
                            <li>
                                <ActionForm action=logout>
                                    <input type="submit" value="Log uit"/>
                                </ActionForm>
                            </li>
                        </ul>
                    }.into_view(),
                    _ => view! {
                        <ul>
                            <li>Niet ingelogd</li>
                        </ul>
                        <ul>
                            <li><a href="/login">Login</a></li>
                            <li><a href="/register">Nieuw account</a></li>
                        </ul>
                    }.into_view()
                })}
            </Transition>
            </nav>
            <main>
                <Router fallback=|| view! { <h1>Router error</h1> }.into_view()>
                    <Routes>
                        <Route path="/" view=move || view! {
                            <AddEntryForm add_entry/>
                            <SearchBar add_entry/>
                        }/>
                        <Route path="/login" view=move || view! {
                            <LoginPage login/>
                        }/>
                        <Route path="/register" view=move || view! {
                            <RegisterPage register/>
                        }/>
                        <Route path="/leaderboard" view=LeaderBoard/>
                    </Routes>
                </Router>
            </main>
        </div>
        <footer class="container">
            <hr/>
            View the <a href="https://github.com/WJehee/loodsenboekje.com">Source code</a>
        </footer>
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
                        <MyInput
                            input_type="text"
                            input_name="username"
                            input_placeholder="Gebruikersnaam"
                            error_msg="Alleen letters toegestaan"
                            validation_function=validate_username
                        />
                    </label>
                    <label for="password">
                        Wachtwoord
                        <MyInput
                            input_type="password"
                            input_name="password"
                            input_placeholder=""
                            error_msg="Wachtwoord moet minimaal 8 karakters bevatten"
                            validation_function=validate_password
                        />
                    </label>
                </div>
                <button type="submit">Inloggen</button>
            </ActionForm>
        </main>
    }
}

#[component]
fn RegisterPage(register: Action<Register, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <main class="container">
            <h2>Registreer een nieuw account</h2>
            <ActionForm action=register>
                <div class="grid">
                    <label for="username">
                        Gebruikersnaam
                        <MyInput
                            input_type="text"
                            input_name="username"
                            input_placeholder="Gebruikersnaam"
                            error_msg="Alleen letters toegestaan"
                            validation_function=validate_username
                        />
                    </label>
                    <label for="password">
                        Wachtwoord
                        <MyInput
                            input_type="password"
                            input_name="password"
                            input_placeholder=""
                            error_msg="Wachtwoord moet minimaal 8 karakters bevatten"
                            validation_function=validate_password
                        />
                    </label>
                </div>
                <label for="creation password">
                    Registratie Wachtwoord 
                    <input type="password" name="creation_password" placeholder="Registratie Wachtwoord" required/>
                </label>
                <button type="submit">Aanmelden</button>
            </ActionForm>
        </main>
    }
}

#[component]
fn LeaderBoard() -> impl IntoView {
    let users = create_resource(|| (), |_| async move { get_all_users().await });
    view!{
        <h1>Leaderboard!</h1>
         <Transition>
            {move || users.get().map(|users| match users{
                // TODO: display error more nicely
                Err(e) => view! {<span>{e.to_string()}</span>}.into_view(),
                Ok(users) => view! {
                    <For
                        each=move || users.clone()
                        key=|user| user.id
                        let:user
                    >
                        <h2>{user.name}</h2>
                    </For>
                }.into_view()
            })}
        </Transition>
    }
}

