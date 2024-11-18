use chrono::NaiveDateTime;
use sqlx;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;

pub mod api;

#[derive(Debug)]
pub struct Roadmaps {
    // NOTE: both id and version makes the unique identifier for the roadmap
    id: String,
    title: String,
    description: Option<String>,
    publisher: String,
    published: Option<bool>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

pub struct UserRoadmaps {
    user_id: String,
    roadmap_id: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

pub struct UserAreas {
    user_roadmap_id: String,
    completed: bool,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct RoadmapLikes {
    user_id: String,
    roadmap_id: String,

    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct RoadmapBookmarkers {
    user_id: String,
    roadmap_id: String,

    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct Areas {
    id: String,
    parent_id: String,
    roadmap_id: String,
    title: String,
    description: String,
    things_to_learn: Vec<String>,

    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

async fn add_roadmap(roadmap: &Roadmaps, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    let query = sqlx::query!(
        r#"
            INSERT INTO ROADMAPS 
                (ID, TITLE, DESCRIPTION, PUBLISHER)
            VALUES
                ($1, $2, $3, $4)
        "#,
        roadmap.id,
        roadmap.title,
        roadmap.description,
        roadmap.publisher
    )
    .execute(pool)
    .await;

    query
}

async fn update_roadmap(roadmap: &Roadmaps, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    let query = sqlx::query!(
        r#"
            UPDATE ROADMAPS
                SET TITLE = $1,
                DESCRIPTION = $2,
                PUBLISHED = $3,
                UPDATED_AT = NOW()
            WHERE 
                ID = $4;
        "#,
        roadmap.title,
        roadmap.description,
        roadmap.published,
        roadmap.id
    )
    .execute(pool)
    .await;

    query
}

async fn delete_roadmap(roadmap_id: String, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    let query = sqlx::query!(
        r#"
            DELETE FROM ROADMAPS WHERE ID = $1
        "#,
        roadmap_id
    )
    .execute(pool)
    .await;

    query
}

//TODO: for now using like is fine i guess but try to migrate it to full text search later on
async fn find_roadmap(
    roadmap_name: String,
    limit: i64,
    offset: i64,
    pool: &PgPool,
) -> Result<Vec<Roadmaps>, sqlx::Error> {
    let query = sqlx::query_as!(
        Roadmaps,
        r#"
        SELECT 
            ID,
            TITLE,
            DESCRIPTION,
            PUBLISHER,
            PUBLISHED,
            CREATED_AT,
            UPDATED_AT
        FROM
            ROADMAPS
        WHERE TITLE ILIKE $1
        LIMIT $2
        OFFSET $3
    "#,
        format!("%{}%", roadmap_name),
        limit,
        offset
    )
    .fetch_all(pool)
    .await;

    query
}

async fn get_user_roadmaps(user_id: String, pool: &PgPool) -> Result<Vec<Roadmaps>, sqlx::Error> {
    let query = sqlx::query_as!(
        Roadmaps,
        r#"
            SELECT * FROM ROADMAPS WHERE PUBLISHER = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await;

    query
}

async fn get_roadmaps(
    offset: i64,
    limit: i64,
    pool: &PgPool,
) -> Result<Vec<Roadmaps>, sqlx::Error> {
    let query = sqlx::query_as!(
        Roadmaps,
        r#"
        SELECT 
            ID,
            TITLE,
            DESCRIPTION,
            PUBLISHER,
            PUBLISHED,
            CREATED_AT,
            UPDATED_AT
        FROM
            ROADMAPS
        LIMIT $1
        OFFSET $2
    "#,
        limit,
        offset
    )
    .fetch_all(pool)
    .await;

    query
}

async fn add_areas(area: &Areas, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    let query = sqlx::query!(
        r#"
            INSERT INTO ROADMAP_AREAS
                (ID, PARENT_ID, TITLE, DESCRIPTION)
            VALUES
                ($1, $2, $3, $4)
        "#,
        area.id,
        area.parent_id,
        area.title,
        area.description
    )
    .execute(pool)
    .await;

    query
}

async fn update_areas(area: &Areas, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    let query = sqlx::query!(
        r#"
            UPDATE ROADMAP_AREAS
            SET 
                parent_id = $1,
                title = $2,
                description = $3
                WHERE 
                    id = $4
        "#,
        area.parent_id,
        area.title,
        area.description,
        area.id
    )
    .execute(pool)
    .await;

    query
}

//TODO: probably use soft delete
async fn delete_area(area_id: String, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    todo!()
}

async fn like_roadmap(roadmap_id: String, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    todo!()
}

async fn get_user_bookmarked_roadmaps(
    user_id: String,
    pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    todo!()
}
