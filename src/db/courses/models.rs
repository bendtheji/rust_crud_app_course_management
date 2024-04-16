use diesel::{Identifiable, Insertable, Queryable, Selectable};

use crate::api::courses::types::CreateCourseRequest;
use crate::schema::courses;

#[derive(Identifiable, Queryable, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = courses)]
pub struct Course {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Clone, Default)]
#[diesel(table_name = courses)]
pub struct NewCourse {
    pub name: String,
}

impl From<CreateCourseRequest> for NewCourse {
    fn from(value: CreateCourseRequest) -> Self {
        Self {
            name: value.name,
            ..Default::default()
        }
    }
}