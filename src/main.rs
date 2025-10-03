mod models;
mod repository;

use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{Form, Html, IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use lazy_static::lazy_static;
use models::contact::{Contact, ContactForm};
use repository::contact_db::ContactDB;
use serde::Deserialize;
use tera::{Context, Tera};
use tokio;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing errors(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![]);
        tera
    };
}

lazy_static! {
    static ref CONTACTS: Mutex<ContactDB> = Mutex::new(ContactDB::new());
}

#[derive(Deserialize)]
struct ContactSearch {
    q: Option<String>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/contacts", get(contacts))
        .route("/contacts/new", get(contacts_new_get))
        .route("/contacts/new", post(contacts_new_post))
        .route("/contacts/{id}", post(contacts_view));
    let app = app.fallback(handler_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Redirect {
    Redirect::permanent("/contacts")
}

async fn contacts(Query(contact_search): Query<ContactSearch>) -> Html<String> {
    let contacts_db = CONTACTS.lock().await;
    let result = match contact_search.q {
        Some(q) => {
            let result = contacts_db.search(q);
            let contacts = match result {
                Some(c) => {
                    c
                }
                None => contacts_db.all(),
            };

            let mut context = Context::new();
            context.insert("contacts", &contacts);

            let to_render = TEMPLATES.render("index.html", &context).unwrap();
            Html(to_render)
        }
        None => {
            let contacts = contacts_db.all();

            let mut context = Context::new();
            context.insert("contacts", &contacts);

            let to_render = TEMPLATES.render("index.html", &context).unwrap();
            Html(to_render)
        }
    };

    result
}

async fn contacts_new_get() -> Html<String> {
    let c = Contact {
        ..Default::default()
    };
    let mut context = Context::new();
    context.insert("contact", &c);
    let to_render = TEMPLATES.render("new.html", &context).unwrap();
    Html(to_render)
}

async fn contacts_new_post(Form(form_data): Form<ContactForm>) -> impl IntoResponse {
    let mut errors = HashMap::new();

    if form_data.email.trim().is_empty() {
        errors.insert("email".to_string(), "Email is required".to_string());
    }
    if form_data.first_name.trim().is_empty() {
        errors.insert(
            "first_name".to_string(),
            "First name is required".to_string(),
        );
    }
    if form_data.last_name.trim().is_empty() {
        errors.insert("last_name".to_string(), "Last name is required".to_string());
    }
    if form_data.phone.trim().is_empty() {
        errors.insert("phone".to_string(), "Phone number is required.".to_string());
    }

    let contact = Contact::new(
        999,
        form_data.first_name,
        form_data.last_name,
        form_data.phone,
        form_data.email,
        errors,
    );

    let mut context = Context::new();

    let mut contacts_db = CONTACTS.lock().await;

    let html = if contact.errors.is_empty() {
        contacts_db.save(contact);
        let contacts = contacts_db.all();
        context.insert("contacts", &contacts);

        TEMPLATES.render("index.html", &context).unwrap()
    } else {
        context.insert("contact", &contact);
        TEMPLATES.render("new.html", &context).unwrap()
    };

    Html(html)
}

async fn contacts_view(Path(id): Path<u32>) -> Html<String> {
    let 
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "whoops didn't find what you are looking for",
    )
}
