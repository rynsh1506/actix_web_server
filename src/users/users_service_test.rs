#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::super::dto::{create_users_dto::CreateUserDTO, get_users_dto::GetUserDTO};
    use crate::utils::{
        errors::AppError,
        query_paginaton::QueryPagination,
        response_data::{ResponseData, ResponseDatas},
    };
    use chrono::Utc;
    use mockall::mock;
    use uuid::Uuid;

    #[async_trait::async_trait]
    pub trait UserRepo {
        async fn create_users_service(
            &self,
            mut payload: CreateUserDTO,
        ) -> Result<ResponseData<GetUserDTO>, AppError>;
        async fn find_all_users_service(
            &self,
            pagination: QueryPagination,
        ) -> Result<ResponseDatas<Vec<GetUserDTO>>, AppError>;

        async fn find_users_service(&self, id: Uuid) -> Result<ResponseData<GetUserDTO>, AppError>;
    }

    mock! {
        pub UserRepository {}

        #[async_trait::async_trait]
        impl UserRepo for UserRepository {
            async fn create_users_service(
                &self,
                payload: CreateUserDTO,
            ) -> Result<ResponseData<GetUserDTO>, AppError>;
            async fn find_all_users_service(
                &self,
                pagination: QueryPagination,
            ) -> Result<ResponseDatas<Vec<GetUserDTO>>, AppError>;

            async fn find_users_service(&self, id: Uuid) -> Result<ResponseData<GetUserDTO>, AppError>;
        }

    }

    #[tokio::test]
    async fn test_create_user_service() {
        let mut mock_repo = MockUserRepository::new();

        mock_repo
            .expect_create_users_service()
            .returning(|payload| {
                Ok(ResponseData::new(GetUserDTO {
                    id: Uuid::new_v4(),
                    name: payload.name.clone(),
                    email: payload.email.clone(),
                    created_at: payload.created_at,
                    updated_at: payload.updated_at,
                }))
            });

        let payload = CreateUserDTO {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            password: "password123".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let result = mock_repo.create_users_service(payload).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.data.name, "Test User");
        assert_eq!(response.data.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_find_all_users_service() {
        let mut mock_repo = MockUserRepository::new();

        mock_repo.expect_find_all_users_service().returning(|_| {
            Ok(ResponseDatas::new(
                10,
                1,
                2,
                2,
                vec![
                    GetUserDTO {
                        id: uuid::Uuid::new_v4(),
                        name: "Test User 1".to_string(),
                        email: "test1@example.com".to_string(),
                        created_at: chrono::Utc::now(),
                        updated_at: chrono::Utc::now(),
                    },
                    GetUserDTO {
                        id: uuid::Uuid::new_v4(),
                        name: "Test User 2".to_string(),
                        email: "test2@example.com".to_string(),
                        created_at: chrono::Utc::now(),
                        updated_at: chrono::Utc::now(),
                    },
                ],
            ))
        });

        let mut order = HashMap::new();
        order.insert("created_at".to_string(), "asc".to_string());

        let pagination = QueryPagination {
            limit: 10,
            page: 1,
            order,
        };

        let result = mock_repo.find_all_users_service(pagination).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.limit, Some(10));
    }

    #[tokio::test]
    async fn test_find_users_service() {
        let mut mock_repo = MockUserRepository::new();

        mock_repo.expect_find_users_service().returning(|id| {
            Ok(ResponseData::new(GetUserDTO {
                id,
                name: "Test User 1".to_string(),
                email: "test1@example.com".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }))
        });

        let id = Uuid::new_v4();

        let result = mock_repo.find_users_service(id).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.data.id, id);
    }
}
