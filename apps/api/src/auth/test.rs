#[cfg(test)]
mod tests {

    use core::panic;
    use std::cmp::Ordering;
    use chrono::{Duration, Local};
    use sqlx::{migrate::Migrator, postgres::PgPoolOptions};


    use crate::auth::{add_user, create_user_session, find_user, login, sign_up, User, DEFAULT_SESSION_DURATION_MIN};

    #[tokio::test]
    async fn test_add_user() {

        let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://myuser:mypassword@localhost/mydatabase").await.unwrap();

        let m = Migrator::new(std::path::Path::new("./migrations")).await.unwrap();
        m.run(&pool).await.unwrap();
        
        let mock_user = User{
            username: String::from("test"),
            email: String::from("test@gmail.com"),
            password: String::from("unhashed"),
            name: String::from("untested"),
            created_at: Some(Local::now().naive_utc()),
            updated_at: Some(Local::now().naive_utc()),
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

        m.undo(&pool, 1).await.unwrap();
    }

    #[tokio::test]
    async fn test_find_user() {
        let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://myuser:mypassword@localhost/mydatabase").await.unwrap();

        let m = Migrator::new(std::path::Path::new("./migrations")).await.unwrap();
        m.run(&pool).await.unwrap();
        
        let mock_user = User{
            username: String::from("test2"),
            email: String::from("test2@gmail.com"),
            password: String::from("unhashed"),
            name: String::from("untested"),
            created_at: Some(Local::now().naive_utc()),
            updated_at: Some(Local::now().naive_utc()),
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

        m.undo(&pool, 1).await.unwrap();
    }

    #[tokio::test]
    async fn test_sign_up() {
        let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://myuser:mypassword@localhost/mydatabase").await.unwrap();

        let m = Migrator::new(std::path::Path::new("./migrations")).await.unwrap();
        m.run(&pool).await.unwrap();

        let mut mock_user = User{
            username: String::from("test3"),
            email: String::from("test3@gmail.com"),
            password: String::from("unhashed"),
            name: String::from("untested"),
            created_at: Some(Local::now().naive_utc()),
            updated_at: Some(Local::now().naive_utc()),
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

        m.undo(&pool, 1).await.unwrap();
    }

    #[tokio::test]
    async fn test_login() {
        let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://myuser:mypassword@localhost/mydatabase").await.unwrap();

        let m = Migrator::new(std::path::Path::new("./migrations")).await.unwrap();
        m.run(&pool).await.unwrap();

        let mut mock_user = User{
            username: String::from("test5"),
            email: String::from("test5@gmail.com"),
            password: String::from("unhashed"),
            name: String::from("untested"),
            created_at: Some(Local::now().naive_utc()),
            updated_at: Some(Local::now().naive_utc()),
        };

        sign_up(&mut mock_user, &pool).await.unwrap();

        let result = login(&String::from("test3"), &String::from("unhashed"), &pool).await;

        match result {
            Ok(_) => {},
            Err(err) => panic!("{}", err),
        }

        m.undo(&pool, 1).await.unwrap();
    }

    #[tokio::test] 
    async fn test_create_user_session() {
        let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://myuser:mypassword@localhost/mydatabase").await.unwrap();

        let m = Migrator::new(std::path::Path::new("./migrations")).await.unwrap();
        m.run(&pool).await.unwrap();

        let mut mock_user = User{
            username: String::from("test4"),
            email: String::from("test4@gmail.com"),
            password: String::from("unhashed"),
            name: String::from("untested"),
            created_at: Some(Local::now().naive_utc()),
            updated_at: Some(Local::now().naive_utc()),
        };

        sign_up(&mut mock_user, &pool).await.unwrap();

        let result = create_user_session(&mock_user.username, &pool).await;

        match result {
            Ok(created_session) => {
                // minus one minutes because some milliseconds delay making sure it always be in range 30 minutes
                let current_time = Local::now() + Duration::minutes(DEFAULT_SESSION_DURATION_MIN) - Duration::minutes(1);
                assert!(created_session.expiry_date.unwrap().cmp(&current_time.naive_utc()) == Ordering::Greater)
            },
            Err(err) => panic!("{}", err.to_string()),
        }

        m.undo(&pool, 1).await.unwrap();
    }

}