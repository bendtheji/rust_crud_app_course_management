use actix_web::{delete, get, post, Responder, Scope, web};

use crate::api::students_courses::types::{CreateStudentCourseRequest, DeleteStudentCourseRequest, GetStudentCourseByCourseRequest, GetStudentCourseByStudentRequest};
use crate::db::{cancel_sign_up, DbPool, get_course_by_name, get_courses_attended_by_student, get_student_by_email, get_students_in_course, new_students_courses};

pub fn students_courses_api_scope() -> Scope {
    web::scope("/students-courses")
        .service(create_student_course)
        .service(get_courses_for_student)
        .service(delete_student_course)
        .service(fetch_students_in_course)
}

#[post("")]
async fn create_student_course(data: web::Data<DbPool>, req: web::Json<CreateStudentCourseRequest>) -> Result<String, std::io::Error> {
    let mut connection = data.get().unwrap();
    let student = get_student_by_email(&mut connection, &req.student_email);
    let course = get_course_by_name(&mut connection, &req.course_name);
    Ok(format!("{:?}", new_students_courses(&mut connection, student.id, course.id)))
}

#[get("/student")]
async fn get_courses_for_student(data: web::Data<DbPool>, params: web::Query<GetStudentCourseByStudentRequest>) -> impl Responder {
    let mut connection = data.get().unwrap();
    let courses_vec = get_courses_attended_by_student(&mut connection, &params.student_email);
    format!("{:?}", courses_vec.into_iter().map(|x| x.name).collect::<Vec<String>>())
}

#[delete("")]
async fn delete_student_course(data: web::Data<DbPool>, req: web::Json<DeleteStudentCourseRequest>) -> Result<String, std::io::Error> {
    let mut connection = data.get().unwrap();
    cancel_sign_up(&mut connection, &req.student_email, &req.course_name);
    Ok("deleted".to_string())
}

#[get("/course")]
async fn fetch_students_in_course(data: web::Data<DbPool>, params: web::Query<GetStudentCourseByCourseRequest>) -> impl Responder {
    let mut connection = data.get().unwrap();
    let students_vec = get_students_in_course(&mut connection, &params.course_name);
    format!("{:?}", students_vec.into_iter().map(|x| x.email).collect::<Vec<String>>())
}