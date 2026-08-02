use axum::Extension;
use axum::response::IntoResponse;
use jiff::{Timestamp};
use std::sync::Arc;
use minijinja::{context, Environment};
use tracing::error;

#[derive(Debug, toasty::Model)]
pub struct Page {
    #[key]
    #[auto]
    id: u32,
    #[unique]
    slug: String,
    title: String,
    content: String,
    html_content: String,
    created_at: Timestamp,
    updated_at: Timestamp,
}

#[derive(Clone)]
pub struct AppState<'a> {
    pub db: toasty::Db,
    pub tmpl_engine: Environment<'a>,
}

pub async fn index(Extension(state): Extension<Arc<AppState<'static>>>) -> impl IntoResponse {
    let template = state.tmpl_engine.get_template("index.html").unwrap();
    
    let s = template.render(context! { name => "Wade"}); // upon /GET, serve index.html with context value for {{ user }} being "Wade".
    
    match s { //what does this do?
        Ok(s) => s,
        Err(e) => { String::from("Undefined render error") }
    }
}

// async fn get_page() -> &'static str {

// }

// async fn edit_page() -> &'static str {

// }

// async fn update_page() -> &'static str {

// }

// async fn delete_page() -> &'static str {

// }
