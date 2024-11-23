use axum::{
    body::Body,
    extract::{ws::WebSocket, Path, Query, State, WebSocketUpgrade},
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

use super::{
    add_areas, add_roadmap, find_roadmap, get_roadmap_by_id, roadmap_exist, update_areas,
    update_roadmap, Areas, Roadmaps,
};

#[derive(Deserialize)]
pub struct AddRoadmapRequest {
    user_id: String,
    title: String,
    description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateRoadmapRequest {
    pub roadmap_id: String,
    pub user_id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct UpsertAreaRequest {
    pub area_id: Option<String>,
    pub parent_id: Option<String>,
    pub title: String,
    pub description: Option<String>,

    pub x: f64,
    pub y: f64,
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
    let roadmap_id = generate_random_str(DEFAULT_ID_LENGTH);
    let roadmap = Roadmaps {
        id: roadmap_id.to_string(),
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
            id: None,
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
        id: Some(roadmap_id),
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
        id: request.roadmap_id.to_string(),
        title: request.title,
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
            id: None,
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
        id: Some(request.roadmap_id),
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
                id: None,
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
                id: None,
            })
            .unwrap();

            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(response))
                .unwrap();
        }
    }
}

//TODO: @Performance, this would probably will be slow near future. it might have been better to
//just update all record at once?
// this will be really slow if the update is too much, one way to optimize this is to
// 1. put tick rate like in video game, in this case for every 1 or 3 seconds might be fine.
// 2. decide whether we should use buffered approach or temporary table or let it be for now.
// 3. on the client, it's mandatory to optimize for only sending areas that only changes, don't
//    send data that dont change to reduce load on the network
// 4. Using compression for compressing message or a better protocol on the websocket instead of
//    using json, but for now we let it be.
// 5. don't allow for client to connect on the same roadmap on different socket, it will cause
//    complicated conflict resolution
async fn roadmap_area_websocket(
    mut socket: WebSocket,
    router_global_state: RouterGlobalState,
    roadmap_id: String,
) {
    match roadmap_exist(roadmap_id.clone(), &router_global_state.pool).await {
        Ok(value) => {
            println!("{} exits? {}", roadmap_id.clone(), value);
            if !value {
                info!("[roadmap_area_websocket] attempted to modify non existing roadmap");
                let response = CreateResponse {
                    is_successful: false,
                    message: "Fail on connecting the editing the roadmap, please try again"
                        .to_string(),
                    id: None,
                };
                let _ = socket
                    .send(axum::extract::ws::Message::Text(
                        serde_json::to_string(&response).unwrap(),
                    ))
                    .await;
                return;
            }
        }
        Err(err) => {
            error!("[roadmap_area_websocket]{}", err);
            let response = CreateResponse {
                is_successful: false,
                message: "Fail on connecting the editing the roadmap, please try again".to_string(),
                id: None,
            };
            let _ = socket
                .send(axum::extract::ws::Message::Text(
                    serde_json::to_string(&response).unwrap(),
                ))
                .await;
            return;
        }
    }

    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(message) => {
                let message_text = message.to_text();
                let upsert_area_request: Result<UpsertAreaRequest, _> =
                    serde_json::from_str(message_text.unwrap());

                match upsert_area_request {
                    Ok(request) => match request.area_id {
                        Some(area_id) => {
                            let area = Areas {
                                id: area_id,
                                parent_id: request.parent_id,
                                roadmap_id: roadmap_id.to_string(),
                                title: request.title,
                                description: request.description,
                                x: request.x,
                                y: request.y,
                                created_at: None,
                                updated_at: Some(Utc::now().naive_utc()),
                            };
                            match update_areas(area, &router_global_state.pool).await {
                                Ok(_) => {
                                    let response = serde_json::to_string(&CreateResponse {
                                        is_successful: true,
                                        message: String::from("Area updated"),
                                        id: None,
                                    })
                                    .unwrap();

                                    if let Err(err) = socket
                                        .send(axum::extract::ws::Message::Text(response))
                                        .await
                                    {
                                        error!("[roadmap_area_websocket] {}", err);
                                    }
                                }
                                Err(err) => {
                                    error!("[roadmap_area_websocket] {}", err);

                                    let response = serde_json::to_string(&CreateResponse {
                                        is_successful: false,
                                        message: String::from(
                                            "Fail updating area please try again",
                                        ),
                                        id: None,
                                    })
                                    .unwrap();

                                    if let Err(err) = socket
                                        .send(axum::extract::ws::Message::Text(response))
                                        .await
                                    {
                                        error!("[roadmap_area_websocket] {}", err);
                                    }
                                }
                            }
                        }
                        None => {
                            let area_id = generate_random_str(DEFAULT_ID_LENGTH);
                            let area = Areas {
                                id: area_id.to_string(),
                                parent_id: request.parent_id,
                                roadmap_id: roadmap_id.to_string(),
                                title: request.title,
                                description: request.description,
                                x: request.x,
                                y: request.y,
                                created_at: None,
                                updated_at: Some(Utc::now().naive_utc()),
                            };

                            match add_areas(area, &router_global_state.pool).await {
                                Ok(_) => {
                                    let response = serde_json::to_string(&CreateResponse {
                                        is_successful: true,
                                        message: String::from("Area created"),
                                        id: Some(area_id),
                                    })
                                    .unwrap();

                                    if let Err(err) = socket
                                        .send(axum::extract::ws::Message::Text(response))
                                        .await
                                    {
                                        error!("[roadmap_area_websocket] {}", err);
                                    }
                                }
                                Err(err) => {
                                    error!("[roadmap_area_websocket] {}", err);

                                    let response = serde_json::to_string(&CreateResponse {
                                        is_successful: false,
                                        message: String::from(
                                            "Fail creating area please try again",
                                        ),
                                        id: None,
                                    })
                                    .unwrap();

                                    if let Err(err) = socket
                                        .send(axum::extract::ws::Message::Text(response))
                                        .await
                                    {
                                        error!("[roadmap_area_websocket] {}", err);
                                    }
                                }
                            }
                        }
                    },
                    Err(err) => {
                        error!(
                            "[roadmap_area_websocket] fail on parsing request with cause {}",
                            err
                        );

                        let response = serde_json::to_string(&CreateResponse {
                            is_successful: false,
                            message: String::from("Fail creating area please try again"),
                            id: None,
                        })
                        .unwrap();

                        if let Err(err) = socket
                            .send(axum::extract::ws::Message::Text(response))
                            .await
                        {
                            error!("[roadmap_area_websocket] {}", err);
                        }

                        continue;
                    }
                }
            }
            Err(_err) => {
                continue;
            }
        }
    }
}

pub async fn area_websocket_router(
    ws: WebSocketUpgrade,
    State(router_global_state): State<RouterGlobalState>,
    Path(roadmap_id): Path<String>,
) -> Response<Body> {
    ws.on_upgrade(|socket| roadmap_area_websocket(socket, router_global_state, roadmap_id))
}
