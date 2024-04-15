use diesel::prelude::*;

use crate::db::students::models::{NewStudent, Student};
use crate::schema::students;

pub fn create_student(conn: &mut PgConnection, new_student: NewStudent) -> QueryResult<Student> {
    diesel::insert_into(students::table)
        .values(&new_student)
        .returning(Student::as_returning())
        .get_result(conn)
}

pub fn get_student(conn: &mut PgConnection, email: &str) -> QueryResult<Student> {
    students::table.filter(students::email.eq(email))
        .select(Student::as_select())
        .first(conn)
}

pub fn delete_student(conn: &mut PgConnection, student_email: &str) -> QueryResult<usize> {
    let predicate = students::email.eq(student_email);
    diesel::delete(students::table.filter(predicate)).execute(conn)
}

#[cfg(test)]
mod tests {
    use diesel::{Connection, result::Error};

    use crate::db;

    use super::*;

    #[test]
    fn test_create_student() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let new_student = NewStudent { email: String::from("test_user@gmail.com"), ..Default::default() };
            let student = create_student(conn, new_student)?;
            assert_eq!("test_user@gmail.com", student.email);
            Ok(())
        });
    }

    #[test]
    fn test_get_student() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let new_student = NewStudent { email: String::from("test_user@gmail.com"), ..Default::default() };
            create_student(conn, new_student)?;
            let student = get_student(conn, "test_user@gmail.com")?;
            assert_eq!("test_user@gmail.com", student.email);
            Ok(())
        });
    }

    #[test]
    #[should_panic]
    fn test_get_student_not_found() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let new_student = NewStudent { email: String::from("test_user@gmail.com"), ..Default::default() };
            create_student(conn, new_student)?;
            let _student = get_student(conn, "test_user_two@gmail.com")?;
            Ok(())
        });
    }

    #[test]
    #[should_panic]
    fn test_create_student_not_unique_email() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let new_student = NewStudent { email: String::from("test_user@gmail.com"), ..Default::default() };
            create_student(conn, new_student.clone())?;
            create_student(conn, new_student)?;
            Ok(())
        });
    }
}