DATABASE_URL=postgres://myuser:mypassword@localhost/test_database cargo sqlx migrate run
cargo test
cargo sqlx migrate revert
