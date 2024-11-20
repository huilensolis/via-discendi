use axum::{
    body::Body,
    extract::ws::WebSocket,
    extract::{Path, Query, State},
    http::{Response, StatusCode},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use log::{error, info};

use crate::{
    router_common::{CreateResponse, RouterGlobalState},
    utils::generate_random_str,
};

use super::{add_roadmap, find_roadmap, get_roadmap_by_id, update_roadmap, Roadmaps};

#[derive(Deserialize)]
pub struct AddRoadmapRequest {
    user_id: String,
    title: String,
    description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateRoadmapRequest {
    roadmap_id: String,
    user_id: String,
    title: Option<String>,
    description: Option<String>,
}

#[derive(Deserialize)]
pub struct AddAreaRequest {
    roadmap_id: String,
    title: String,
    description: Option<String>,
    x: f64,
    y: f64,
}

#[derive(Deserialize)]
pub struct FindRoadmapParams {
    title: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct RoadmapResponse {
    id: String,
    title: String,
    description: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct AreaResponse {
    id: String,
    parent_id: Option<String>,
    title: String,
    description: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct FindRoadmapResult {
    roadmaps: Vec<RoadmapResponse>,
    page: i64,
    total: i64,
}

#[derive(Deserialize, Serialize)]
pub struct RoadmapWithAreasResponse {
    roadmap_id: String,
    roadmap_title: String,
    roadmap_description: Option<String>,
    areas: Vec<AreaResponse>,
}

const DEFAULT_ID_LENGTH: u8 = 12;
const DEFAULT_LIMIT: i64 = 10;
const DEFAULT_OFFSET: i64 = 0;

pub async fn add_roadmap_router(
    State(router_global_state): State<RouterGlobalState>,
    Json(request): Json<AddRoadmapRequest>,
) -> Response<Body> {
    let roadmap = Roadmaps {
        id: generate_random_str(DEFAULT_ID_LENGTH),
        title: request.title,
        description: request.description,
        publisher: request.user_id,
        published: None,
        created_at: None,
        updated_at: None,
    };

    let roadmap_added = match add_roadmap(roadmap, &router_global_state.pool).await {
        Ok(result) => result.rows_affected() == 1,
        Err(err) => {
            error!(
                "[roadmap][add_roadmap] error on adding roadmap with cause {}",
                err
            );
            false
        }
    };

    if !roadmap_added {
        let response = serde_json::to_string(&CreateResponse {
            is_successful: false,
            message: String::from("Fail on adding roadmap please try again"),
        })
        .unwrap();

        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(response))
            .unwrap();
    }

    let response = serde_json::to_string(&CreateResponse {
        is_successful: true,
        message: String::from("Roadmap successfully added"),
    })
    .unwrap();

    return Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(response))
        .unwrap();
}

pub async fn update_roadmap_router(
    State(router_global_state): State<RouterGlobalState>,
    Json(request): Json<UpdateRoadmapRequest>,
) -> Response<Body> {
    let roadmap = Roadmaps {
        id: request.roadmap_id,
        title: request.title.unwrap_or("".to_string()),
        description: request.description,
        publisher: request.user_id,
        published: None,
        created_at: None,
        updated_at: Some(Utc::now().naive_utc()),
    };

    let update_result = match update_roadmap(roadmap, &router_global_state.pool).await {
        Ok(result) => result.rows_affected() == 1,
        Err(err) => {
            error!(
                "[roadmap][add_roadmap] error on adding roadmap with cause {}",
                err
            );
            false
        }
    };

    if !update_result {
        let response = serde_json::to_string(&CreateResponse {
            is_successful: false,
            message: String::from("Fail on updating roadmap please try again"),
        })
        .unwrap();

        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(response))
            .unwrap();
    }

    let response = serde_json::to_string(&CreateResponse {
        is_successful: true,
        message: String::from("Roadmap successfully updated"),
    })
    .unwrap();

    return Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(response))
        .unwrap();
}

pub async fn find_roadmap_router(
    State(router_global_state): State<RouterGlobalState>,
    Query(pagination): Query<FindRoadmapParams>,
) -> Response<Body> {
    let limit = pagination.limit.unwrap_or(DEFAULT_LIMIT);
    let offset = pagination.offset.unwrap_or(DEFAULT_OFFSET);
    let title = pagination.title.unwrap_or("".to_string());

    match find_roadmap(title, limit, offset, &router_global_state.pool).await {
        Ok(result) => {
            let mut roadmap_responses: Vec<_> = Vec::new();

            for roadmap in result {
                roadmap_responses.push(RoadmapResponse {
                    id: roadmap.id,
                    title: roadmap.title,
                    description: roadmap.description,
                });
            }

            let response = serde_json::to_string(&FindRoadmapResult {
                roadmaps: roadmap_responses,
                page: offset,
                total: limit,
            })
            .unwrap();

            return Response::builder()
                .status(StatusCode::OK)
                .body(Body::from(response))
                .unwrap();
        }
        Err(err) => {
            error!(
                "[roadmap][find_roadmap] error on finding roadmap with cause {}",
                err
            );

            let response = serde_json::to_string(&CreateResponse {
                is_successful: false,
                message: String::from("Fail on finding roadmap please try again"),
            })
            .unwrap();

            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(response))
                .unwrap();
        }
    }
}

pub async fn get_roadmap_detail_router(
    State(router_global_state): State<RouterGlobalState>,
    Path(roadmap_id): Path<String>,
) -> Response<Body> {
    match get_roadmap_by_id(roadmap_id.clone(), &router_global_state.pool).await {
        Ok(result) => {
            let mut area_responses: Vec<AreaResponse> = Vec::new();

            for area in result.areas {
                area_responses.push(AreaResponse {
                    id: area.id,
                    parent_id: area.parent_id,
                    title: area.title,
                    description: area.description,
                });
            }

            let roadmap_with_area = RoadmapWithAreasResponse {
                roadmap_id: result.roadmap_id,
                roadmap_title: result.roadmap_title,
                roadmap_description: result.roadmap_description,
                areas: area_responses,
            };

            let response = serde_json::to_string(&roadmap_with_area).unwrap();

            return Response::builder()
                .status(StatusCode::OK)
                .body(Body::from(response))
                .unwrap();
        }
        Err(err) => {
            error!(
                "[roadmap][get_roadmap_by_id] error on finding roadmap {} with cause {}",
                roadmap_id, err
            );

            let response = serde_json::to_string(&CreateResponse {
                is_successful: false,
                message: String::from("Fail on finding roadmap please try again"),
            })
            .unwrap();

            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(response))
                .unwrap();
        }
    }
}

//TODO: add handler for handling different type of update on the area be it for updating or
//creating area
pub async fn roadmap_area_websocket(
    mut socket: WebSocket,
    State(router_global_state): State<RouterGlobalState>,
    Path(roadmap_id): Path<String>,
) {
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(message) => {
                let res: Result<AddAreaRequest, serde_json::Error> =
                    serde_json::from_str(message.to_text().unwrap_or(""));

                match res {
                    Ok(request) => {}
                    Err(err) => {
                        error!("[roadmap_area_websocket][add_area] Failed adding area");
                        continue;
                    }
                }
            }
            Err(err) => {
                info!("User disconnect");
                return;
            }
        }
    }
}
