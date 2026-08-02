use axum::{
    Extension, Router,
    extract::*,
    routing::{get, post},
};
use std::sync::Arc;
use tracing::{event, span, Level, Subscriber, info};
use tracing_subscriber::{
    FmtSubscriber, EnvFilter
};
mod handlers;
mod templates;
use axum_template::engine::Engine;
use handlers::*;
use minijinja::{Environment, path_loader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than info will be written to stdout.
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed"); // make subscriber global default. But this also makes every other logger go to this subscriber.

    info!("Spectre wiki is starting...");

    let template_path = "./templates";
    let mut tmpl_env = Environment::new();
    tmpl_env.set_loader(path_loader(template_path));

    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "turso:./app.db".to_string());

    let db = toasty::Db::builder()
        .models(toasty::models!(crate::*))
        .connect(&db_url)
        .await?;
    info!("Database connected.");

    let shared_state = Arc::new(handlers::AppState {
        db,
        tmpl_engine: tmpl_env,
    });

    // Router
    let app = Router::new()
        .route("/", get(index))
        // .route("/page/:slug", get(get_page))
        // .route("/page/:slug/edit", get(edit_page).post(update_page))
        // .route("/page/:slug/delete", post(delete_page))
        .layer(Extension(shared_state));

    info!("Spectre wiki stared!");
    info!("Listening on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}
