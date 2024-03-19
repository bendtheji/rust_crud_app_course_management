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
        .expect("database URL should be valid path to Postgres instance")
}

pub fn create_student(conn: &mut PgConnection, email: &str) -> QueryResult<Student> {
    diesel::insert_into(students::table)
        .values(students::email.eq(email))
        .returning(Student::as_returning())
        .get_result(conn)
}

pub fn create_course(conn: &mut PgConnection, name: &str) -> QueryResult<Course> {
    diesel::insert_into(courses::table)
        .values(courses::name.eq(name))
        .returning(Course::as_returning())
        .get_result(conn)
}

pub fn create_student_course(conn: &mut PgConnection, student_id: i32, course_id: i32) -> QueryResult<StudentCourse> {
    diesel::insert_into(students_courses::table)
        .values((
            students_courses::student_id.eq(student_id),
            students_courses::course_id.eq(course_id)
        ))
        .returning(StudentCourse::as_returning())
        .get_result(conn)
}

pub fn get_student(conn: &mut PgConnection, email: &str) -> QueryResult<Student> {
    students::table.filter(students::email.eq(email))
        .select(Student::as_select())
        .first(conn)
}

pub fn get_course(conn: &mut PgConnection, name: &str) -> QueryResult<Course> {
    courses::table.filter(courses::name.eq(name))
        .select(Course::as_select())
        .first(conn)
}

pub fn get_courses_attended_by_student(conn: &mut PgConnection, email: &str) -> QueryResult<Vec<Course>> {
    let student = get_student(conn, email)?;
    StudentCourse::belonging_to(&student)
        .inner_join(courses::table)
        .select(Course::as_select())
        .load(conn)
}

pub fn get_students_in_course(conn: &mut PgConnection, name: &str) -> QueryResult<Vec<Student>> {
    let course = get_course(conn, name)?;
    StudentCourse::belonging_to(&course)
        .inner_join(students::table)
        .select(Student::as_select())
        .load(conn)
}

pub fn delete_student_course(conn: &mut PgConnection, student_email: &str, course_name: &str) -> QueryResult<usize> {
    let student = get_student(conn, student_email)?;
    let course = get_course(conn, course_name)?;

    let predicate = students_courses::student_id.eq(student.id).and(students_courses::course_id.eq(course.id));

    diesel::delete(students_courses::table.filter(predicate)).execute(conn)
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    use diesel::{Connection, result::Error};

    use crate::db::{create_student, establish_connection, get_student};

    #[test]
    fn test_create_student() {
        let mut conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let student = create_student(conn, "test_user@gmail.com")?;
            assert_eq!("test_user@gmail.com", student.email);
            Ok(())
        });
    }

    #[test]
    fn test_get_student() {
        let mut conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            create_student(conn, "test_user@gmail.com")?;
            let student = get_student(conn, "test_user@gmail.com")?;
            assert_eq!("test_user@gmail.com", student.email);
            Ok(())
        });
    }

    #[test]
    #[should_panic]
    fn test_get_student_not_found() {
        let mut conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            create_student(conn, "test_user@gmail.com")?;
            let student = get_student(conn, "test_user_two@gmail.com")?;
            Ok(())
        });
    }

    #[test]
    #[should_panic]
    fn test_create_student_not_unique_email() {
        let mut conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            create_student(conn, "test_user@gmail.com")?;
            create_student(conn, "test_user@gmail.com")?;
            Ok(())
        });
    }
}