
use std::collections::HashMap;

use axum::{body::Body, extract::State, http::{header, HeaderMap, Response, StatusCode}, Json};
use serde::Deserialize;
use serde_json::Map;
use crate::router_common::{CreateResponse, RouterGlobalState};

use super::{create_user_session, login, refresh_user_session, sign_up, User, DEFAULT_SESSION_DURATION_MIN};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String
}

#[derive(Deserialize)]
pub struct SignUpRequest {
    username: String,
    password: String,
    email: String,
    name: String
}

fn parse_cookie_value(value: String) -> HashMap<String, String> {
    let separated_kv=  value.split(";");
    let mut map  = HashMap::new();

    for unstructured_kv in separated_kv {
        let mut kv = unstructured_kv.split("=");
        let key = kv.next();
        let value= kv.next();

        if key.is_some() && value.is_some() {
            map.insert(String::from(key.unwrap()), String::from(value.unwrap()));
        }
    }

    return map;
}

// TODO: Create a logger for sensitive error only to server not returning raw error
// TODO: fix all unwrap
pub async fn login_router(
    State(router_global_state): State<RouterGlobalState>, // Use '_ to elide the explicit lifetime
    Json(request): Json<LoginRequest>
) -> Response<Body> {

    let result = login(&request.username, &request.password, &router_global_state.pool).await;
    
    match result {
        Ok(login_succesful) => {
            if !login_succesful {
                let response = serde_json::to_string(
                    &CreateResponse { 
                        is_successful: false,
                        message: String::from("Invalid username or password")
                    }
                ).unwrap();

                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from(response))
                    .unwrap();
            }

            let session_result = create_user_session(&request.username, &router_global_state.pool).await;

            match session_result {
                Ok(session) => {
                    //TODO: this probably won't work so need to change how to set the cookies
                    let access_token_cookie = format!("token={}; HttpOnly; Secure; Path=/; Max-Age={}", &session.token, DEFAULT_SESSION_DURATION_MIN * 60);
                    let refresh_token_cookie = format!("refresh_token={}; HttpOnly; Secure; Path=/refresh_token", &session.refresh_token);

                    let response =  serde_json::to_string(
                        &CreateResponse {
                            is_successful: true,
                            message: String::from("Successfully login")
                        }).unwrap();

                    return Response::builder()
                        .status(StatusCode::OK)
                        .header(header::SET_COOKIE, access_token_cookie)
                        .header(header::SET_COOKIE, refresh_token_cookie)
                        .body(Body::from(response)).unwrap();
                },
                Err(err_msg) => {

                    let response =  serde_json::to_string(
                        &CreateResponse {
                            is_successful: false,
                            message: String::from(err_msg)
                        }).unwrap();

                    Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from(response))
                    .unwrap()
                },
            }

        },
        Err(err) => {
                let response =  serde_json::to_string(
                        &CreateResponse { 
                            is_successful: false,
                            message: String::from("Invalid username or password")
                    }
                ).unwrap();

                return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(response))
                .unwrap();
        },
    }
}

pub async fn sign_up_router(
    State(RouterGlobalState {pool,..}): State<RouterGlobalState>,
    Json(request): Json<SignUpRequest>
) -> Response<Body> {
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
                let response = serde_json::to_string(&CreateResponse { 
                        is_successful: false,
                        message: String::from("Could not sign up due to error please try again")
                }).unwrap();

                return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(response))
                .unwrap();
            }

        let response = serde_json::to_string(
            &CreateResponse {
                is_successful: true,
                message: String::from("Successfuly sign up")
            }).unwrap();
        
        Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(response)).unwrap()
        },
        Err(error_message) => {
                let response = serde_json::to_string(
                    &CreateResponse { 
                        is_successful: false,
                        message: String::from(error_message)
                        }
                ).unwrap();

                return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(response))
                .unwrap();
        },
    }
}

pub async fn refresh_token_router(
    State(RouterGlobalState {pool,..}): State<RouterGlobalState>,
    headers: HeaderMap,
) -> Response<Body> {
    let cookies= headers.get("cookie");

    match cookies {
        Some(value) => {
            let cookie_map = parse_cookie_value(value.to_str().unwrap().to_owned());
            //TODO: probably need to set enum for  better settings
            let refresh_token = cookie_map.get("refresh_token");
            if refresh_token.is_none() {
                let response = serde_json::to_string( 
                    &CreateResponse { 
                        is_successful: false,
                        message: String::from("Please login before refreshing token")
                        }
                ).unwrap();

                return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(response))
                .unwrap();
            }
            let new_token_result = refresh_user_session(&refresh_token.unwrap(), &pool).await;
            match new_token_result {
                Ok(token) => {
                    let response = serde_json::to_string(
                        &CreateResponse { 
                            is_successful: true,
                            message: String::from("Token refreshed")
                        }).unwrap();

                    Response::builder()
                        .status(StatusCode::OK)
                        .header(header::SET_COOKIE, token)
                        .body(Body::from(response))
                        .unwrap()
                },
                Err(err) => {
                    let response = serde_json::to_string(
                        &CreateResponse { 
                        is_successful: false,
                        message: String::from(err)
                        }
                    ).unwrap();

                    return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from(response))
                    .unwrap();
                    },
            }
        } ,
        None => {
                let response = serde_json::to_string( 
                    &CreateResponse { 
                        is_successful: false,
                        message: String::from("Please login before refreshing token")
                        }
                ).unwrap();

                return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(response))
                .unwrap();
        },
    }
}