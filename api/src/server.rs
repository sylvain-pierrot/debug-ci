use crate::{domains::{health::controller::health_check, questions::handlers, scoreboard::handlers::{get_last_ten_scores_handler, create_score_handler}}, common::{state::app_state::AppState, config::env::AppEnvironment, middleware::auth::basic_auth_middleware}};
use axum::{
    routing::{get, post, put},
    Router, Server, middleware::from_fn_with_state,
};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite};
use std::net::SocketAddr;
use tracing::event;
use tower_http::cors::CorsLayer;

pub fn create_router(app_state: AppState) -> Router {
    let router = Router::new();

    let misc = Router::new().route("/health", get(health_check));

    router
        .route("/questions", post(handlers::create_question_handler))
        .layer(from_fn_with_state(app_state.clone(), basic_auth_middleware))
        .route("/questions/random", get(handlers::get_random_question_handler))
        .route("/questions/:question_id/answer", put(handlers::increment_answer_count_handler))
        .route("/scores", post(create_score_handler))
        .route("/scores", get(get_last_ten_scores_handler))
        .nest_service("/", misc)
        .with_state(app_state)
        .layer(CorsLayer::permissive())
}

pub async fn bootstrap() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();

    let env = AppEnvironment {
        address: dotenvy::var("ADDRESS")?,
        port: dotenvy::var("PORT")?.to_string().parse()?,
        database_url: dotenvy::var("DATABASE_URL")?,
        admin_username: dotenvy::var("ADMIN_USERNAME")?,
        admin_password: dotenvy::var("ADMIN_PASSWORD")?,
    };

    tracing_subscriber::fmt::init();

    // create database if it does not exist
    if !Sqlite::database_exists(&env.database_url).await? {
        event!(
            tracing::Level::INFO,
            "database {} does not exist, creating it...",
            env.database_url
        );
        match Sqlite::create_database(&env.database_url).await {
            Ok(_) => {
                event!(
                    tracing::Level::INFO,
                    "created database at {}",
                    env.database_url
                );
            }
            Err(e) => Err(e)?,
        }
    }

    // connect to database
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&env.database_url)
        .await?;
    event!(
        tracing::Level::INFO,
        "successfully connected to {}",
        env.database_url
    );

    // run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    let app_state = AppState::new(pool, env.clone());
    let app = create_router(app_state);

    event!(
        tracing::Level::INFO,
        "starting server on {}:{}",
        env.address,
        env.port
    );

    // start server
    let socker_addr: SocketAddr = format!("{}:{}", env.address, env.port).parse()?;
    Server::bind(&socker_addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
