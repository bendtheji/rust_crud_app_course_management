use actix_web::{get, post, Responder, Scope, web};

use crate::api::courses::types::{CourseResponse, CreateCourseRequest, GetCourseRequest};
use crate::api::errors::ApiError;
use crate::db;
use crate::db::courses::db_functions;

pub fn courses_api_scope() -> Scope {
    web::scope("/courses")
        .service(get_course)
        .service(create_course)
}

#[get("")]
async fn get_course(data: web::Data<db::DbPool>, params: web::Query<GetCourseRequest>) -> Result<impl Responder, ApiError> {
    let mut connection = data.get().unwrap();
    let course = db_functions::get_course(&mut connection, &params.name)?;
    Ok(CourseResponse::from(course))
}

#[post("")]
async fn create_course(data: web::Data<db::DbPool>, req: web::Json<CreateCourseRequest>) -> Result<impl Responder, ApiError> {
    let mut connection = data.get().unwrap();
    let new_course_name = &req.name;
    let course = db_functions::create_course(&mut connection, new_course_name)?;
    Ok(CourseResponse::from(course))
}

#[cfg(test)]
mod tests {
    use actix_web::{App, test, web};
    use diesel::PgConnection;

    use crate::api::courses::handlers::courses_api_scope;
    use crate::api::courses::types::CreateCourseRequest;
    use crate::db::courses::db_functions;
    use crate::db::initialize_db_pool;

    #[actix_web::test]
    async fn test_create_course_happy_path() {
        let pool = initialize_db_pool();
        let course = "pizza making".to_string();
        setup_existing_course(false, &mut pool.clone().get().unwrap(), &course);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(courses_api_scope())
        ).await;

        let req = test::TestRequest::post()
            .uri("/courses")
            .set_json(CreateCourseRequest { name: course.clone() })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        cleanup(&mut pool.clone().get().unwrap(), &course);
    }

    #[actix_web::test]
    async fn test_create_course_duplicate() {
        let pool = initialize_db_pool();
        let course = "pizza baking".to_string();
        setup_existing_course(true, &mut pool.clone().get().unwrap(), &course);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(courses_api_scope())
        ).await;

        let req = test::TestRequest::post()
            .uri("/courses")
            .set_json(CreateCourseRequest { name: course.clone() })
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_client_error());

        cleanup(&mut pool.clone().get().unwrap(), &course);
    }

    #[actix_web::test]
    async fn test_get_course_happy_path() {
        let pool = initialize_db_pool();
        let course = "astronomy".to_string();
        setup_existing_course(true, &mut pool.clone().get().unwrap(), &course);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(courses_api_scope())
        ).await;

        let req = test::TestRequest::get()
            .uri(&format!("/courses?name={}", &course))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        cleanup(&mut pool.clone().get().unwrap(), &course);
    }

    #[actix_web::test]
    async fn test_get_course_not_found() {
        let pool = initialize_db_pool();
        let course = "astrology".to_string();
        setup_existing_course(false, &mut pool.clone().get().unwrap(), &course);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(courses_api_scope())
        ).await;

        let req = test::TestRequest::get()
            .uri(&format!("/courses?name={}", &course))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        cleanup(&mut pool.clone().get().unwrap(), &course);
    }

    fn setup_existing_course(should_exist: bool, conn: &mut PgConnection, name: &str) {
        match should_exist {
            true => db_functions::create_course(conn, name).map(|_| ()).expect("setup failed"),
            false => db_functions::delete_course(conn, name).map(|_| ()).expect("setup failed"),
        };
    }

    fn cleanup(conn: &mut PgConnection, email: &str) {
        db_functions::delete_course(conn, email).expect("cleanup failed");
    }
}