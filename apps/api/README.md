## Development Guide

### Required tools

1. Should at least has docker running or postgres on local to be able to integrate with database.
2. sqlx-cli, can be installed with `cargo install sqlx-cli`


### Useful Commands

To include table from the migrations, run the following command

```
cargo sqlx migrate run
```

This will automatically create the tables from `migration` directory which define the tables and whatnot.

To revert changes from migration, run the following command

```
cargo sqlx migrate revert
```

To run local postgres container, run the following command

```
docker compose up
```

Note that sqlx supports compiled time checking for SQL queries. look up on `.env.examples` make sure to fill in the URL for the database for it to work properly.