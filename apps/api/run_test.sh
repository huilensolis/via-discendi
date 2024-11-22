
#!/bin/bash

# Check for load test flag
TEST_FLAGS=""
if [ "$1" == "load" ]; then
   TEST_FLAGS="--ignored"
fi

DATABASE_URL=postgres://myuser:mypassword@localhost/test_database cargo sqlx migrate revert
DATABASE_URL=postgres://myuser:mypassword@localhost/test_database cargo sqlx migrate run
cargo test  -- --nocapture $TEST_FLAGS
DATABASE_URL=postgres://myuser:mypassword@localhost/test_database cargo sqlx migrate revert
