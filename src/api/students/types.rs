use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};

use crate::db::students::models::Student;

#[derive(Deserialize)]
pub struct GetStudentRequest {
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CreateStudentRequest {
    pub email: String,
    pub phone_number: Option<String>,
}

#[derive(Serialize)]
pub struct StudentResponse {
    id: i32,
    email: String,
    phone_number: Option<String>,
}

impl From<Student> for StudentResponse {
    fn from(student: Student) -> Self {
        StudentResponse {
            id: student.id,
            email: student.email,
            phone_number: student.phone_number,
        }
    }
}

impl Responder for StudentResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}