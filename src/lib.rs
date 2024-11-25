pub mod configs {
    pub mod config_conn;
    pub mod config_env;
    pub mod config_load;
    pub mod config_tls;
}

pub mod middlewares {
    pub mod middleware_logger;
}

pub mod utils {
    pub mod errors;
    pub mod logger;
    pub mod password;
    pub mod query_builder;
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
    }
    pub mod users_handler;
    pub mod users_service;
    pub mod users_service_test;
}
