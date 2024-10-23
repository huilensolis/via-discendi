
use axum::{extract::State, http::{header, HeaderMap, Response, StatusCode}, Json};
use crate::router_config::{CreateResponse, RouterGlobalState};

use super::{create_user_session, login, refresh_user_session, sign_up, User, DEFAULT_SESSION_DURATION_MIN};

pub struct LoginRequest {
    username: String,
    password: String
}

pub struct SignUpRequest {
    username: String,
    password: String,
    email: String,
    name: String
}

// TODO: Create a logger for sensitive error only to server not returning raw error
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
                    //TODO: this probably won't work so need to change how to set the cookies
                    let access_token_cookie = format!("token={}; HttpOnly; Secure; Path=/; Max-Age={}", &session.token, DEFAULT_SESSION_DURATION_MIN * 60);
                    let refresh_token_cookie = format!("refresh_token={}; HttpOnly; Secure; Path=/refresh", &session.refresh_token);

                    Response::builder()
                        .status(StatusCode::OK)
                        .header(header::SET_COOKIE, access_token_cookie)
                        .header(header::SET_COOKIE, refresh_token_cookie)
                        .body(CreateResponse {
                            is_successful: true,
                            message: String::from("Successfully login")
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

pub async fn sign_up_router(
    State(RouterGlobalState {pool,..}): State<RouterGlobalState>,
    Json(request): Json<SignUpRequest>
) -> Response<CreateResponse> {
    let mut sign_up_user = User{
        created_at: None,
        updated_at: None,
        username: request.username,
        password: request.password,
        email: request.email,
        name: request.name
    };

    let sign_up_result = sign_up(&mut sign_up_user, &pool).await;

    //TODO: change error to some enum type to be able to decide whether to return error directly on API side
    match sign_up_result {
        Ok(is_successful) => {

            if !is_successful {
                return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(
                    CreateResponse { 
                        is_successful: false,
                        message: String::from("Could not sign up due to error please try again")
                        }
                    )
                .unwrap();
            }

        Response::builder()
            .status(StatusCode::OK)
            .body(CreateResponse {
                is_successful: true,
                message: String::from("Successfuly sign up")
            }).unwrap()
        },
        Err(error_message) => {
                return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(
                    CreateResponse { 
                        is_successful: false,
                        message: String::from(error_message)
                        }
                    )
                .unwrap();
        },
    }
}

pub async fn refresh_token_router(
    State(RouterGlobalState {pool,..}): State<RouterGlobalState>,
    headers: HeaderMap,
) -> Response<CreateResponse> {
    //TODO: get the correct key
    let refresh_token = headers.get("REFRESH_TOKEN");
    match refresh_token {
        Some(value) => {
            
            let new_token_result = refresh_user_session(&String::from(value.to_str().unwrap()), &pool).await;
            match new_token_result {
                Ok(token) => {
                    Response::builder()
                        .status(StatusCode::OK)
                        .header(header::SET_COOKIE, token)
                        .body(CreateResponse { 
                            is_successful: true,
                            message: String::from("Token refreshed")
                        })
                        .unwrap()
                },
                Err(err) => {
                return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(
                    CreateResponse { 
                        is_successful: false,
                        message: String::from(err)
                        }
                    )
                .unwrap();
                },
            }
        } ,
        None => {
                return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(
                    CreateResponse { 
                        is_successful: false,
                        message: String::from("Please login before refreshing token")
                        }
                    )
                .unwrap();
        },
    }
}
