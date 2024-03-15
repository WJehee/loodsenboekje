
_default:
    just --list

# Leptos watch
watch:
    cargo leptos watch

# Initialize project
init: setup-env
    cargo sqlx db create
    cargo sqlx migrate run

# Setup environment variables in .env for local development
setup-env:
    #!/usr/bin/env bash
    rm .env
    touch .env
    p1=$(openssl rand -base64 32)
    p2=$(openssl rand -base64 32)
    p3=$(openssl rand -base64 32)
    echo "READ_PASSWORD=$p1" >> .env
    echo "WRITE_PASSWORD=$p2" >> .env
    echo "ADMIN_PASSWORD=$p3" >> .env

# Build for release
build:
    cargo leptos build --release -vv

# Load data
load:
    cargo run --bin load_data --features="ssr"

