use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    components::{MyInput, SearchBar, AddEntryForm},
    model::{
        entry::AddEntry,
        user::{Register, validate_username, validate_password, UserType, User, user_leaderboard}
    },
    auth::{Login, Logout, current_user}
};

#[component]
pub fn App() -> impl IntoView {
    let login = create_server_action::<Login>();
    let register = create_server_action::<Register>();
    let logout = create_server_action::<Logout>();
   
    let user = create_resource(
        move || {(
            login.version().get(),
            register.version().get(),
            logout.version().get(),
        )},
        move |_| current_user(),
    );
    provide_context(user);
    provide_meta_context();

    view! {
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@next/css/pico.min.css"/>
        <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png"/>
        <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png"/>
        <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png"/>
        <link rel="manifest" href="/site.webmanifest"/>
        <link rel="stylesheet" href="/style.css"/>

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
                    Ok(Some(user)) => {
                        view! {
                        <ul>
                            <li class="hidden-mobile">Ingelogd als {user.name}</li>
                        </ul>
                        <ul>
                            <li><a href="/leaderboard">Leaderboard</a></li>
                            <li>
                                <ActionForm action=logout>
                                    <input type="submit" value="Log uit"/>
                                </ActionForm>
                            </li>
                        </ul>
                    }}.into_view(),
                    _ => {
                        view! {
                        <ul>
                            <li class="hidden-mobile">Niet ingelogd</li>
                        </ul>
                        <ul>
                            <li><a href="/login">Login</a></li>
                            <li><a href="/register">Nieuw account</a></li>
                        </ul>
                    }}.into_view()
                })}
            </Transition>
            </nav>
            <main>
                <Router fallback=|| view! { <h1>Router error</h1> }.into_view()>
                    <Routes>
                        <Route path="/" view=MainPage/>
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
            View the <a href="https://github.com/WJehee/loodsenboekje.com" target="_blank">Source code</a>
        </footer>
    }
}

#[component]
fn MainPage() -> impl IntoView {
    let add_entry = create_server_action::<AddEntry>();
    let user = use_context::<Resource<(usize, usize, usize), Result<Option<User>, ServerFnError>>>()
        .expect("to have user set in context");

    view! {
        <Transition
            fallback=move || view!{<span>Loading...</span>}
        >
        {move || user.get().map(|user| match user {
           Ok(Some(user)) => match user.user_type {
               UserType::Writer | UserType::Admin => view! {
                   <AddEntryForm add_entry/> 
                   <SearchBar add_entry/>
               }.into_view(),
               UserType::Reader | UserType::Inactive => view!{
                    <SearchBar add_entry/>
               }.into_view(),
           },
           _ => ().into_view(),
        })}
        </Transition>
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
    let leaderboard = create_resource(|| (), |_| async move { user_leaderboard().await });
    view!{
        <h1>Leaderboard</h1>
         <Transition>
            {move || leaderboard.get().map(|users| match users{
                Err(e) => {
                    let e = match e {
                        ServerFnError::ServerError(e) => e.to_string(),
                        _ => "Error loading leaderboard".to_string(),
                    };
                    view! {<span>{e.to_string()}</span>}.into_view()
                },
                Ok(users) => view! {
                    <table>
                        <thead>
                            <tr>
                                <td>Naam</td>
                                <td>Aantal</td>
                            </tr>
                        </thead>
                        <tbody>
                        <For
                            each=move || users.clone()
                            key=|user| user.id
                            let:user
                        >
                            <tr>
                                <td><a href=format!("/?search={0}", user.name)>{user.name}</a></td>
                                <td>{user.count}</td>
                            </tr>
                        </For>
                        </tbody>
                    </table>
                }.into_view()
            })}
        </Transition>
    }
}

