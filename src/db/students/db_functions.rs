use diesel::prelude::*;
use crate::db::schema::students;
use crate::db::students::models::Student;


pub fn create_student(conn: &mut PgConnection, email: &str) -> QueryResult<Student> {
    diesel::insert_into(students::table)
        .values(students::email.eq(email))
        .returning(Student::as_returning())
        .get_result(conn)
}

pub fn get_student(conn: &mut PgConnection, email: &str) -> QueryResult<Student> {
    students::table.filter(students::email.eq(email))
        .select(Student::as_select())
        .first(conn)
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
            let student = create_student(conn, "test_user@gmail.com")?;
            assert_eq!("test_user@gmail.com", student.email);
            Ok(())
        });
    }

    #[test]
    fn test_get_student() {
        let mut conn = db::establish_connection();
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
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            create_student(conn, "test_user@gmail.com")?;
            let student = get_student(conn, "test_user_two@gmail.com")?;
            Ok(())
        });
    }

    #[test]
    #[should_panic]
    fn test_create_student_not_unique_email() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            create_student(conn, "test_user@gmail.com")?;
            create_student(conn, "test_user@gmail.com")?;
            Ok(())
        });
    }
}