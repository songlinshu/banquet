mod api;
mod jwt;
mod res;
mod utils;

use axum::{
    routing::{delete, get, post},
    AddExtensionLayer, Router,
};
use dotenv::dotenv;
use hyper::Method;
use sqlx::mysql::MySqlPoolOptions;
use std::env::var;
use std::net::SocketAddr;
use tower_http::cors::{any, CorsLayer};

use crate::api::{
    cook::create_cook,
    user::{get_phone_code, me}, menus::create_menu,
};
use crate::api::{
    cook::{delete_cook, list_cooks, set_spare_time},
    user::{get_users, phone_login},
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let client = redis::Client::open(format!(
        "redis://{}:{}/{}",
        var("REDIS_HOST").expect("获取 REDIS_HOST 错误"),
        var("REDIS_PORT").unwrap_or("6379".to_owned()),
        var("REDIS_DB").unwrap_or("0".to_owned())
    ))
    .unwrap();

    // 数据库连接池
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let user = Router::new()
        .route("/", get(get_users))
        .route("/me", get(me));

    let menu = Router::new().route("/", post(create_menu));

    let cook = Router::new()
        .route("/", post(create_cook))
        .route("/", get(list_cooks))
        .route("/:id", delete(delete_cook))
        .route("/:id/spare_time", post(set_spare_time));

    let api = Router::new()
        .nest("/users", user) // 用户相关接口
        .nest("/cooks", cook) // 厨师相关接口
        .nest("/menus", menu) // 菜单相关接口
        .route("/login/phone/code", post(get_phone_code))
        .route("/login/phone", post(phone_login))
        .route("/upload", post(api::upload::upload_file));

    let app = Router::new()
        .nest("/api", api)
        .route("/protected", get(jwt::protected))
        .route("/authorize", post(jwt::authorize))
        .layer(AddExtensionLayer::new(client))
        .layer(AddExtensionLayer::new(pool))
        .layer(
            CorsLayer::new()
                .allow_origin(any())
                .allow_methods(vec![Method::GET, Method::POST]),
        );

    let port: u16 = var("PORT").unwrap_or("8000".to_string()).parse().unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
