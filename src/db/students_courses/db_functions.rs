use diesel::prelude::*;

use crate::db::courses::db_functions as courses_db_functions;
use crate::db::courses::models::Course;
use crate::db::students::db_functions as students_db_functions;
use crate::db::students::models::Student;
use crate::db::students_courses::models::StudentCourse;
use crate::schema::*;

pub fn get_courses_attended_by_student(conn: &mut PgConnection, email: &str) -> QueryResult<Vec<Course>> {
    let student = students_db_functions::get_student(conn, email)?;
    StudentCourse::belonging_to(&student)
        .inner_join(courses::table)
        .select(Course::as_select())
        .load(conn)
}

pub fn get_students_in_course(conn: &mut PgConnection, name: &str) -> QueryResult<Vec<Student>> {
    let course = courses_db_functions::get_course(conn, name)?;
    StudentCourse::belonging_to(&course)
        .inner_join(students::table)
        .select(Student::as_select())
        .load(conn)
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

pub fn delete_student_course(conn: &mut PgConnection, student_id: i32, course_id: i32) -> QueryResult<usize> {
    let predicate = students_courses::student_id.eq(student_id).and(students_courses::course_id.eq(course_id));
    diesel::delete(students_courses::table.filter(predicate)).execute(conn)
}

#[cfg(test)]
mod tests {
    use diesel::{Connection, result::Error};

    use crate::db;
    use crate::db::courses::db_functions as courses_db_functions;
    use crate::db::students::db_functions as students_db_functions;

    use super::*;

    #[test]
    fn test_get_courses_attended_by_students() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let student = students_db_functions::create_student(conn, "some_user@gmail.com")?;
            let course_one = courses_db_functions::create_course(conn, "data science")?;
            let course_two = courses_db_functions::create_course(conn, "machine learning")?;
            create_student_course(conn, student.id, course_one.id)?;
            create_student_course(conn, student.id, course_two.id)?;

            let courses_attended = get_courses_attended_by_student(conn, "some_user@gmail.com")?;
            assert_eq!(vec!["data science", "machine learning"], courses_attended.into_iter().map(|item| item.name).collect::<Vec<String>>());

            Ok(())
        });
    }

    #[test]
    #[should_panic]
    fn test_get_courses_attended_by_students_student_not_found() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let student = students_db_functions::create_student(conn, "some_user@gmail.com")?;
            let course_one = courses_db_functions::create_course(conn, "data science")?;
            let course_two = courses_db_functions::create_course(conn, "machine learning")?;
            create_student_course(conn, student.id, course_one.id)?;
            create_student_course(conn, student.id, course_two.id)?;

            let courses_attended = get_courses_attended_by_student(conn, "some_unknown_user@gmail.com")?;
            assert_eq!(vec!["data science", "machine learning"], courses_attended.into_iter().map(|item| item.name).collect::<Vec<String>>());

            Ok(())
        });
    }

    #[test]
    fn test_get_students_in_course() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let student_one = students_db_functions::create_student(conn, "some_user@gmail.com")?;
            let student_two = students_db_functions::create_student(conn, "some_user_two@gmail.com")?;
            let course = courses_db_functions::create_course(conn, "machine learning")?;
            create_student_course(conn, student_one.id, course.id)?;
            create_student_course(conn, student_two.id, course.id)?;

            let students_in_course = get_students_in_course(conn, "machine learning")?;
            assert_eq!(vec!["some_user@gmail.com", "some_user_two@gmail.com"], students_in_course.into_iter().map(|item| item.email).collect::<Vec<String>>());

            Ok(())
        });
    }

    #[test]
    #[should_panic]
    fn test_get_students_in_course_course_not_found() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let student_one = students_db_functions::create_student(conn, "some_user@gmail.com")?;
            let student_two = students_db_functions::create_student(conn, "some_user_two@gmail.com")?;
            let course = courses_db_functions::create_course(conn, "machine learning")?;
            create_student_course(conn, student_one.id, course.id)?;
            create_student_course(conn, student_two.id, course.id)?;

            let students_in_course = get_students_in_course(conn, "culinary")?;
            assert_eq!(vec!["some_user@gmail.com", "some_user_two@gmail.com"], students_in_course.into_iter().map(|item| item.email).collect::<Vec<String>>());

            Ok(())
        });
    }

    #[test]
    fn test_create_student_course() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let student = students_db_functions::create_student(conn, "some_user@gmail.com")?;
            let course = courses_db_functions::create_course(conn, "machine learning")?;
            create_student_course(conn, student.id, course.id)?;
            Ok(())
        })
    }

    #[test]
    #[should_panic]
    fn test_create_student_course_duplicate() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let student = students_db_functions::create_student(conn, "some_user@gmail.com")?;
            let course = courses_db_functions::create_course(conn, "machine learning")?;
            create_student_course(conn, student.id, course.id)?;
            create_student_course(conn, student.id, course.id)?;
            Ok(())
        })
    }

    #[test]
    fn test_delete_student_course() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let student = students_db_functions::create_student(conn, "some_user@gmail.com")?;
            let course = courses_db_functions::create_course(conn, "machine learning")?;
            create_student_course(conn, student.id, course.id)?;
            delete_student_course(conn, student.id, course.id)?;
            Ok(())
        })
    }
}