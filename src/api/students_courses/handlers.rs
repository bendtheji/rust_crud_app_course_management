use actix_web::{delete, get, HttpResponse, post, Responder, Scope, web};

use crate::api::errors::ApiError;
use crate::api::students_courses::types::{CreateStudentCourseRequest, DeleteStudentCourseRequest, GetStudentCourseByCourseRequest, GetStudentCourseByStudentRequest};
use crate::db;
use crate::db::courses::db_functions as courses_db_functions;
use crate::db::students::db_functions as students_db_functions;
use crate::db::students_courses::db_functions as students_courses_db_functions;

pub fn students_courses_api_scope() -> Scope {
    web::scope("/students-courses")
        .service(create_student_course)
        .service(get_courses_for_student)
        .service(delete_student_course)
        .service(fetch_students_in_course)
}

#[post("")]
async fn create_student_course(data: web::Data<db::DbPool>, req: web::Json<CreateStudentCourseRequest>) -> Result<impl Responder, ApiError> {
    let mut connection = data.get().unwrap();
    let student = students_db_functions::get_student(&mut connection, &req.student_email)?;
    let course = courses_db_functions::get_course(&mut connection, &req.course_name)?;
    students_courses_db_functions::create_student_course(&mut connection, student.id, course.id)?;
    Ok(HttpResponse::Ok().body("student sign up successful"))
}

#[get("/student")]
async fn get_courses_for_student(data: web::Data<db::DbPool>, params: web::Query<GetStudentCourseByStudentRequest>) -> Result<impl Responder, ApiError> {
    let mut connection = data.get().unwrap();
    let courses_vec = students_courses_db_functions::get_courses_attended_by_student(&mut connection, &params.student_email)?;
    let courses_list = format!("{:?}", courses_vec.into_iter().map(|x| x.name).collect::<Vec<String>>());
    Ok(HttpResponse::Ok().body(courses_list))
}

#[delete("")]
async fn delete_student_course(data: web::Data<db::DbPool>, req: web::Json<DeleteStudentCourseRequest>) -> Result<impl Responder, ApiError> {
    let mut connection = data.get().unwrap();
    let student = students_db_functions::get_student(&mut connection, &req.student_email)?;
    let course = courses_db_functions::get_course(&mut connection, &req.course_name)?;
    students_courses_db_functions::delete_student_course(&mut connection, student.id, course.id)?;
    Ok(HttpResponse::Ok().body("sign-up deleted successfully"))
}

#[get("/course")]
async fn fetch_students_in_course(data: web::Data<db::DbPool>, params: web::Query<GetStudentCourseByCourseRequest>) -> Result<impl Responder, ApiError> {
    let mut connection = data.get().unwrap();
    let students_vec = students_courses_db_functions::get_students_in_course(&mut connection, &params.course_name)?;
    let students_list = format!("{:?}", students_vec.into_iter().map(|x| x.email).collect::<Vec<String>>());
    Ok(HttpResponse::Ok().body(students_list))
}

#[cfg(test)]
pub mod tests {
    use actix_web::{App, test, web};
    use actix_web::web::resource;

    use crate::api::courses::handlers::tests as courses_test;
    use crate::api::students::handlers::tests as students_tests;
    use crate::api::students::types::CreateStudentRequest;
    use crate::api::students_courses::handlers::students_courses_api_scope;
    use crate::api::students_courses::types::{CreateStudentCourseRequest, DeleteStudentCourseRequest};
    use crate::db::initialize_db_pool;

    #[actix_web::test]
    async fn test_create_and_delete_student_course_happy_path() {
        let pool = initialize_db_pool();
        let request = CreateStudentRequest { email: String::from("test_student_one@gmail.com"), ..Default::default() };
        students_tests::setup_existing_student(true, &mut pool.clone().get().unwrap(), request.clone().into());
        let course = "test_course_one".to_string();
        courses_test::setup_existing_course(true, &mut pool.clone().get().unwrap(), &course);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(students_courses_api_scope())
        ).await;

        let req = test::TestRequest::post()
            .uri("/students-courses")
            .set_json(CreateStudentCourseRequest { student_email: request.email.clone(), course_name: course.clone() })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let req = test::TestRequest::delete()
            .uri("/students-courses")
            .set_json(DeleteStudentCourseRequest { student_email: request.email.clone(), course_name: course.clone() })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        students_tests::cleanup(&mut pool.clone().get().unwrap(), &request.email);
        courses_test::cleanup(&mut pool.clone().get().unwrap(), &course);
    }

    #[actix_web::test]
    async fn test_create_student_course_no_student_found() {
        let pool = initialize_db_pool();
        let request = CreateStudentRequest { email: String::from("test_student_two@gmail.com"), ..Default::default() };
        students_tests::setup_existing_student(false, &mut pool.clone().get().unwrap(), request.clone().into());
        let course = "test_course_two".to_string();
        courses_test::setup_existing_course(true, &mut pool.clone().get().unwrap(), &course);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(students_courses_api_scope())
        ).await;

        let req = test::TestRequest::post()
            .uri("/students-courses")
            .set_json(CreateStudentCourseRequest { student_email: request.email.clone(), course_name: course.clone() })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        students_tests::cleanup(&mut pool.clone().get().unwrap(), &request.email);
        courses_test::cleanup(&mut pool.clone().get().unwrap(), &course);
    }

    #[actix_web::test]
    async fn test_create_student_course_no_course_found() {
        let pool = initialize_db_pool();
        let request = CreateStudentRequest { email: String::from("test_student_three@gmail.com"), ..Default::default() };
        students_tests::setup_existing_student(true, &mut pool.clone().get().unwrap(), request.clone().into());
        let course = "test_course_three".to_string();
        courses_test::setup_existing_course(false, &mut pool.clone().get().unwrap(), &course);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(students_courses_api_scope())
        ).await;

        let req = test::TestRequest::post()
            .uri("/students-courses")
            .set_json(CreateStudentCourseRequest { student_email: request.email.clone(), course_name: course.clone() })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());


        students_tests::cleanup(&mut pool.clone().get().unwrap(), &request.email);
        courses_test::cleanup(&mut pool.clone().get().unwrap(), &course);
    }

    #[actix_web::test]
    async fn test_create_student_course_duplicate() {
        let pool = initialize_db_pool();
        let request = CreateStudentRequest { email: String::from("test_student_four@gmail.com"), ..Default::default() };
        students_tests::setup_existing_student(true, &mut pool.clone().get().unwrap(), request.clone().into());
        let course = "test_course_four".to_string();
        courses_test::setup_existing_course(true, &mut pool.clone().get().unwrap(), &course);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(students_courses_api_scope())
        ).await;

        let req = test::TestRequest::post()
            .uri("/students-courses")
            .set_json(CreateStudentCourseRequest { student_email: request.email.clone(), course_name: course.clone() })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let req = test::TestRequest::post()
            .uri("/students-courses")
            .set_json(CreateStudentCourseRequest { student_email: request.email.clone(), course_name: course.clone() })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        let req = test::TestRequest::delete()
            .uri("/students-courses")
            .set_json(DeleteStudentCourseRequest { student_email: request.email.clone(), course_name: course.clone() })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        students_tests::cleanup(&mut pool.clone().get().unwrap(), &request.email);
        courses_test::cleanup(&mut pool.clone().get().unwrap(), &course);
    }

    #[actix_web::test]
    async fn test_fetch_courses_attended_by_student() {
        let pool = initialize_db_pool();
        let request = CreateStudentRequest { email: String::from("test_student_five@gmail.com"), ..Default::default() };
        students_tests::setup_existing_student(true, &mut pool.clone().get().unwrap(), request.clone().into());
        let course = "test_course_five".to_string();
        courses_test::setup_existing_course(true, &mut pool.clone().get().unwrap(), &course);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(students_courses_api_scope())
        ).await;

        let req = test::TestRequest::post()
            .uri("/students-courses")
            .set_json(CreateStudentCourseRequest { student_email: request.email.clone(), course_name: course.clone() })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let req = test::TestRequest::get()
            .uri(&format!("/students-courses/student?student_email={}", &request.email))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let req = test::TestRequest::delete()
            .uri("/students-courses")
            .set_json(DeleteStudentCourseRequest { student_email: request.email.clone(), course_name: course.clone() })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        students_tests::cleanup(&mut pool.clone().get().unwrap(), &request.email);
        courses_test::cleanup(&mut pool.clone().get().unwrap(), &course);
    }

    #[actix_web::test]
    async fn test_fetch_courses_attended_by_student_no_courses() {
        let pool = initialize_db_pool();
        let request = CreateStudentRequest { email: String::from("test_student_six@gmail.com"), ..Default::default() };
        students_tests::setup_existing_student(true, &mut pool.clone().get().unwrap(), request.clone().into());
        let course = "test_course_six".to_string();
        courses_test::setup_existing_course(true, &mut pool.clone().get().unwrap(), &course);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(students_courses_api_scope())
        ).await;

        let req = test::TestRequest::get()
            .uri(&format!("/students-courses/student?student_email={}", &request.email))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        students_tests::cleanup(&mut pool.clone().get().unwrap(), &request.email);
        courses_test::cleanup(&mut pool.clone().get().unwrap(), &course);
    }

    #[actix_web::test]
    async fn test_fetch_students_in_course() {
        let pool = initialize_db_pool();
        let request = CreateStudentRequest { email: String::from("test_student_seven@gmail.com"), ..Default::default() };
        students_tests::setup_existing_student(true, &mut pool.clone().get().unwrap(), request.clone().into());
        let course = "test_course_seven".to_string();
        courses_test::setup_existing_course(true, &mut pool.clone().get().unwrap(), &course);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(students_courses_api_scope())
        ).await;

        let req = test::TestRequest::post()
            .uri("/students-courses")
            .set_json(CreateStudentCourseRequest { student_email: request.email.clone(), course_name: course.clone() })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let req = test::TestRequest::get()
            .uri(&format!("/students-courses/course?course_name={}", course))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let req = test::TestRequest::delete()
            .uri("/students-courses")
            .set_json(DeleteStudentCourseRequest { student_email: request.email.clone(), course_name: course.clone() })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        students_tests::cleanup(&mut pool.clone().get().unwrap(), &request.email);
        courses_test::cleanup(&mut pool.clone().get().unwrap(), &course);
    }

    #[actix_web::test]
    async fn test_fetch_students_in_course_no_student() {
        let pool = initialize_db_pool();
        let request = CreateStudentRequest { email: String::from("test_student_eight@gmail.com"), ..Default::default() };
        students_tests::setup_existing_student(true, &mut pool.clone().get().unwrap(), request.clone().into());
        let course = "test_course_eight".to_string();
        courses_test::setup_existing_course(true, &mut pool.clone().get().unwrap(), &course);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(students_courses_api_scope())
        ).await;

        let req = test::TestRequest::get()
            .uri(&format!("/students-courses/course?course_name={}", course))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        students_tests::cleanup(&mut pool.clone().get().unwrap(), &request.email);
        courses_test::cleanup(&mut pool.clone().get().unwrap(), &course);
    }
}