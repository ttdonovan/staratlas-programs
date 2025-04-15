# Star Atlas: Fleet Rentals

* [SRSLY Program IDL Documentation](https://staratlas.notion.site/SRSLY-Program-IDL-Documentation-1cec619086e9808ebefbd06b0821dc6f)

```bash
cargo install sqlx-cli
export DATABASE_URL=sqlite:tmp/db.sqlite?mode=rwc
sqlx migrate run --source migrations
cargo run --example fleet-rentals-01
cargo run --example fleet-rentals-02
```

## DuckDB

```bash
$ duckdb
D INSTALL sqlite3;
D LOAD sqlite3;
D ATTACH 'tmp/db.sqlite' AS sqlite_db (TYPE sqlite3);
```

```
.read scripts/rentals.sql
COPY (...) TO 'tmp/rentals.csv' (HEADER, DELIMITER ',');
```
