use actix_web::{get, post, Responder, Scope, web};

use crate::api::errors::ApiError;
use crate::api::students::types::{CreateStudentRequest, GetStudentRequest, StudentResponse};
use crate::api::utils;
use crate::db;
use crate::db::students::db_functions;
use crate::db::students::models::NewStudent;

pub fn student_api_scope() -> Scope {
    web::scope("/students")
        .service(get_student)
        .service(create_student)
}

#[get("")]
async fn get_student(data: web::Data<db::DbPool>, params: web::Query<GetStudentRequest>) -> Result<impl Responder, ApiError> {
    let mut connection = data.get().unwrap();
    let student = db_functions::get_student(&mut connection, &params.email)?;
    Ok(StudentResponse::from(student))
}

#[post("")]
async fn create_student(data: web::Data<db::DbPool>, req: web::Json<CreateStudentRequest>) -> Result<impl Responder, ApiError> {
    let mut connection = data.get().unwrap();
    let new_email = &req.email;
    if !utils::validate_email(new_email.as_str()) { return Err(ApiError::BadClientData); }
    let student = db_functions::create_student(&mut connection, NewStudent::from(req.0))?;
    Ok(StudentResponse::from(student))
}

#[cfg(test)]
pub mod tests {
    use actix_web::{App, test, web};
    use diesel::PgConnection;

    use crate::api::students::handlers::student_api_scope;
    use crate::api::students::types::CreateStudentRequest;
    use crate::db::initialize_db_pool;
    use crate::db::students::db_functions;
    use crate::db::students::models::NewStudent;

    #[actix_web::test]
    async fn test_create_student_happy_path() {
        let pool = initialize_db_pool();
        let request = CreateStudentRequest { email: String::from("sample_user_one@gmail.com") };
        setup_existing_student(false, &mut pool.clone().get().unwrap(), request.clone().into());

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(student_api_scope())
        ).await;

        let req = test::TestRequest::post()
            .uri("/students")
            .set_json(request.clone())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        cleanup(&mut pool.clone().get().unwrap(), &request.email);
    }

    #[actix_web::test]
    async fn test_create_student_invalid_email_format() {
        let pool = initialize_db_pool();
        let request = CreateStudentRequest { email: String::from("invalid email") };
        setup_existing_student(false, &mut pool.clone().get().unwrap(), request.clone().into());

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(student_api_scope())
        ).await;

        let req = test::TestRequest::post()
            .uri("/students")
            .set_json(request.clone())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        cleanup(&mut pool.clone().get().unwrap(), &request.email);
    }

    #[actix_web::test]
    async fn test_create_student_duplicate() {
        let pool = initialize_db_pool();
        let request = CreateStudentRequest { email: String::from("sample_user_two@gmail.com") };
        setup_existing_student(true, &mut pool.clone().get().unwrap(), request.clone().into());

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(student_api_scope())
        ).await;

        let req = test::TestRequest::post()
            .uri("/students")
            .set_json(request.clone())
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_client_error());

        cleanup(&mut pool.clone().get().unwrap(), &request.email);
    }

    #[actix_web::test]
    async fn test_get_student_happy_path() {
        let pool = initialize_db_pool();
        let request = CreateStudentRequest { email: String::from("sample_user_three@gmail.com") };
        setup_existing_student(true, &mut pool.clone().get().unwrap(), request.clone().into());

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(student_api_scope())
        ).await;

        let req = test::TestRequest::get()
            .uri(&format!("/students?email={}", &request.email))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        cleanup(&mut pool.clone().get().unwrap(), &request.email);
    }

    #[actix_web::test]
    async fn test_get_student_not_found() {
        let pool = initialize_db_pool();
        let request = CreateStudentRequest { email: String::from("sample_user_four@gmail.com") };
        setup_existing_student(false, &mut pool.clone().get().unwrap(), request.clone().into());

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(student_api_scope())
        ).await;

        let req = test::TestRequest::get()
            .uri(&format!("/students?email={}", &request.email))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        cleanup(&mut pool.clone().get().unwrap(), &request.email);
    }

    pub fn setup_existing_student(should_exist: bool, conn: &mut PgConnection, student: NewStudent) {
        match should_exist {
            true => db_functions::create_student(conn, student).map(|_| ()).expect("setup failed"),
            false => db_functions::delete_student(conn, &student.email).map(|_| ()).expect("setup failed"),
        };
    }

    pub fn cleanup(conn: &mut PgConnection, email: &str) {
        db_functions::delete_student(conn, email).expect("cleanup failed");
    }
}