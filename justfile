alias w := watch
alias l := load

# Leptos watch
watch:
    cargo leptos watch

# Load data
load:
    cargo run --bin load_data --features="ssr"

# Initialize database
init:
    cargo sqlx db create
    cargo sqlx migrate run

