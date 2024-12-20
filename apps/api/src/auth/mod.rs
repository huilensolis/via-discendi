use std::cmp::Ordering;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use chrono::{Duration, NaiveDateTime};
use log::error;
use sqlx::{
    postgres::{PgPool, PgQueryResult},
    types::chrono::Local,
};

use crate::utils::generate_random_str;

pub mod api;
mod test;

// TODO: add config so can be defined when running the application
const DEFAULT_TOKEN_LENGTH: u8 = 128;
const DEFAULT_SESSION_DURATION_MIN: i64 = 30;

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username
            && self.email == other.email
            && self.password == other.password
            && self.name == other.name
    }
}

#[derive(PartialEq, Debug)]
pub struct UserSession {
    username: String,
    token: String,
    refresh_token: String,
    expiry_date: Option<NaiveDateTime>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

pub async fn add_user(user: &User, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    let query = sqlx::query!(
        "INSERT INTO USERS (USERNAME, EMAIL, PASSWORD, NAME) VALUES ($1, $2, $3, $4)",
        user.username,
        user.email,
        user.password,
        user.name
    )
    .execute(pool)
    .await;

    query
}

pub async fn find_user(username: &String, pool: &PgPool) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(User, "SELECT * FROM USERS WHERE USERNAME = $1", username)
        .fetch_one(pool)
        .await;

    user
}

pub async fn update_user_token(
    user_session: &UserSession,
    pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "UPDATE USERS_SESSION SET token = $1, expiry_date = $2 WHERE username = $3",
        user_session.token,
        user_session.expiry_date,
        user_session.username,
    )
    .execute(pool)
    .await
}

pub async fn find_session_by_username(
    username: &String,
    pool: &PgPool,
) -> Result<UserSession, sqlx::Error> {
    let query = sqlx::query_as!(
        UserSession,
        "SELECT * FROM USERS_SESSION WHERE USERNAME = $1",
        username
    )
    .fetch_one(pool)
    .await;
    query
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
        }
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
                    let argon = Argon2::default();
                    match argon.verify_password(password.as_bytes(), &hash_value) {
                        Ok(_) => Ok(true),
                        Err(err_message) => Err(err_message.to_string()),
                    }
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub async fn refresh_user_session(
    refresh_token: &String,
    pool: &PgPool,
) -> Result<UserSession, String> {
    let result = get_user_session_by_refresh_token(refresh_token, pool).await;

    match result {
        Ok(mut user_session) => {
            let new_token = generate_random_str(DEFAULT_TOKEN_LENGTH);
            user_session.token = new_token.clone();
            user_session.expiry_date =
                Some((Local::now() + Duration::minutes(DEFAULT_SESSION_DURATION_MIN)).naive_utc());
            if !update_user_token(&user_session, pool).await.is_ok() {
                return Err(String::from("Could not update token"));
            }
            Ok(user_session)
        }
        Err(err) => {
            error!("{}", err);
            Err(String::from("Invalid refresh token"))
        }
    }
}

pub async fn get_user_session_by_refresh_token(
    refresh_token: &String,
    pool: &PgPool,
) -> Result<UserSession, sqlx::Error> {
    let query = sqlx::query_as!(
        UserSession,
        "SELECT *  FROM USERS_SESSION WHERE REFRESH_TOKEN = $1",
        refresh_token
    )
    .fetch_one(pool)
    .await;

    query
}

pub async fn create_user_session(username: &String, pool: &PgPool) -> Result<UserSession, String> {
    let token = generate_random_str(DEFAULT_TOKEN_LENGTH);
    let refresh_token = generate_random_str(DEFAULT_TOKEN_LENGTH);
    let current_time = Local::now();

    let existing_session = find_session_by_username(username, pool).await;

    if existing_session.is_ok() {
        let session = existing_session.unwrap();
        let update_result = refresh_user_session(&session.refresh_token, pool).await;
        return update_result;
    }

    let expiry_date = Local::now() + Duration::minutes(DEFAULT_SESSION_DURATION_MIN);

    let query = sqlx::query!(
            "INSERT INTO USERS_SESSION (token, refresh_token, expiry_date, username) VALUES($1, $2, $3, $4)",
            token,
            refresh_token,
            expiry_date.naive_utc(),
            username
        )
        .execute(pool)
        .await;

    match query {
        Ok(res) => {
            if res.rows_affected() == 0 {
                return Err(String::from("Failed to create user session"));
            }
            Ok(UserSession {
                token,
                refresh_token,
                expiry_date: Some(expiry_date.naive_utc()),
                username: username.clone(),
                updated_at: Some(current_time.naive_utc()),
                created_at: Some(current_time.naive_utc()),
            })
        }
        Err(err) => Err(err.to_string()),
    }
}

pub async fn validate_session(user_session: &UserSession) -> Result<bool, String> {
    match user_session.expiry_date {
        Some(expiry_date) => {
            let current_time = Local::now().naive_utc();
            Ok(current_time.cmp(&expiry_date) == Ordering::Greater)
        }
        None => {
            panic!("Expiry date should not have been none")
        }
    }
}
