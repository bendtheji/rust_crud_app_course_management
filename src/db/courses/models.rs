use diesel::{Identifiable, Queryable, Selectable};

use crate::db::schema::courses;

#[derive(Identifiable, Queryable, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = courses)]
pub struct Course {
    pub id: i32,
    pub name: String,
}