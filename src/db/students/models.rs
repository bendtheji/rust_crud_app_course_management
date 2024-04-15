use diesel::{Identifiable, Insertable, Queryable, Selectable};

use crate::api::students::types::CreateStudentRequest;
use crate::schema::students;

#[derive(Identifiable, Queryable, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = students)]
pub struct Student {
    pub id: i32,
    pub email: String,
}

#[derive(Insertable, Clone, Default)]
#[diesel(table_name = students)]
pub struct NewStudent {
    pub email: String,
}

impl From<CreateStudentRequest> for NewStudent {
    fn from(value: CreateStudentRequest) -> Self {
        Self {
            email: value.email,
            ..Default::default()
        }
    }
}