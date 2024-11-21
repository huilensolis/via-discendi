use chrono::NaiveDateTime;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;
use sqlx::{self, query};

pub mod api;
mod test;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Areas {
    id: String,
    parent_id: Option<String>,
    roadmap_id: String,
    title: String,
    description: Option<String>,

    x: f64,
    y: f64,

    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

pub struct RoadmapWithAreas {
    roadmap_id: String,
    roadmap_title: String,
    roadmap_description: Option<String>,
    areas: Vec<Areas>,
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

async fn add_roadmap(roadmap: Roadmaps, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    let query = query!(
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

async fn update_roadmap(roadmap: Roadmaps, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    let query = query!(
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

//TODO: for now using like is fine i guess but try to migrate it to full text search later on
async fn find_roadmap(
    roadmap_title: String,
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
        WHERE 
            (TITLE = '' OR TITLE ILIKE $1) AND
            published = true
        LIMIT $2
        OFFSET $3
    "#,
        format!("%{}%", roadmap_title),
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
        ORDER BY CREATED_AT
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

async fn roadmap_exist(roadmap_id: String, pool: &PgPool) -> Result<bool, sqlx::Error> {
    let query = query!(
        r#"SELECT EXISTS (SELECT 1 FROM ROADMAPS WHERE ID = $1)"#,
        roadmap_id
    )
    .fetch_one(pool)
    .await;

    match query {
        Ok(result) => Ok(result.exists.unwrap_or(false)),
        Err(err) => Err(err),
    }
}

async fn get_roadmap_by_id(
    roadmap_id: String,
    pool: &PgPool,
) -> Result<RoadmapWithAreas, sqlx::Error> {
    match query!(
        r#"
        SELECT 
            r.id as roadmap_id,
            r.title as roadmap_title, 
            r.description as roadmap_description,
            ra.id as "area_id?",
            ra.title as "area_title?",
            ra.description as "area_description?",
            ra.parent_id as "area_parent_id?",
            ra.x as "x?", 
            ra.y as "y?"
        FROM ROADMAPS R 
        LEFT JOIN
            ROADMAP_AREAS RA ON R.ID = RA.ROADMAP_ID
        WHERE 
            R.ID = $1
        "#,
        roadmap_id
    )
    .fetch_all(pool)
    .await
    {
        Ok(records) => {
            let mut roadmap_id = "".to_string();
            let mut roadmap_title = "".to_string();
            let mut roadmap_description = None;
            let mut areas: Vec<_> = Vec::new();
            for record in records {
                let area = Areas {
                    id: record.area_id.unwrap_or("".to_string()),
                    title: record.area_title.unwrap_or("".to_string()),
                    description: record.area_description,
                    parent_id: record.area_parent_id,
                    roadmap_id: record.roadmap_id.clone(),
                    x: record.x.unwrap_or(0.0),
                    y: record.y.unwrap_or(0.0),
                    created_at: None,
                    updated_at: None,
                };
                areas.push(area);
                roadmap_id = record.roadmap_id.clone();
                roadmap_title = record.roadmap_title;
                roadmap_description = record.roadmap_description;
            }

            return Ok(RoadmapWithAreas {
                roadmap_id,
                roadmap_description,
                roadmap_title,
                areas,
            });
        }
        Err(err) => {
            println!("{}", err);
            return Err(err);
        }
    }
}

async fn add_areas(area: Areas, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    let query = query!(
        r#"
            INSERT INTO ROADMAP_AREAS
                (ID, PARENT_ID, TITLE, DESCRIPTION, ROADMAP_ID, X, Y)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7)
        "#,
        area.id,
        area.parent_id,
        area.title,
        area.description,
        area.roadmap_id,
        area.x,
        area.y,
    )
    .execute(pool)
    .await;

    query
}

async fn update_areas(area: Areas, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    let query = query!(
        r#"
            UPDATE ROADMAP_AREAS
            SET 
                parent_id = $1,
                title = $2,
                description = $3,
                X = $5,
                Y = $6
                WHERE 
                    id = $4
        "#,
        area.parent_id,
        area.title,
        area.description,
        area.id,
        area.x,
        area.y
    )
    .execute(pool)
    .await;

    query
}

//TODO: probably use soft delete
async fn delete_area(area_id: String, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    let query = query!(
        r#"
        DELETE FROM ROADMAP_AREAS WHERE ID = $1
    "#,
        area_id
    )
    .execute(pool)
    .await;

    query
}

async fn delete_roadmap(roadmap_id: String, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    let query = query!(
        r#"
            DELETE FROM ROADMAPS WHERE ID = $1
        "#,
        roadmap_id
    )
    .execute(pool)
    .await;

    query
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
