#[cfg(test)]
mod tests {
    #[tokio::test]

    async fn test_add_roadmap() {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://myuser:mypassword@localhost/test_database")
            .await
            .unwrap();
    }

    async fn test_update_roadmap() {}

    async fn test_find_roadmap() {}

    async fn test_get_roadmap() {}

    async fn test_find_user_roadmap() {}

    async fn test_delete_roadmap() {}

    async fn test_add_area() {}

    async fn test_update_area() {}

    async fn test_delete_area() {}
}
