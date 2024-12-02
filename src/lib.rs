pub mod configs {
    pub mod config_conn;
    pub mod config_env;
    pub mod config_load;
    pub mod config_tls;
}

pub mod middlewares {
    pub mod middleware_auth;
    pub mod middleware_logger;
}

pub mod utils {
    pub mod auth;
    pub mod errors;
    pub mod jwt;
    pub mod logger;
    pub mod password;
    pub mod query_paginaton;
    pub mod response_data;
    pub mod time;
}

pub mod router;
pub mod server;

// ALL Service
// User
pub mod users {
    pub mod dto {
        pub mod create_users_dto;
        pub mod get_users_dto;
        pub mod update_users_dto;

        pub use create_users_dto::CreateUserDTO;
        pub use get_users_dto::GetUserDTO;
        pub use update_users_dto::UpdateUserDTO;
    }

    pub mod entity {
        pub mod users_model;

        pub use users_model::User;
    }

    pub mod users_handler;
    pub mod users_query;
    pub mod users_service;
    pub mod users_service_test;
}

pub mod auth {
    pub mod dto {
        pub mod jwt_dto;
        pub mod login_dto;
        pub use jwt_dto::{Claims, JwtDto};
        pub use login_dto::{GetLoginDto, LoginDto};
    }
    pub mod auth_handler;
    pub mod auth_service;
}
