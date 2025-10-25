mod models;
mod repository;

use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    http::{HeaderMap, StatusCode},
    response::{Form, Html, IntoResponse, Redirect},
    routing::{delete, get, post},
    Router,
};
use lazy_static::lazy_static;
use models::contact::{Contact, ContactForm};
use repository::contact_db::ContactDB;
use serde::Deserialize;
use tera::{Context, Tera};
use tokio;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;

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
    static ref CONTACTS: Mutex<ContactDB> = Mutex::new(ContactDB::new(30));
}

#[derive(Deserialize)]
struct ContactSearch {
    q: Option<String>,
}

#[derive(Deserialize)]
struct EmailValidation {
    email: Option<String>,
}

#[tokio::main]
async fn main() {
    let static_files = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(index))
        .route("/contacts", get(contacts))
        .route("/contacts/", get(contacts))
        .route("/contacts/new", get(contacts_new_get))
        .route("/contacts/new", post(contacts_new_post))
        .route("/contacts/{id}", get(contacts_view))
        .route("/contacts/{id}/edit", get(contacts_edit_get))
        .route("/contacts/{id}/edit", post(contacts_edit_post))
        .route("/contacts/{id}", delete(contacts_delete))
        .route("/contacts", delete(contacts_bulk_delete))
        .route("/contacts/{id}/email", get(contacts_email))
        .nest_service("/static", static_files)
        .fallback(handler_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Redirect {
    Redirect::permanent("/contacts")
}

async fn contacts(
    headers: HeaderMap,
    Query(contact_search): Query<ContactSearch>,
) -> impl IntoResponse {
    let contacts_db = CONTACTS.lock().await;
    let search = contact_search.q.unwrap_or_else(|| String::new());
    let contacts = match search.is_empty() {
        true => contacts_db.all(),
        false => contacts_db.search(search),
    };
    let mut context = Context::new();
    context.insert("contacts", &contacts);

    let htmx_trigger = headers.get("hx-trigger").map(|h| h.to_str().unwrap_or(""));
    let rendered = match htmx_trigger {
        Some("search") => {
            context.insert("contacts", &contacts);
            Html(TEMPLATES.render("rows.html", &context).unwrap())
        }
        _ => {
            context.insert("contacts", &contacts);
            Html(TEMPLATES.render("index.html", &context).unwrap())
        }
    };
    rendered
}

async fn contacts_bulk_delete(body: String) -> impl IntoResponse {
    let mut selected_ids = Vec::new();

    // Parse "selected_contact_ids=1&selected_contact_ids=2" format
    for pair in body.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            if key == "selected_contact_ids" {
                if let Ok(id) = value.parse::<u32>() {
                    selected_ids.push(id);
                }
            }
        }
    }

    let mut contacts_db = CONTACTS.lock().await;

    for id in selected_ids {
        contacts_db.delete(id);
        println!("Deleted contact with ID: {}", id);
    }

    Redirect::to("/contacts").into_response()
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
    let errors = HashMap::new();

    let mut contact = Contact::new(
        999,
        form_data.first_name,
        form_data.last_name,
        form_data.phone,
        form_data.email,
        errors,
    );

    contact.validate();

    let mut context = Context::new();

    let html = if contact.errors.is_empty() {
        let mut contacts_db = CONTACTS.lock().await;
        contacts_db.save(contact);
        Redirect::to("/contacts").into_response()
    } else {
        context.insert("contact", &contact);
        Html(TEMPLATES.render("new.html", &context).unwrap()).into_response()
    };

    html
}

async fn contacts_view(Path(id): Path<u32>) -> Html<String> {
    let contacts_db = CONTACTS.lock().await;
    let result = match contacts_db.find(id) {
        Some(c) => {
            let mut context = Context::new();
            context.insert("contact", c);
            Html(TEMPLATES.render("show.html", &context).unwrap())
        }
        None => Html(TEMPLATES.render("404.html", &Context::new()).unwrap()),
    };

    result
}

async fn contacts_edit_get(Path(id): Path<u32>) -> Html<String> {
    let contacts_db = CONTACTS.lock().await;
    let result = match contacts_db.find(id) {
        Some(c) => {
            let mut context = Context::new();
            context.insert("contact", c);
            Html(TEMPLATES.render("edit.html", &context).unwrap())
        }
        None => Html(TEMPLATES.render("404.html", &Context::new()).unwrap()),
    };

    result
}

async fn contacts_email(Path(id): Path<u32>, Query(email): Query<EmailValidation>) -> Html<String> {
    let contacts_db = CONTACTS.lock().await;
    let curr_contact = contacts_db.find(id).unwrap();
    let curr_email = email.email.unwrap().to_string();
    if contacts_db.email_exists(&curr_email) && curr_email != curr_contact.email {
        return Html("Email already exists".to_string());
    } else {
        return Html("".to_string());
    }
}

async fn contacts_edit_post(
    Path(id): Path<u32>,
    Form(form_data): Form<ContactForm>,
) -> impl IntoResponse {
    let errors = HashMap::new();

    let mut contact = Contact::new(
        id,
        form_data.first_name,
        form_data.last_name,
        form_data.phone,
        form_data.email,
        errors,
    );

    contact.validate();

    let mut context = Context::new();

    let html = if contact.errors.is_empty() {
        let mut contacts_db = CONTACTS.lock().await;
        contacts_db.update(id, &contact);
        Redirect::to("/contacts").into_response()
    } else {
        context.insert("contact", &contact);
        Html(TEMPLATES.render("edit.html", &context).unwrap()).into_response()
    };

    html
}

async fn contacts_delete(Path(id): Path<u32>, headers: HeaderMap) -> impl IntoResponse {
    let mut contacts_db = CONTACTS.lock().await;
    contacts_db.delete(id);

    let hx_trigger = headers.get("hx-trigger").map(|h| h.to_str().unwrap_or(""));

    let rendered = match hx_trigger {
        Some("delete-btn") => Redirect::to("/contacts").into_response(),
        _ => Html("Contact deleted").into_response(),
    };

    rendered
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "whoops didn't find what you are looking for",
    )
}
