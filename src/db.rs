use std::env;

use diesel::{prelude::*, r2d2};
use diesel::pg::PgConnection;
use dotenvy::dotenv;

use crate::models::{Course, Student, StudentCourse};
use crate::schema::*;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn initialize_db_pool() -> DbPool {
    dotenv().ok();
    let conn_spec = env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}

pub fn new_student(conn: &mut PgConnection, email: &str) -> Student {
    diesel::insert_into(students::table)
        .values(students::email.eq(email))
        .returning(Student::as_returning())
        .get_result(conn)
        .expect("Error saving new student")
}

pub fn new_course(conn: &mut PgConnection, name: &str) -> Course {
    diesel::insert_into(courses::table)
        .values(courses::name.eq(name))
        .returning(Course::as_returning())
        .get_result(conn)
        .expect("Error saving new course")
}

pub fn new_students_courses(conn: &mut PgConnection, student_id: i32, course_id: i32) -> StudentCourse {
    diesel::insert_into(students_courses::table)
        .values((
            students_courses::student_id.eq(student_id),
            students_courses::course_id.eq(course_id)
        ))
        .returning(StudentCourse::as_returning())
        .get_result(conn)
        .expect("Error saving new course")
}

pub fn get_student_by_email(conn: &mut PgConnection, email: &str) -> Student {
    students::table.filter(students::email.eq(email))
        .select(Student::as_select())
        .first(conn)
        .expect("Error fetching student")
}

pub fn get_course_by_name(conn: &mut PgConnection, name: &str) -> Course {
    courses::table.filter(courses::name.eq(name))
        .select(Course::as_select())
        .first(conn)
        .expect("Error fetching course")
}

pub fn get_courses_attended_by_student(conn: &mut PgConnection, email: &str) -> Vec<Course> {
    let student = get_student_by_email(conn, email);

    let courses = StudentCourse::belonging_to(&student)
        .inner_join(courses::table)
        .select(Course::as_select())
        .load(conn)
        .expect("issue fetching courses tied to student");

    courses
}

pub fn get_students_in_course(conn: &mut PgConnection, name: &str) -> Vec<Student> {
    let course = get_course_by_name(conn, name);
    println!("fetched course: {:?}", course);

    let students = StudentCourse::belonging_to(&course)
        .inner_join(students::table)
        .select(Student::as_select())
        .load(conn)
        .expect("issue fetching students in course");

    students
}

pub fn cancel_sign_up(conn: &mut PgConnection, student_email: &str, course_name: &str) {
    let student = get_student_by_email(conn, student_email);
    let course = get_course_by_name(conn, course_name);

    let predicate = students_courses::student_id.eq(student.id).and(students_courses::course_id.eq(course.id));

    diesel::delete(students_courses::table.filter(predicate)).execute(conn).expect("issue deleting signup");
}