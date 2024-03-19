use diesel::prelude::*;

use crate::schema::{students, courses, students_courses};

#[derive(Identifiable, Queryable, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = students)]
pub struct Student {
    pub id: i32,
    pub email: String,
}


#[derive(Identifiable, Queryable, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = courses)]
pub struct Course {
    pub id: i32,
    pub name: String,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, Clone)]
#[diesel(belongs_to(Student))]
#[diesel(belongs_to(Course))]
#[diesel(table_name = students_courses)]
#[diesel(primary_key(student_id, course_id))]
pub struct StudentCourse {
    pub student_id: i32,
    pub course_id: i32,
}