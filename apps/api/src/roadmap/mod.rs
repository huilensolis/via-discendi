use chrono::NaiveDateTime;
use sqlx;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;

pub mod api;

#[derive(Debug)]
pub struct Roadmaps {
    // NOTE: both id and version makes the unique identifier for the roadmap
    id: String,
    version: String,
    tile: String,
    description: String,
    author_id: String,
    published: bool, //NOTE: indicates whether it can be seen by other user
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
    todo!()
}

async fn update_roadmap(roadmap: &Roadmaps, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    todo!()
}

async fn delete_roadmap(roadmap_id: String, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    todo!()
}

async fn find_roadmap(roadmap_name: String, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
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

async fn get_user_roadmaps(user_id: String, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    todo!();
}

async fn get_roadmaps(
    offset: u32,
    limit: u32,
    pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    todo!();
}

async fn add_areas(area: &Areas, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    todo!()
}

async fn update_areas(area: &Areas, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    todo!()
}

async fn delete_area(area_id: String, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    todo!()
}
