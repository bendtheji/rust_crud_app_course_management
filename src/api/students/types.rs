use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use chrono::{Local, TimeZone};
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
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl From<Student> for StudentResponse {
    fn from(student: Student) -> Self {
        StudentResponse {
            id: student.id,
            email: student.email,
            phone_number: student.phone_number,
            created_at: Some(Local.from_utc_datetime(&student.created_at.unwrap()).format("%Y-%m-%d %H:%M:%S").to_string()),
            updated_at: Some(Local.from_utc_datetime(&student.updated_at.unwrap()).format("%Y-%m-%d %H:%M:%S").to_string()),
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