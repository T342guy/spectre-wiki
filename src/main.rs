use axum::{
    Extension, Router,
    extract::*,
    routing::{get, post},
};
use std::sync::Arc;

mod handlers;
mod templates;

use axum_template::engine::Engine;
use handlers::*;
use minijinja::{Environment, path_loader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Spectre Wiki");

    let template_path = "./templates";
    let mut tmpl_env = Environment::new();
    tmpl_env.set_loader(path_loader(template_path));

    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "turso:./app.db".to_string());

    let db = toasty::Db::builder()
        .models(toasty::models!(crate::*))
        .connect(&db_url)
        .await?;

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

    println!("Listening on http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
