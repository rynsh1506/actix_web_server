#[cfg(test)]
mod tests {
    use crate::{
        users::dto::{create_users_dto::CreateUserDTO, get_users_dto::GetUserDTO},
        utils::{
            errors::AppError,
            query_paginaton::QueryPagination,
            response_data::{ResponseData, ResponseDatas},
        },
    };
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
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                }))
            });

        let payload = CreateUserDTO {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            password: "password123".to_string(),
        };

        let result = mock_repo.create_users_service(payload).await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.data.name, "Test User");
        assert_eq!(user.data.email, "test@example.com");
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
                "ASC".to_string(),
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
        let pagination = QueryPagination {
            limit: Some(10),
            page: Some(1),
            order: Some("ASC".to_string()),
        };

        let result = mock_repo.find_all_users_service(pagination).await;

        assert!(result.is_ok());
        let response_data = result.unwrap();
        assert_eq!(response_data.data.len(), 2);
        assert_eq!(response_data.data[0].name, "Test User 1");
    }
}
