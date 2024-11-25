#[cfg(test)]
mod tests {
    use crate::{
        configs::config_load::{load_connection, load_env},
        users::{
            dto::create_users_dto::CreateUserDTO,
            users_service::{create, find_all},
        },
        utils::{query_builder::QueryBuilder, query_paginaton::QueryPagination},
    };
    use fake::{faker::internet::en::SafeEmail, faker::name::en::Name, Fake};

    #[tokio::test]
    async fn test_create_user() {
        dotenvy::dotenv().ok();
        let config = load_env();
        let pool = load_connection(&config.db_url).await;

        let user = CreateUserDTO {
            email: "test@example.com".to_string(),
            name: "John Doe".to_string(),
            password: "password123".to_string(),
        };
        let result = create(&pool, actix_web::web::Json(user)).await;
        let query = QueryBuilder::new()
            .delete("users")
            .where_clause()
            .condition("email = $1")
            .build();

        match result {
            Ok(response) => {
                assert_eq!(response.data.name, "John Doe");
                assert_eq!(response.data.email, "test@example.com");
                sqlx::query(&query)
                    .bind("test@example.com")
                    .execute(&pool)
                    .await
                    .expect("Failed to delete test users");
            }
            Err(err) => panic!("Error occurred: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_find_all_users() {
        dotenvy::dotenv().ok();
        let config = load_env();
        let pool = load_connection(&config.db_url).await;

        // generate query insert
        let query = QueryBuilder::new()
            .insert("users", "name, email, password", "$1, $2, $3")
            .build();

        let users: Vec<(String, String)> = (0..2)
            .map(|_| (Name().fake(), SafeEmail().fake()))
            .collect();

        for (name, email) in &users {
            sqlx::query(&query)
                .bind(name)
                .bind(email)
                .bind("password123")
                .execute(&pool)
                .await
                .expect("Failed to insert user");
        }

        let pagination = QueryPagination {
            limit: Some("10".to_string()),
            page: Some(1),
            order: Some("DESC".to_string()),
        };

        let result = find_all(&pool, actix_web::web::Query(pagination)).await;

        // generate query delete
        let query = QueryBuilder::new()
            .delete("users")
            .where_clause()
            .condition("email = $1")
            .build();

        match result {
            Ok(response) => {
                assert_eq!(response.order, "DESC");
                assert_eq!(response.limit, "10");
                assert_eq!(response.count, 2);
                assert_eq!(response.page_count, 2);
                assert_eq!(response.data.len(), users.len());

                let emails: Vec<String> = response
                    .data
                    .iter()
                    .map(|user| user.email.clone())
                    .collect();

                for (_, email) in &users {
                    assert!(emails.contains(email));
                }

                for (_, email) in &users {
                    sqlx::query(&query)
                        .bind(email)
                        .execute(&pool)
                        .await
                        .expect("Failed to delete test users");
                }
            }
            Err(err) => panic!("Error occurred: {:?}", err),
        }
    }
}
