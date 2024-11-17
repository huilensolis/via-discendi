use chrono::NaiveDateTime;

pub mod api;

#[derive(Debug)]
pub struct Roadmap {
    tile: String,
    description: String,
    like_count: u32,
    author_id: string,

    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}
