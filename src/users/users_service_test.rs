#[cfg(test)]
mod tests {
    use crate::{
        users::{
            dto::{CreateUserDTO, GetUserDTO, UpdateUserDTO},
            entity::User,
        },
        utils::{
            errors::AppError,
            query_paginaton::QueryPagination,
            response_data::{ResponseData, ResponseDatas},
        },
    };
    use chrono::Utc;
    use mockall::mock;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[async_trait::async_trait]
    pub trait UserRepo {
        async fn create_users_service(
            &self,
            mut payload: CreateUserDTO,
        ) -> Result<ResponseData<GetUserDTO>, AppError>;
        async fn find_all_users_service(
            &self,
            query_pagination: QueryPagination,
        ) -> Result<ResponseDatas<Vec<GetUserDTO>>, AppError>;

        async fn find_users_service(&self, id: Uuid) -> Result<ResponseData<GetUserDTO>, AppError>;
        async fn update_users_service(
            &self,
            id: Uuid,
            payload: UpdateUserDTO,
        ) -> Result<ResponseData<GetUserDTO>, AppError>;
        async fn delete_users_service(
            &self,
            id: Uuid,
        ) -> Result<ResponseData<GetUserDTO>, AppError>;
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
                query_pagination: QueryPagination,
            ) -> Result<ResponseDatas<Vec<GetUserDTO>>, AppError>;

            async fn find_users_service(&self, id: Uuid) -> Result<ResponseData<GetUserDTO>, AppError>;
            async fn update_users_service(
                &self,
                id: Uuid,
                payload: UpdateUserDTO,
            ) -> Result<ResponseData<GetUserDTO>, AppError>;
            async fn delete_users_service(&self, id: Uuid) -> Result<ResponseData<GetUserDTO>, AppError>;
        }

    }

    #[tokio::test]
    async fn test_create_user_service() {
        let mut mock_repo = MockUserRepository::new();

        mock_repo
            .expect_create_users_service()
            .returning(|payload| {
                let user: User = payload.into();
                Ok(ResponseData::new(
                    GetUserDTO { ..user.into() },
                    "data success full",
                ))
            });

        let payload = CreateUserDTO {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            password: "password123".to_string(),
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

        mock_repo
            .expect_find_all_users_service()
            .returning(|query_pagination| {
                let (limit, offset, page, order) = query_pagination.paginate();
                let mut users = vec![
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
                    GetUserDTO {
                        id: uuid::Uuid::new_v4(),
                        name: "Test User 3".to_string(),
                        email: "test3@example.com".to_string(),
                        created_at: chrono::Utc::now(),
                        updated_at: chrono::Utc::now(),
                    },
                    GetUserDTO {
                        id: uuid::Uuid::new_v4(),
                        name: "Test User 4".to_string(),
                        email: "test4@example.com".to_string(),
                        created_at: chrono::Utc::now(),
                        updated_at: chrono::Utc::now(),
                    },
                ];

                if let Some((key, value)) = order.iter().next() {
                    match key.as_str() {
                        "created_at" => {
                            if value.eq("ASC") {
                                users.sort_by_key(|u| u.created_at);
                            } else {
                                users.sort_by_key(|u| std::cmp::Reverse(u.created_at));
                            }
                        }
                        "updated_at" => {
                            if value.eq("ASC") {
                                users.sort_by_key(|u| u.updated_at);
                            } else {
                                users.sort_by_key(|u| std::cmp::Reverse(u.updated_at));
                            }
                        }
                        "name" => {
                            if value.eq("ASC") {
                                users.sort_by_key(|u| u.name.clone());
                            } else {
                                users.sort_by_key(|u| std::cmp::Reverse(u.name.clone()));
                            }
                        }
                        "email" => {
                            if value.eq("ASC") {
                                users.sort_by_key(|u| u.email.clone());
                            } else {
                                users.sort_by_key(|u| u.email.clone());
                            }
                        }
                        _ => panic!("field not fount"),
                    }
                }

                let sliced_users = &users
                    [offset as usize..std::cmp::min(offset as usize + limit as usize, users.len())];
                Ok(ResponseDatas::new(
                    limit,
                    page,
                    users.len() as i64,
                    sliced_users.len(),
                    sliced_users.to_vec(),
                ))
            });

        let mut order = HashMap::new();
        order.insert("name".to_string(), "desc".to_string());

        let query_pagination = QueryPagination {
            limit: None,
            page: None,
            order,
        };

        let result = mock_repo.find_all_users_service(query_pagination).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.data.len(), 4);
        assert_eq!(response.page, 1);
        assert_eq!(response.limit, Some(4));
        assert_eq!(response.data[3].name, "Test User 1")
    }

    #[tokio::test]
    async fn test_find_users_service() {
        let mut mock_repo = MockUserRepository::new();

        mock_repo.expect_find_users_service().returning(|id| {
            let user = GetUserDTO {
                id,
                name: "Test User 1".to_string(),
                email: "test1@example.com".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            if user.id.eq(&id) {
                Ok(ResponseData::new(
                    user,
                    "Data has been successfully retrieved",
                ))
            } else {
                Err(AppError::NotFound("User Not Found".to_string()))
            }
        });

        let id = Uuid::new_v4();

        let result = mock_repo.find_users_service(id).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.data.id, id);
    }

    #[tokio::test]
    async fn test_update_users_service() {
        let mut mock_repo = MockUserRepository::new();

        mock_repo
            .expect_update_users_service()
            .returning(|id, payload| {
                let user = GetUserDTO {
                    id,
                    name: payload.name.unwrap(),
                    email: payload.email.unwrap(),
                    created_at: chrono::Utc::now(),
                    updated_at: payload.updated_at,
                };

                if user.id.eq(&id) {
                    Ok(ResponseData::new(
                        user,
                        "Data has been successfully updated.",
                    ))
                } else {
                    Err(AppError::NotFound("User Not Found".to_string()))
                }
            });

        let id = Uuid::new_v4();

        let updated_at = Utc::now();

        let payload = UpdateUserDTO {
            email: Some("test1@example.com".to_string()),
            name: Some("Test User 1".to_string()),
            password: None,
            updated_at,
        };

        let result = mock_repo.update_users_service(id, payload).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.data.id, id);
        assert_eq!(response.data.name, "Test User 1");
        assert_eq!(response.data.email, "test1@example.com");
        assert_eq!(response.data.updated_at, updated_at);
    }

    #[tokio::test]
    async fn test_delete_users_service() {
        let mut mock_repo = MockUserRepository::new();

        mock_repo.expect_delete_users_service().returning(|id| {
            let user = GetUserDTO {
                id,
                name: "Test User 1".to_string(),
                email: "test1@example.com".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            if user.id.eq(&id) {
                Ok(ResponseData::new(user, "Data has ben successfuly deleted."))
            } else {
                Err(AppError::NotFound("User Not Found".to_string()))
            }
        });

        let id = Uuid::new_v4();

        let result = mock_repo.delete_users_service(id).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.data.id, id);
        assert_eq!(response.data.name, "Test User 1");
        assert_eq!(response.data.email, "test1@example.com");
    }
}
