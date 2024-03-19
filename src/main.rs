use actix_web::{App, HttpServer, Responder, web};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use serde::{Deserialize, Serialize};

use crate::api::courses::handlers::courses_api_scope;
use crate::api::students::handlers::student_api_scope;
use crate::api::students_courses::handlers::students_courses_api_scope;
use crate::db::initialize_db_pool;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
pub mod db;
mod models;
mod schema;
mod api;

struct AppState {
    db_connection: PgConnection,
}

fn run_migration(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Started main");
    let pool = initialize_db_pool();
    println!("CONNECTED!");
    run_migration(&mut pool.clone().get().unwrap());
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