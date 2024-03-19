use diesel::{Associations, Identifiable, Queryable, Selectable};

use crate::db::courses::models::Course;
use crate::db::schema::students_courses;
use crate::db::students::models::Student;

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, Clone)]
#[diesel(belongs_to(Student))]
#[diesel(belongs_to(Course))]
#[diesel(table_name = students_courses)]
#[diesel(primary_key(student_id, course_id))]
pub struct StudentCourse {
    pub student_id: i32,
    pub course_id: i32,
}