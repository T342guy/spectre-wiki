use axum::{Extension, Router, routing::{get, post}};

#[derive(Debug, toasty::Model)]
struct Page {
    #[key]
    #[auto]
    id: u32,
    #[unique]
    slug: String,
    title: String,
    content: String,
}

async fn index() -> &'static str {
    "Welcome to Spectre Wiki!"
}

// async fn get_page() -> &'static str {

// }

// async fn edit_page() -> &'static str {

// }

// async fn update_page() -> &'static str {

// }

// async fn delete_page() -> &'static str {

// }


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Spectre Wiki");

    // todo: Initialize the database connection

    // in-memory database for testing
    // let db = toasty::Db::builder()
    //     .models(toasty::models!(crate::*))
    //     .connect("turso::memory:")
    //     .await?;
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "turso:./app.db".to_string());    

    let db = toasty::Db::builder()
    .models(toasty::models!(crate::*))
    .connect(&db_url)    
    .await?;

    //db.push_schema().await?;

    // Router
    let app = Router::new()
        .route("/", get(index))
        // .route("/page/:slug", get(get_page))
        // .route("/page/:slug/edit", get(edit_page).post(update_page))
        // .route("/page/:slug/delete", post(delete_page))
        .layer(Extension(db));

    println!("Listening on http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();    
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
