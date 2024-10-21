use sqlx::{query, query_as, Error};
use sqlx::postgres::{PgPool, PgQueryResult};

mod test;

pub struct User {
    username: String,
    email: String,
    password: String,
    name: String
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username && self.email == other.email && self.password == other.password && self.name == other.name
    }
}

pub async fn add_user(user: &User, pool: &PgPool) -> Result<PgQueryResult, Error> {
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

pub async fn find_user(username: &String, pool: &PgPool) -> Result<User, Error>{
    let user = sqlx::query_as!(
        User,
        "select * from users where username = $1",
        username
    )
    .fetch_one(pool)
    .await;

    user
}
