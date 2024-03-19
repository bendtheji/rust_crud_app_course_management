use actix_web::{get, post, Responder, Scope, web};

use crate::api::students::types::{CreateStudentRequest, GetStudentRequest, StudentResponse};
use crate::db::{DbPool, get_student_by_email, new_student};

pub fn student_api_scope() -> Scope {
    web::scope("/students")
        .service(get_student)
        .service(create_student)
}

#[get("")]
async fn get_student(data: web::Data<DbPool>, params: web::Query<GetStudentRequest>) -> impl Responder {
    let mut connection = data.get().unwrap();
    let student = get_student_by_email(&mut connection, &params.email);
    StudentResponse::from(student)
}

#[post("")]
async fn create_student(data: web::Data<DbPool>, req: web::Json<CreateStudentRequest>) -> Result<String, std::io::Error, > {
    let mut connection = data.get().unwrap();
    let new_email = &req.email;
    Ok(format!("{:?}", new_student(&mut connection, new_email)))
}