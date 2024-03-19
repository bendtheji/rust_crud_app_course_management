use diesel::prelude::*;

use crate::db::courses::models::Course;
use crate::db::schema::courses;

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
            let course = create_course(conn, "mathematics")?;
            assert_eq!("mathematics", course.name);
            Ok(())
        });
    }

    #[test]
    fn test_get_course() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            create_course(conn, "mathematics")?;
            let course = get_course(conn, "mathematics")?;
            assert_eq!("mathematics", course.name);
            Ok(())
        });
    }

    #[test]
    #[should_panic]
    fn test_get_course_not_found() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            create_course(conn, "mathematics")?;
            let student = create_course(conn, "physics")?;
            Ok(())
        });
    }

    #[test]
    #[should_panic]
    fn test_create_course_not_unique_name() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            create_course(conn, "mathematics")?;
            create_course(conn, "mathematics")?;
            Ok(())
        });
    }
}