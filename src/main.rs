use axum::{
    Router,
    routing::{get, post},
};
use rust_axum_todo_api::time_helper::{IntoTimerHelperShared, TimerHelper};
use rust_axum_todo_api::todo::handler::{create_todo, get_todos};
use rust_axum_todo_api::todo::repositories::{SharedTodoRepository, TodoRepository};
use rust_axum_todo_api::{database, setting::Setting};
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let setting = Setting::new().unwrap();
    info!("setting has been loaded.");

    let db_pool = database::conn_getting(Arc::clone(&setting)).await.unwrap();
    info!("database connection has been established.");

    // match sqlx::migrate!("./migrations").run(&db_pool).await {
    //     Ok(_) => println!("run migration successful."),
    //     Err(e) => println!("Migration error: {}", e),
    // };

    let timer_helper: IntoTimerHelperShared = TimerHelper::Directly.creation();

    let todo_repository: SharedTodoRepository = TodoRepository::creation(db_pool.clone());

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/todos",
            post({
                let repo = Arc::clone(&todo_repository);
                move |body| create_todo(body, repo, Arc::clone(&timer_helper))
            })
            .get({
                let repo = Arc::clone(&todo_repository);
                move || get_todos(repo)
            }),
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("Application running!");

    axum::serve(listener, app).await.unwrap();
    info!("Application running! 2");
}
