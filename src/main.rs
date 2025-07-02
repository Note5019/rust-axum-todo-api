use axum::{
    Router,
    routing::{get, post, put},
};
use rust_axum_todo_api::todo::handler::{
    create_todo, delete_todo, get_todo, get_todos, update_todo,
};
use rust_axum_todo_api::{app_state::AppState, time_helper::TimerHelper};
use rust_axum_todo_api::{database, setting::Setting};
use sqlx::Postgres;
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let setting = Setting::new().unwrap();
    info!("setting has been loaded.");

    let db_pool: sqlx::Pool<Postgres> = database::conn_getting(Arc::clone(&setting)).await.unwrap();
    info!("database connection has been established.");

    let shared_state = Arc::new(AppState {
        db: db_pool.to_owned(),
        time_helper: TimerHelper::Directly.creation(),
    });

    // match sqlx::migrate!("./migrations").run(&db_pool).await {
    //     Ok(_) => println!("run migration successful."),
    //     Err(e) => println!("Migration error: {}", e),
    // };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/todos", post(create_todo).get(get_todos))
        .route(
            "/todos/{id}",
            put(update_todo).get(get_todo).delete(delete_todo),
        )
        .with_state(shared_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("Application running!");

    axum::serve(listener, app).await.unwrap();
    info!("Application running! 2");
}
