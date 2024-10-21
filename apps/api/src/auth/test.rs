#[cfg(test)]
mod tests {
    use core::panic;

    use sqlx::{postgres::PgPoolOptions, PgPool};

    use crate::auth::{add_user, find_user, User};

    #[tokio::test]
    async fn test_add_user() {


        let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://myuser:mypassword@localhost/mydatabase").await.unwrap();

        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        
        let mock_user = User{
            username: String::from("test"),
            email: String::from("test@gmail.com"),
            password: String::from("unhashed"),
            name: String::from("untested")
        };

        let result = add_user(&mock_user, &pool).await;
        match result {
            Ok(value) => {
                assert!(value.rows_affected() ==  1); 
            },
            Err(err) => {
                panic!("Failed inserting mock user {}", err);
            },
        }

    }

    #[tokio::test]
    async fn test_find_user() {
        let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://myuser:mypassword@localhost/mydatabase").await.unwrap();

        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        
        let mock_user = User{
            username: String::from("test2"),
            email: String::from("test2@gmail.com"),
            password: String::from("unhashed"),
            name: String::from("untested")
        };

        add_user(&mock_user, &pool).await.unwrap();

        let result = find_user(&mock_user.username, &pool).await;

        match result {
            Ok(value) => {
                assert!(value.eq(&mock_user))
            },
            Err(err) => {
                panic!("Failed finding user with username {} with  root cause {}", mock_user.username, err);
            },
        }

    }


}