
use chrono::{DateTime, Utc};
use rand::{distributions::Alphanumeric, Rng};
use sqlx::postgres::{PgPool, PgQueryResult};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher,  SaltString
    }, Argon2, PasswordHash, PasswordVerifier
};

mod test;
mod api;

const DEFAULT_TOKEN_LENGTH: u8 = 32;

#[derive(PartialEq, Debug)]
pub struct User {
    username: String,
    email: String,
    password: String,
    name: String
}

pub struct UserSession {
    token: String,
    refresh_token: String,
    expiry_date: DateTime<Utc>, 
    username: String
}

async fn add_user(user: &User, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    let query = sqlx::query!(
        "INSERT INTO users (username, email, password, name) VALUES ($1, $2, $3, $4)",
        user.username,
        user.email,
        user.password,
        user.name
    )
    .execute(pool)
    .await;

    query
}

async fn find_user(username: &String, pool: &PgPool) -> Result<User, sqlx::Error>{
    let user = sqlx::query_as!(
        User,
        "SELECT * from USERS WHERE username = $1",
        username
    )
    .fetch_one(pool)
    .await;

    user
}

fn generate_token(length: u8) -> String {
        let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)  // generates random alphanumeric characters
        .take(length.into())                     // take n characters
        .map(char::from)             // map them to characters
        .collect();                  // collect into a String

    random_string
}

//TODO: Add logger for knowing what happen to the code
pub async fn sign_up(user: &mut User, pool: &PgPool) -> Result<bool, String> {

    let password = user.password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password, &salt);

    match password_hash {
        Ok(hashhed_password) => {
            user.password = hashhed_password.to_string();
            let result = add_user(&user, pool).await;

            match result {
                Ok(_) => Ok(true),
                Err(err) => Err(err.to_string()),
            }
        },
        Err(err) => Err(err.to_string()),
    }
}

pub async fn login(username: &String, password: &String, pool: &PgPool) -> Result<bool, String> {

    let find_result = find_user(&username, pool).await;

    match find_result {
        Ok(user) => {
            let hash = PasswordHash::new(&user.password);
            match hash {
                Ok(hash_value) => {
                    let argon= Argon2::default();
                    Ok(argon.verify_password(password.as_bytes(), &hash_value).is_ok())
                },
                Err(err) => Err(err.to_string()),
            }
        },
        Err(err) => Err(err.to_string()),
    }
}

pub async fn create_user_session(username: &String, pool: &PgPool) -> Result<bool, String> {
    todo!()
}

pub async fn refresh_user_session() {
    todo!()
}

pub async fn validate_session(userSession: &UserSession) -> Option<bool> {
    todo!()
}
