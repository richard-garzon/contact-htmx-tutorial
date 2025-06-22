mod models;
mod repository;

use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Router,
};
use lazy_static::lazy_static;
use repository::contact_db::ContactDB;
use serde::Deserialize;
use tera::{Context, Result, Tera};
use tokio;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing errors(s): {}", e);
                ::std::process::exit(1);
            }
        };
        // tera.autoescape_on(vec![]);
        tera
    };
}

lazy_static! {
    static ref CONTACTS: ContactDB = ContactDB::new();
}

#[derive(Deserialize)]
struct ContactSearch {
    q: Option<String>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/contacts", get(contacts));
    let app = app.fallback(handler_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Redirect {
    Redirect::permanent("/contacts")
}

async fn contacts(Query(contact_search): Query<ContactSearch>) -> Html<String> {
    let result = match contact_search.q {
        Some(q) => {
            let result = CONTACTS.get(q);
            let contacts = vec![result];

            let mut context = Context::new();
            context.insert("contacts", &contacts);

            let to_render = TEMPLATES.render("index.html", &context).unwrap();
            Html(to_render)
        }
        None => {
            let contacts = CONTACTS.all();

            let mut context = Context::new();
            context.insert("contacts", &contacts);

            let to_render = TEMPLATES.render("index.html", &context).unwrap();
            Html(to_render)
        }
    };

    result
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404")
}
