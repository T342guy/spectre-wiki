use axum::{
    response::IntoResponse,
    Extension
};
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
const CONTEXT_HTML_INSTERT: &str = "Nathan";
#[derive(Clone)]
pub struct AppState<'a> {
    pub db: toasty::Db,
    pub tmpl_engine: Environment<'a>,
}

pub async fn index(Extension(state): Extension<Arc<AppState<'static>>>) -> impl IntoResponse {
    let template = state.tmpl_engine.get_template("index.html").unwrap();

    /*
    upon /GET, serve index.html with context value for {{ user }}.
    Currently, this is set as an &str const value for testing purposes.
     */
    let s = template.render(context! { name => CONTEXT_HTML_INSTERT});
    
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
