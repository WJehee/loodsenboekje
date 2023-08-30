# Loodsenboekje.com

## Setup database
```
export DATABASE_URL="sqlite://sqlite.db"
cargo sqlx db create
cargo sqlx migrate run
```

## Adding a migration
```
cargo sqlx migrate add <name>
```

