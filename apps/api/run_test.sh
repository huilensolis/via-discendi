DATABASE_URL=postgres://myuser:mypassword@localhost/test_database cargo sqlx migrate revert
DATABASE_URL=postgres://myuser:mypassword@localhost/test_database cargo sqlx migrate run
cargo test
DATABASE_URL=postgres://myuser:mypassword@localhost/test_database cargo sqlx migrate revert
