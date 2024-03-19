use actix_web::{get, post, Responder, Scope, web};

use crate::api::errors::ApiError;
use crate::api::students::types::{CreateStudentRequest, GetStudentRequest, StudentResponse};
use crate::db;
use crate::db::students::db_functions;

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
    let student = db_functions::create_student(&mut connection, new_email)?;
    Ok(StudentResponse::from(student))
}