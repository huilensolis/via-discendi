#[cfg(test)]
mod tests {

    use core::panic;
    use sqlx::postgres::PgPoolOptions;

    use crate::auth::{add_user, find_user, login, sign_up, User};

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

    #[tokio::test]
    async fn test_sign_up() {
        let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://myuser:mypassword@localhost/mydatabase").await.unwrap();

        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        let mut mock_user = User{
            username: String::from("test2"),
            email: String::from("test2@gmail.com"),
            password: String::from("unhashed"),
            name: String::from("untested")
        };

        let result = sign_up(&mut mock_user, &pool).await;

        match result {
            Ok(value) => {
                if !value {
                    panic!("Should not return false if no error occured")
                }
            },
            Err(err) => {
                panic!("Failed signing up account with cause {}", err)
            },
        }
    }

    #[tokio::test]
    async fn test_login() {
        let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://myuser:mypassword@localhost/mydatabase").await.unwrap();

        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        let mut mock_user = User{
            username: String::from("test3"),
            email: String::from("test3@gmail.com"),
            password: String::from("unhashed"),
            name: String::from("untested")
        };

        sign_up(&mut mock_user, &pool).await.unwrap();

        let result = login(&String::from("test3"), &String::from("unhashed"), &pool).await;

        match result {
            Ok(_) => {},
            Err(err) => panic!("{}", err),
        }
    }


}