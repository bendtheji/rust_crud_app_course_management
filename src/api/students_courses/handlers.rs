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