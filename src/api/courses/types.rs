use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};

use crate::db::courses::models::Course;

#[derive(Deserialize)]
pub struct GetCourseRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CreateCourseRequest {
    pub name: String,
}

#[derive(Serialize, Debug)]
pub struct CourseResponse {
    id: i32,
    name: String,
}

impl From<Course> for CourseResponse {
    fn from(course: Course) -> Self {
        CourseResponse { id: course.id, name: course.name }
    }
}

impl Responder for CourseResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}