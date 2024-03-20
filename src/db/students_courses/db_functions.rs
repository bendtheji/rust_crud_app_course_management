use diesel::prelude::*;

use crate::db::courses::db_functions as courses_db_functions;
use crate::db::courses::db_functions::{CourseDao, CourseImpl};
use crate::db::courses::models::Course;
use crate::db::students::db_functions::{StudentDao, StudentImpl};
use crate::db::students::models::Student;
use crate::db::students_courses::models::StudentCourse;
use crate::schema::*;

trait StudentCourseDao {
    fn get_courses_attended_by_student(&mut self, email: &str) -> QueryResult<Vec<Course>>;
    fn get_students_in_course(&mut self, name: &str) -> QueryResult<Vec<Student>>;
    fn create_student_course(&mut self, student_id: i32, course_id: i32) -> QueryResult<StudentCourse>;
    fn delete_student_course(&mut self, student_id: i32, course_id: i32) -> QueryResult<usize>;
}

struct StudentCourseImpl<'a> {
    conn: &'a mut PgConnection,
}

impl<'a> StudentCourseImpl<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self { Self { conn } }
}

impl StudentCourseDao for StudentCourseImpl<'_> {
    fn get_courses_attended_by_student(&mut self, email: &str) -> QueryResult<Vec<Course>> {
        let mut student_impl = StudentImpl::new(&mut self.conn);
        let student = student_impl.get_student(email)?;
        StudentCourse::belonging_to(&student)
            .inner_join(courses::table)
            .select(Course::as_select())
            .load(self.conn)
    }

    fn get_students_in_course(&mut self, name: &str) -> QueryResult<Vec<Student>> {
        let mut course_impl = CourseImpl::new(&mut self.conn);
        let course = course_impl.get_course(name)?;
        StudentCourse::belonging_to(&course)
            .inner_join(students::table)
            .select(Student::as_select())
            .load(self.conn)
    }

    fn create_student_course(&mut self, student_id: i32, course_id: i32) -> QueryResult<StudentCourse> {
        diesel::insert_into(students_courses::table)
            .values((
                students_courses::student_id.eq(student_id),
                students_courses::course_id.eq(course_id)
            ))
            .returning(StudentCourse::as_returning())
            .get_result(self.conn)
    }

    fn delete_student_course(&mut self, student_id: i32, course_id: i32) -> QueryResult<usize> {
        let predicate = students_courses::student_id.eq(student_id).and(students_courses::course_id.eq(course_id));
        diesel::delete(students_courses::table.filter(predicate)).execute(self.conn)
    }
}

pub fn get_courses_attended_by_student(conn: &mut PgConnection, email: &str) -> QueryResult<Vec<Course>> {
    let mut student_impl = StudentImpl::new(conn);
    let student = student_impl.get_student(email)?;
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
    use crate::db::students::db_functions::StudentImpl;

    use super::*;

    #[test]
    fn test_get_courses_attended_by_students() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let mut student_impl = StudentImpl::new(conn);
            let student = student_impl.create_student("some_user@gmail.com")?;
            let mut course_impl = CourseImpl::new(conn);
            let course_one = course_impl.create_course("data science")?;
            let course_two = course_impl.create_course("machine learning")?;
            let mut student_course_impl = StudentCourseImpl::new(conn);
            student_course_impl.create_student_course(student.id, course_one.id)?;
            student_course_impl.create_student_course(student.id, course_two.id)?;

            let courses_attended = student_course_impl.get_courses_attended_by_student("some_user@gmail.com")?;
            assert_eq!(vec!["data science", "machine learning"], courses_attended.into_iter().map(|item| item.name).collect::<Vec<String>>());

            Ok(())
        });
    }

    #[test]
    #[should_panic]
    fn test_get_courses_attended_by_students_student_not_found() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let mut student_impl = StudentImpl::new(conn);
            let student = student_impl.create_student("some_user@gmail.com")?;
            let mut course_impl = CourseImpl::new(conn);
            let course_one = course_impl.create_course("data science")?;
            let course_two = course_impl.create_course("machine learning")?;
            let mut student_course_impl = StudentCourseImpl::new(conn);
            student_course_impl.create_student_course(student.id, course_one.id)?;
            student_course_impl.create_student_course(student.id, course_two.id)?;

            let courses_attended = student_course_impl.get_courses_attended_by_student("some_unknown_user@gmail.com")?;
            assert_eq!(vec!["data science", "machine learning"], courses_attended.into_iter().map(|item| item.name).collect::<Vec<String>>());

            Ok(())
        });
    }

    #[test]
    fn test_get_students_in_course() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let mut student_impl = StudentImpl::new(conn);
            let student_one = student_impl.create_student("some_user@gmail.com")?;
            let student_two = student_impl.create_student("some_user_two@gmail.com")?;
            let mut course_impl = CourseImpl::new(conn);
            let course = course_impl.create_course("machine learning")?;
            let mut student_course_impl = StudentCourseImpl::new(conn);
            student_course_impl.create_student_course(student_one.id, course.id)?;
            student_course_impl.create_student_course(student_two.id, course.id)?;

            let students_in_course = student_course_impl.get_students_in_course("machine learning")?;
            assert_eq!(vec!["some_user@gmail.com", "some_user_two@gmail.com"], students_in_course.into_iter().map(|item| item.email).collect::<Vec<String>>());

            Ok(())
        });
    }

    #[test]
    #[should_panic]
    fn test_get_students_in_course_course_not_found() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let mut student_impl = StudentImpl::new(conn);
            let student_one = student_impl.create_student("some_user@gmail.com")?;
            let student_two = student_impl.create_student("some_user_two@gmail.com")?;
            let mut course_impl = CourseImpl::new(conn);
            let course = course_impl.create_course("machine learning")?;
            let mut student_course_impl = StudentCourseImpl::new(conn);
            student_course_impl.create_student_course(student_one.id, course.id)?;
            student_course_impl.create_student_course(student_two.id, course.id)?;

            let students_in_course = student_course_impl.get_students_in_course("culinary")?;
            assert_eq!(vec!["some_user@gmail.com", "some_user_two@gmail.com"], students_in_course.into_iter().map(|item| item.email).collect::<Vec<String>>());

            Ok(())
        });
    }

    #[test]
    fn test_create_student_course() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let mut student_impl = StudentImpl::new(conn);
            let student = student_impl.create_student("some_user@gmail.com")?;
            let mut course_impl = CourseImpl::new(conn);
            let course = course_impl.create_course("machine learning")?;
            let mut student_course_impl = StudentCourseImpl::new(conn);
            student_course_impl.create_student_course(student.id, course.id)?;
            Ok(())
        })
    }

    #[test]
    #[should_panic]
    fn test_create_student_course_duplicate() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let mut student_impl = StudentImpl::new(conn);
            let student = student_impl.create_student("some_user@gmail.com")?;
            let mut course_impl = CourseImpl::new(conn);
            let course = course_impl.create_course("machine learning")?;
            let mut student_course_impl = StudentCourseImpl::new(conn);
            student_course_impl.create_student_course(student.id, course.id)?;
            student_course_impl.create_student_course(student.id, course.id)?;
            Ok(())
        })
    }

    #[test]
    fn test_delete_student_course() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let mut student_impl = StudentImpl::new(conn);
            let student = student_impl.create_student("some_user@gmail.com")?;
            let mut course_impl = CourseImpl::new(conn);
            let course = course_impl.create_course("machine learning")?;
            let mut student_course_impl = StudentCourseImpl::new(conn);
            student_course_impl.create_student_course(student.id, course.id)?;
            student_course_impl.delete_student_course(student.id, course.id)?;
            Ok(())
        })
    }
}