use diesel::{Identifiable, Queryable, Selectable};

use crate::schema::students;

#[derive(Identifiable, Queryable, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = students)]
pub struct Student {
    pub id: i32,
    pub email: String,
}
