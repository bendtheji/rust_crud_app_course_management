use actix_web::{get, post, Responder, Scope, web};

use crate::api::courses::types::{CourseResponse, CreateCourseRequest, GetCourseRequest};
use crate::api::errors::ApiError;
use crate::db;

pub fn courses_api_scope() -> Scope {
    web::scope("/courses")
        .service(get_course)
        .service(create_course)
}

#[get("")]
async fn get_course(data: web::Data<db::DbPool>, params: web::Query<GetCourseRequest>) -> Result<impl Responder, ApiError> {
    let mut connection = data.get().unwrap();
    let course = db::get_course(&mut connection, &params.name)?;
    Ok(CourseResponse::from(course))
}

#[post("")]
async fn create_course(data: web::Data<db::DbPool>, req: web::Json<CreateCourseRequest>) -> Result<impl Responder, ApiError> {
    let mut connection = data.get().unwrap();
    let new_course_name = &req.name;
    let course = db::create_course(&mut connection, new_course_name)?;
    Ok(CourseResponse::from(course))
}