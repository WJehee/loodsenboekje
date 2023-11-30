alias w := watch
alias l := load

# Leptos watch
watch:
    cargo leptos watch

# Initialize database
init:
    cargo sqlx db create
    cargo sqlx migrate run

# Build for release
build:
    cargo leptos build --release -vv

# Load data
load:
    cargo run --bin load_data --features="ssr"

# Package for running on server
package: build init load
    mkdir -p app
    cp target/release/loodsenboekje app/
    cp -r target/site/ app/site/

    cp sqlite.db app/
    touch app/.env

    echo "LEPTOS_OUTPUT_NAME=leptos-loodsenboekje LEPTOS_SITE_ROOT=site LEPTOS_SITE_ADDR="0.0.0.0:1744" ./loodsenboekje" >> app/run.sh
    chmod +x app/run.sh

    # scp -r app/ server:./
    # rm -rf app/

