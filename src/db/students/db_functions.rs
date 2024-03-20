use diesel::prelude::*;

use crate::db::students::models::Student;
use crate::schema::students;

pub trait StudentDao {
    fn create_student(&mut self, email: &str) -> QueryResult<Student>;
    fn get_student(&mut self, email: &str) -> QueryResult<Student>;
}

pub struct StudentImpl<'a> {
    conn: &'a mut PgConnection,
}

impl<'a> StudentImpl<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }
}

impl StudentDao for StudentImpl<'_> {
    fn create_student(&mut self, email: &str) -> QueryResult<Student> {
        diesel::insert_into(students::table)
            .values(students::email.eq(email))
            .returning(Student::as_returning())
            .get_result(self.conn)
    }

    fn get_student(&mut self, email: &str) -> QueryResult<Student> {
        students::table.filter(students::email.eq(email))
            .select(Student::as_select())
            .first(self.conn)
    }
}


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
            let mut student_impl = StudentImpl::new(conn);
            let student = student_impl.create_student("test_user@gmail.com")?;
            assert_eq!("test_user@gmail.com", student.email);
            Ok(())
        });
    }

    #[test]
    fn test_get_student() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let mut student_impl = StudentImpl::new(conn);
            student_impl.create_student("test_user@gmail.com")?;
            let student = student_impl.get_student("test_user@gmail.com")?;
            assert_eq!("test_user@gmail.com", student.email);
            Ok(())
        });
    }

    #[test]
    #[should_panic]
    fn test_get_student_not_found() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let mut student_impl = StudentImpl::new(conn);
            student_impl.create_student("test_user@gmail.com")?;
            let student = student_impl.get_student("test_user_two@gmail.com")?;
            Ok(())
        });
    }

    #[test]
    #[should_panic]
    fn test_create_student_not_unique_email() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let mut student_impl = StudentImpl::new(conn);
            student_impl.create_student("test_user@gmail.com")?;
            student_impl.create_student("test_user@gmail.com")?;
            Ok(())
        });
    }
}