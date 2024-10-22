
use axum::{extract::State, http::{header, Response, StatusCode}, Json};
use crate::router_config::{CreateResponse, RouterGlobalState};

use super::{create_user_session, login, DEFAULT_SESSION_DURATION_MIN};

pub struct LoginRequest {
    username: String,
    password: String
}

pub async fn login_router(
    State(RouterGlobalState {pool,..}): State<RouterGlobalState>,
    Json(request): Json<LoginRequest>
) -> Response<CreateResponse> {

    let result = login(&request.username, &request.password, &pool).await;
    
    match result {
        Ok(login_succesful) => {
            if !login_succesful {
                return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(
                    CreateResponse { 
                        is_successful: false,
                        message: String::from("Invalid username or password")
                        }
                    )
                .unwrap()
            }

            let session_result = create_user_session(&request.username, &pool).await;

            match session_result {
                Ok(session) => {
                    let access_token_cookie = format!("token={}; HttpOnly; Secure; Path=/; Max-Age={}", &session.token, DEFAULT_SESSION_DURATION_MIN * 60);
                    let refresh_token_cookie = format!("refresh_token={}; HttpOnly; Secure; Path=/refresh", &session.refresh_token);

                    Response::builder()
                        .status(StatusCode::OK)
                        .header(header::SET_COOKIE, access_token_cookie)
                        .header(header::SET_COOKIE, refresh_token_cookie)
                        .body(CreateResponse {
                            is_successful: true,
                            message: String::from("Successful login")
                        }).unwrap()
                },
                Err(err_msg) => {
                    Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(
                        CreateResponse { 
                            is_successful: false,
                            message: err_msg
                            }
                        )
                    .unwrap()
                },
            }

        },
        Err(err) => {
                Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(
                    CreateResponse { 
                        is_successful: false,
                        message: String::from("Invalid username or password")
                        }
                    )
                .unwrap()
        },
    }
}