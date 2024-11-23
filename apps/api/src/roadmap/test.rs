#[cfg(test)]

mod tests {

    use sqlx::postgres::PgPoolOptions;

    use crate::{
        auth::{add_user, User},
        roadmap::{
            add_areas, add_roadmap, api::UpsertAreaRequest, delete_area, delete_roadmap,
            find_roadmap, get_roadmap_by_id, get_roadmaps, get_user_roadmaps, roadmap_exist,
            update_areas, update_roadmap, Areas, Roadmaps,
        },
        router_common::CreateResponse,
        utils::generate_random_str,
    };

    use core::panic;
    use std::{
        process::{Command, Stdio},
        sync::atomic::AtomicU64,
        time::Instant,
    };

    use futures_util::{sink::SinkExt, StreamExt};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use tokio::time::{self, Duration};
    use tokio_tungstenite::{connect_async, tungstenite::Message};

    const DEFAULT_RANDOM_LENGTH: u8 = 10;

    #[tokio::test]
    async fn test_add_roadmap() {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://myuser:mypassword@localhost/test_database")
            .await
            .unwrap();

        let mock_user = User {
            username: String::from("publisher1"),
            email: String::from("publisher@gmail.com"),
            name: String::from("Publisher"),
            password: String::from("password"),
            created_at: None,
            updated_at: None,
        };

        let _ = add_user(&mock_user, &pool).await;

        let mock_roadmap = Roadmaps {
            id: String::from("1"),
            title: String::from("title 1"),
            description: None,
            publisher: String::from("publisher1"),
            published: None,
            created_at: None,
            updated_at: None,
        };
        let result = add_roadmap(mock_roadmap, &pool).await;
        match result {
            Ok(pg_result) => {
                assert!(pg_result.rows_affected() == 1, "Affected rows are 0")
            }
            Err(err) => {
                panic!("Failed inserting roadmap {}", err)
            }
        }
    }

    #[tokio::test]
    async fn test_update_roadmap() {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://myuser:mypassword@localhost/test_database")
            .await
            .unwrap();

        let mock_user = User {
            username: "publisher2".to_string(),
            name: "Publisher 2".to_string(),
            email: "publisher2@pgmail.com".to_string(),
            password: "password".to_string(),
            created_at: None,
            updated_at: None,
        };

        let _ = add_user(&mock_user, &pool);

        let mock_roadmap = Roadmaps {
            id: String::from("2"),
            title: String::from("title 1"),
            description: None,
            publisher: String::from("publisher1"),
            published: None,
            created_at: None,
            updated_at: None,
        };

        let _ = add_roadmap(mock_roadmap.clone(), &pool).await;

        let updated_mock_roadmap = Roadmaps {
            title: "new title from the previous one".to_string(),
            description: Some("new description".to_string()),
            ..mock_roadmap
        };

        let result = update_roadmap(updated_mock_roadmap.clone(), &pool).await;

        match result {
            Ok(value) => {
                assert!(
                    value.rows_affected() == 1,
                    "Update not properly done, {} rows affected",
                    value.rows_affected()
                )
            }
            Err(err) => panic!("{}", err),
        };

        let existing_roadmap = get_roadmap_by_id("2".to_string(), &pool).await;
        match existing_roadmap {
            Ok(value) => {
                assert_eq!(
                    value.roadmap_title, updated_mock_roadmap.title,
                    "title are not equal"
                )
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[tokio::test]
    async fn test_get_roadmap() {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://myuser:mypassword@localhost/test_database")
            .await
            .unwrap();

        let mock_user = User {
            username: generate_random_str(DEFAULT_RANDOM_LENGTH),
            name: "Publisher 53".to_string(),
            email: "publisher53@pgmail.com".to_string(),
            password: "password".to_string(),
            created_at: None,
            updated_at: None,
        };

        let total_roadmap = 10;
        let _ = add_user(&mock_user, &pool).await;
        let mut add_roadmap_futures: Vec<_> = Vec::new();

        for _i in 0..total_roadmap {
            add_roadmap_futures.push(add_roadmap(
                Roadmaps {
                    id: generate_random_str(DEFAULT_RANDOM_LENGTH),
                    title: generate_random_str(DEFAULT_RANDOM_LENGTH),
                    description: None,
                    publisher: mock_user.username.clone(),
                    published: None,
                    created_at: None,
                    updated_at: None,
                },
                &pool,
            ));
        }

        //waiting for all the concurrent request result
        for roadmap in add_roadmap_futures {
            let _ = roadmap.await.unwrap();
        }

        let result = get_roadmaps(0, 5, &pool).await.unwrap();
        assert_eq!(
            result.len(),
            5,
            "getting roadmap should have shown 5 roadmaps found {} instead",
            result.len()
        );
    }

    #[tokio::test]
    async fn test_find_roadmap() {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://myuser:mypassword@localhost/test_database")
            .await
            .unwrap();

        let total_roadmap = 10;
        let mut add_roadmap_futures: Vec<_> = Vec::new();

        let mock_user = User {
            username: generate_random_str(DEFAULT_RANDOM_LENGTH),
            name: "Publisher 2".to_string(),
            email: "publisher2@pgmail.com".to_string(),
            password: "password".to_string(),
            created_at: None,
            updated_at: None,
        };

        let _ = add_user(&mock_user, &pool).await;

        for i in 0..total_roadmap {
            add_roadmap_futures.push(add_roadmap(
                Roadmaps {
                    id: generate_random_str(DEFAULT_RANDOM_LENGTH),
                    title: format!("title {}", i),
                    description: None,
                    publisher: mock_user.username.clone(),
                    published: None,
                    created_at: None,
                    updated_at: None,
                },
                &pool,
            ));
        }

        for roadmap in add_roadmap_futures {
            let _ = roadmap.await;
        }

        match find_roadmap("title".to_string(), 10, 0, &pool).await {
            Ok(value) => {
                if value.len() != 10 {
                    panic!(
                        "Should have found 10 elements, found {} instead",
                        value.len()
                    )
                }
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[tokio::test]
    async fn test_get_user_roadmap() {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://myuser:mypassword@localhost/test_database")
            .await
            .unwrap();

        let mock_user = User {
            username: generate_random_str(DEFAULT_RANDOM_LENGTH),
            name: "Publisher 2".to_string(),
            email: "publisher100@pgmail.com".to_string(),
            password: "password".to_string(),
            created_at: None,
            updated_at: None,
        };

        let _ = add_user(&mock_user, &pool).await;

        let total_roadmap = 10;
        let mut add_roadmap_futures: Vec<_> = Vec::new();

        for i in 0..total_roadmap {
            add_roadmap_futures.push(add_roadmap(
                Roadmaps {
                    id: generate_random_str(DEFAULT_RANDOM_LENGTH),
                    title: format!("title {}", i),
                    description: None,
                    publisher: mock_user.username.clone(),
                    published: None,
                    created_at: None,
                    updated_at: None,
                },
                &pool,
            ));
        }

        //waiting for all the concurrent request result
        for roadmap in add_roadmap_futures {
            let result = roadmap.await.unwrap();
        }

        match get_user_roadmaps(mock_user.username.clone(), &pool).await {
            Ok(value) => {
                if value.len() != 10 {
                    panic!("user roadmaps are not 10 found {} instead", value.len());
                }
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[tokio::test]
    async fn test_delete_roadmap() {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://myuser:mypassword@localhost/test_database")
            .await
            .unwrap();

        let mock_user = User {
            username: generate_random_str(DEFAULT_RANDOM_LENGTH),
            name: "Publisher 2".to_string(),
            email: "publisher100@pgmail.com".to_string(),
            password: "password".to_string(),
            created_at: None,
            updated_at: None,
        };

        let _ = add_user(&mock_user, &pool).await;
        let roadmap_id = generate_random_str(DEFAULT_RANDOM_LENGTH);
        let roadmap = Roadmaps {
            id: roadmap_id.clone(),
            title: generate_random_str(DEFAULT_RANDOM_LENGTH),
            description: None,
            publisher: mock_user.username.clone(),
            published: None,
            created_at: None,
            updated_at: None,
        };

        let _ = add_roadmap(roadmap, &pool).await;

        match delete_roadmap(roadmap_id, &pool).await {
            Ok(value) => {
                if value.rows_affected() != 1 {
                    panic!(
                        "affected row should have been 1 found {} instead",
                        value.rows_affected()
                    );
                }
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[tokio::test]
    async fn test_add_area() {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://myuser:mypassword@localhost/test_database")
            .await
            .unwrap();

        let mock_email = format!("{}@gmail.com", generate_random_str(DEFAULT_RANDOM_LENGTH));

        let mock_user = User {
            username: generate_random_str(DEFAULT_RANDOM_LENGTH),
            name: "Publisher 2".to_string(),
            email: mock_email,
            password: "password".to_string(),
            created_at: None,
            updated_at: None,
        };

        let _ = add_user(&mock_user, &pool).await;

        let roadmap_id = generate_random_str(DEFAULT_RANDOM_LENGTH);
        let roadmap = Roadmaps {
            id: roadmap_id.clone(),
            title: generate_random_str(DEFAULT_RANDOM_LENGTH),
            description: None,
            publisher: mock_user.username.clone(),
            published: None,
            created_at: None,
            updated_at: None,
        };

        let _ = add_roadmap(roadmap, &pool).await;
        let mut previous_area_id: Option<_> = None;
        let mut fut_add_areas: Vec<_> = Vec::new();

        let total_areas = 10;

        for _i in 0..total_areas {
            let area = Areas {
                id: generate_random_str(DEFAULT_RANDOM_LENGTH),
                parent_id: previous_area_id.clone(),
                roadmap_id: roadmap_id.clone(),
                title: generate_random_str(DEFAULT_RANDOM_LENGTH),
                description: None,
                created_at: None,
                updated_at: None,
                x: 0.0,
                y: 0.0,
            };
            previous_area_id = Some(area.id.clone());
            fut_add_areas.push(add_areas(area, &pool));
        }

        for area in fut_add_areas {
            let _ = area.await.unwrap();
        }

        match get_roadmap_by_id(roadmap_id.clone(), &pool).await {
            Ok(result) => {
                if result.areas.len() != 10 {
                    panic!(
                        "Roadmap area should have been 10, found {} instead",
                        result.areas.len()
                    );
                }
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[tokio::test]
    async fn test_update_area() {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://myuser:mypassword@localhost/test_database")
            .await
            .unwrap();

        let mock_email = format!("{}@gmail.com", generate_random_str(DEFAULT_RANDOM_LENGTH));

        let mock_user = User {
            username: generate_random_str(DEFAULT_RANDOM_LENGTH),
            name: "Publisher 2".to_string(),
            email: mock_email,
            password: "password".to_string(),
            created_at: None,
            updated_at: None,
        };

        let _ = add_user(&mock_user, &pool).await;

        let roadmap_id = generate_random_str(DEFAULT_RANDOM_LENGTH);
        let roadmap = Roadmaps {
            id: roadmap_id.clone(),
            title: generate_random_str(DEFAULT_RANDOM_LENGTH),
            description: None,
            publisher: mock_user.username.clone(),
            published: None,
            created_at: None,
            updated_at: None,
        };

        let _ = add_roadmap(roadmap, &pool).await;
        let area = Areas {
            id: generate_random_str(DEFAULT_RANDOM_LENGTH),
            parent_id: None,
            roadmap_id: roadmap_id.clone(),
            title: generate_random_str(DEFAULT_RANDOM_LENGTH),
            description: None,
            created_at: None,
            updated_at: None,
            x: 0.0,
            y: 0.0,
        };

        let updated_area = Areas {
            title: "New title".to_string(),
            ..area.clone()
        };

        let _ = add_areas(area, &pool).await;
        match update_areas(updated_area, &pool).await {
            Ok(result) => {
                if result.rows_affected() != 1 {
                    panic!(
                        "only one area should have been affected, found {} instead",
                        result.rows_affected()
                    );
                }
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[tokio::test]
    async fn test_delete_area() {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://myuser:mypassword@localhost/test_database")
            .await
            .unwrap();

        let mock_email = format!("{}@gmail.com", generate_random_str(DEFAULT_RANDOM_LENGTH));

        let mock_user = User {
            username: generate_random_str(DEFAULT_RANDOM_LENGTH),
            name: "Publisher 2".to_string(),
            email: mock_email,
            password: "password".to_string(),
            created_at: None,
            updated_at: None,
        };

        let _ = add_user(&mock_user, &pool).await;

        let roadmap_id = generate_random_str(DEFAULT_RANDOM_LENGTH);
        let roadmap = Roadmaps {
            id: roadmap_id.clone(),
            title: generate_random_str(DEFAULT_RANDOM_LENGTH),
            description: None,
            publisher: mock_user.username.clone(),
            published: None,
            created_at: None,
            updated_at: None,
        };

        let _ = add_roadmap(roadmap, &pool).await;
        let area_id = generate_random_str(DEFAULT_RANDOM_LENGTH);

        let area = Areas {
            id: area_id.clone(),
            parent_id: None,
            roadmap_id: roadmap_id.clone(),
            title: generate_random_str(DEFAULT_RANDOM_LENGTH),
            description: None,
            created_at: None,
            updated_at: None,
            x: 0.0,
            y: 0.0,
        };

        let _ = add_areas(area, &pool).await;
        match delete_area(area_id, &pool).await {
            Ok(result) => {
                if result.rows_affected() != 1 {
                    panic!(
                        "only one area should have been affected, found {} instead",
                        result.rows_affected()
                    );
                }
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[tokio::test]
    #[ignore]
    async fn load_test_area_update() {
        let connections = 100;
        let test_duration = Duration::from_secs(10);
        let server_runtime = Duration::from_secs(3);

        let mut child = Command::new("cargo")
            .arg("run")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("The server failed to run");

        tokio::time::sleep(server_runtime).await;

        let running = Arc::new(AtomicBool::new(true));

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://myuser:mypassword@localhost/test_database")
            .await
            .unwrap();

        let mock_email = format!("{}@gmail.com", generate_random_str(DEFAULT_RANDOM_LENGTH));
        let mock_username = generate_random_str(DEFAULT_RANDOM_LENGTH);

        let mock_user = User {
            username: mock_username.to_string(),
            name: "Publisher 2".to_string(),
            email: mock_email,
            password: "password".to_string(),
            created_at: None,
            updated_at: None,
        };

        let _ = add_user(&mock_user, &pool).await;

        let roadmap_id = generate_random_str(DEFAULT_RANDOM_LENGTH);

        let roadmap = Roadmaps {
            id: roadmap_id.clone(),
            title: generate_random_str(DEFAULT_RANDOM_LENGTH),
            description: None,
            publisher: mock_user.username.clone(),
            published: None,
            created_at: None,
            updated_at: None,
        };

        let _ = add_roadmap(roadmap, &pool)
            .await
            .expect("Adding roadmap should not throw error");

        let _ = roadmap_exist(roadmap_id.to_string(), &pool)
            .await
            .expect("Roadmap should have existed");

        let mut handles = Vec::new();

        let url = format!(
            "ws://localhost:3000/api/v1/roadmaps/{}/areas",
            roadmap_id.to_string()
        );

        let total_areas = 100_000;

        let mut fut_add_areas: Vec<_> = Vec::new();

        for _i in 0..total_areas {
            let area = Areas {
                id: generate_random_str(DEFAULT_RANDOM_LENGTH),
                parent_id: None,
                roadmap_id: roadmap_id.clone(),
                title: generate_random_str(DEFAULT_RANDOM_LENGTH),
                description: None,
                created_at: None,
                updated_at: None,
                x: 0.0,
                y: 0.0,
            };
            fut_add_areas.push(add_areas(area, &pool));
        }

        for area in fut_add_areas {
            let _ = area.await.unwrap();
        }

        for i in 0..connections {
            let url = url.clone();
            let running_clone = running.clone();
            let handle = tokio::spawn(async move {
                match connect_async(&url).await {
                    Ok((mut ws_stream, _)) => {
                        // 60 rps
                        let mut current_area_id: Option<String> = None;
                        let mut latency = 0;
                        let mut total_messages = 0;

                        while running_clone.load(Ordering::Relaxed) {
                            // Create message using the ID from previous response
                            total_messages += 1;
                            let message = UpsertAreaRequest {
                                area_id: current_area_id.clone(),
                                parent_id: None,
                                title: generate_random_str(DEFAULT_RANDOM_LENGTH),
                                description: None,
                                x: 0.0,
                                y: 0.0,
                            };

                            let message_json = serde_json::to_string(&message).unwrap();
                            let message = Message::Text(message_json);
                            let start_time = Instant::now();

                            match ws_stream.send(message).await {
                                Ok(_) => {
                                    // Wait for and parse the response
                                    match ws_stream.next().await {
                                        Some(Ok(Message::Text(response_text))) => {
                                            match serde_json::from_str::<CreateResponse>(
                                                &response_text,
                                            ) {
                                                Ok(response) => {
                                                    if response.is_successful {
                                                        // Update the area_id for the next iteration
                                                        current_area_id = response.id;
                                                    } else {
                                                        eprintln!(
                                                            "Server returned error: {}",
                                                            response.message
                                                        );
                                                    }

                                                    latency +=
                                                        start_time.elapsed().as_millis() as u64;
                                                }
                                                Err(e) => {
                                                    eprintln!(
                                                        "Failed to parse response JSON: {}",
                                                        e
                                                    );
                                                    eprintln!("Response text: {}", response_text);
                                                }
                                            }
                                        }
                                        Some(Ok(_)) => {
                                            eprintln!("Received non-text message");
                                        }
                                        Some(Err(e)) => {
                                            eprintln!("Error receiving message: {}", e);
                                            break;
                                        }
                                        None => {
                                            eprintln!("WebSocket connection closed");
                                            break;
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed sending message: {}", e);
                                    break;
                                }
                            }
                        }

                        println!(
                            "total message sent for connection {}: {}ms",
                            i, total_messages
                        );
                        println!(
                            "average latency for connection {}: {}ms",
                            i,
                            latency / total_messages
                        );
                    }
                    Err(e) => {
                        eprintln!("Failed connecting to websocket: {}", e);
                    }
                }
            });

            handles.push(handle);
        }

        // Wait for specified duration
        tokio::time::sleep(test_duration).await;

        // Signal all connections to stop
        running.store(false, Ordering::Relaxed);

        // Wait for all connections to complete
        for handle in handles {
            handle.await.unwrap();
        }

        child
            .kill()
            .expect("Failed to kill the process after finishing the test");
    }
}
