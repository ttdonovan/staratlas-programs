# Star Atlas: Fleet Rentals

* [SRSLY Program IDL Documentation](https://staratlas.notion.site/SRSLY-Program-IDL-Documentation-1cec619086e9808ebefbd06b0821dc6f)

```bash
cargo install sqlx-cli
export DATABASE_URL='sqlite:tmp/db.sqlite?mode=rwc&cache=shared'
sqlx migrate run --source migrations
cargo run --example fleet-rentals-01
cargo run --example fleet-rentals-02
```

## DuckDB

```bash
$ duckdb
D INSTALL sqlite3;
D LOAD sqlite3;
D ATTACH 'tmp/db.sqlite' AS db (TYPE sqlite3);
D .read scripts/rentals_csv.sql
```

```sql
SELECT
    fleet,
    fleet_rental_rate,
    faction,
    fleet_ship_counts,
    fleet_label,
    ship_index,
    ship_name,
    ship_count
FROM
    'tmp/rentals.csv'
WHERE
    fleet IN (
        SELECT DISTINCT
            (fleet)
        FROM
            'tmp/rentals.csv'
        WHERE
            ship_name = '...'
    )
```
