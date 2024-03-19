use actix_web::{get, post, Responder, Scope, web};

use crate::api::courses::types::{CourseResponse, CreateCourseRequest, GetCourseRequest};
use crate::db::{DbPool, get_course_by_name, new_course};

pub fn courses_api_scope() -> Scope {
    web::scope("/courses")
        .service(get_course)
        .service(create_course)
}

#[get("")]
async fn get_course(data: web::Data<DbPool>, params: web::Query<GetCourseRequest>) -> impl Responder {
    let mut connection = data.get().unwrap();
    let course = get_course_by_name(&mut connection, &params.name);
    CourseResponse::from(course)
}

#[post("")]
async fn create_course(data: web::Data<DbPool>, req: web::Json<CreateCourseRequest>) -> Result<String, std::io::Error> {
    let mut connection = data.get().unwrap();
    let new_course_name = &req.name;
    Ok(format!("{:?}", new_course(&mut connection, new_course_name)))
}