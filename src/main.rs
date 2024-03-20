use std::env;

use actix_web::{App, HttpServer, web};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;

use crate::api::courses::handlers::courses_api_scope;
use crate::api::students::handlers::student_api_scope;
use crate::api::students_courses::handlers::students_courses_api_scope;
use crate::db::initialize_db_pool;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub mod db;
mod api;
pub mod schema;


fn run_migration(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

fn get_binding_address<'a>() -> &'a str {
    dotenv().ok();
    let app_env = env::var("APP_ENV").expect("should have APP_ENV set");
    let binding_address = match &app_env[..] {
        "DEV" => "127.0.0.1",
        "DOCKER" => "0.0.0.0",
        _ => panic!("wrong APP_ENV setting")
    };
    binding_address
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let binding_address = get_binding_address();
    let pool = initialize_db_pool();
    run_migration(&mut pool.clone().get().unwrap());
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(student_api_scope())
            .service(courses_api_scope())
            .service(students_courses_api_scope())
    })
        .bind((binding_address, 8080))?
        .run()
        .await
}
