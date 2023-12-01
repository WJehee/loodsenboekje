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

# Prepare server for deploy
prepare: init load
    cp sqlite.db loodsenboekje/
    touch loodsenboekje/.env

# Deploy to server
deploy: build 
    mkdir -p loodsenboekje
    cp target/release/loodsenboekje loodsenboekje/
    cp -r target/site/ loodsenboekje/site/

    echo "LEPTOS_OUTPUT_NAME=leptos-loodsenboekje LEPTOS_SITE_ROOT=site LEPTOS_SITE_ADDR="0.0.0.0:1744" ./loodsenboekje" >> loodsenboekje/run.sh
    chmod +x loodsenboekje/run.sh

    scp -r loodsenboekje/ server:./
    rm -rf loodsenboekje/


