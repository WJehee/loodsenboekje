# Loodsenboekje.com

Website to keep track of the ways a beer has been opened.

Favicon generated with: https://favicon.io/emoji-favicons/

# Preparing to run

Create a .env file with the following values:

- READER_PASSWORD
- WRITER_PASSWORD

## Make sure sqlx-cli is installed
```
cargo install sqlx-cli
```

## Setup database
```
export DATABASE_URL="sqlite://sqlite.db"
cargo sqlx db create
cargo sqlx migrate run
```

# Adding a migration
```
cargo sqlx migrate add <name>
```

