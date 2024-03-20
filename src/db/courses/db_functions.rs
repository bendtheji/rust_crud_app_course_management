use diesel::prelude::*;

use crate::db::courses::models::Course;
use crate::schema::courses;

pub trait CourseDao {
    fn create_course(&mut self, name: &str) -> QueryResult<Course>;
    fn get_course(&mut self, name: &str) -> QueryResult<Course>;
}

pub struct CourseImpl<'a> {
    conn: &'a mut PgConnection,
}

impl<'a> CourseImpl<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self { Self { conn } }
}

impl CourseDao for CourseImpl<'_> {
    fn create_course(&mut self, name: &str) -> QueryResult<Course> {
        diesel::insert_into(courses::table)
            .values(courses::name.eq(name))
            .returning(Course::as_returning())
            .get_result(self.conn)
    }

    fn get_course(&mut self, name: &str) -> QueryResult<Course> {
        courses::table.filter(courses::name.eq(name))
            .select(Course::as_select())
            .first(self.conn)
    }
}

pub fn create_course(conn: &mut PgConnection, name: &str) -> QueryResult<Course> {
    diesel::insert_into(courses::table)
        .values(courses::name.eq(name))
        .returning(Course::as_returning())
        .get_result(conn)
}

pub fn get_course(conn: &mut PgConnection, name: &str) -> QueryResult<Course> {
    courses::table.filter(courses::name.eq(name))
        .select(Course::as_select())
        .first(conn)
}

#[cfg(test)]
mod tests {
    use diesel::{Connection, result::Error};

    use crate::db;

    use super::*;

    #[test]
    fn test_create_course() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let mut course_impl = CourseImpl::new(conn);
            let course = course_impl.create_course("mathematics")?;
            assert_eq!("mathematics", course.name);
            Ok(())
        });
    }

    #[test]
    fn test_get_course() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let mut course_impl = CourseImpl::new(conn);
            course_impl.create_course("mathematics")?;
            let course = course_impl.get_course("mathematics")?;
            assert_eq!("mathematics", course.name);
            Ok(())
        });
    }

    #[test]
    #[should_panic]
    fn test_get_course_not_found() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let mut course_impl = CourseImpl::new(conn);
            course_impl.create_course("mathematics")?;
            let student = course_impl.create_course("physics")?;
            Ok(())
        });
    }

    #[test]
    #[should_panic]
    fn test_create_course_not_unique_name() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let mut course_impl = CourseImpl::new(conn);
            course_impl.create_course("mathematics")?;
            course_impl.create_course("mathematics")?;
            Ok(())
        });
    }
}