use chrono::prelude::*;
use diesel::{Identifiable, Insertable, Queryable, Selectable};

use crate::api::students::types::CreateStudentRequest;
use crate::schema::students;

#[derive(Identifiable, Queryable, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = students)]
pub struct Student {
    pub id: i32,
    pub email: String,
    pub phone_number: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Clone, Default)]
#[diesel(table_name = students)]
pub struct NewStudent {
    pub email: String,
    pub phone_number: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<CreateStudentRequest> for NewStudent {
    fn from(value: CreateStudentRequest) -> Self {
        Self {
            email: value.email,
            phone_number: value.phone_number,
            created_at: Some(Local::now().naive_utc()),
            updated_at: Some(Local::now().naive_utc()),
            ..Default::default()
        }
    }
}