use actix_web::{App, HttpServer, Responder, web};
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

use crate::api::courses::handlers::courses_api_scope;
use crate::api::students::handlers::student_api_scope;
use crate::api::students_courses::handlers::students_courses_api_scope;
use crate::db::initialize_db_pool;

pub mod db;
mod models;
mod schema;
mod api;

struct AppState {
    db_connection: PgConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = initialize_db_pool();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(student_api_scope())
            .service(courses_api_scope())
            .service(students_courses_api_scope())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}