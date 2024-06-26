use diesel::prelude::*;

use crate::db::courses::models::{Course, NewCourse};
use crate::schema::courses;

pub fn create_course(conn: &mut PgConnection, new_course: NewCourse) -> QueryResult<Course> {
    diesel::insert_into(courses::table)
        .values(&new_course)
        .returning(Course::as_returning())
        .get_result(conn)
}

pub fn get_course(conn: &mut PgConnection, name: &str) -> QueryResult<Course> {
    courses::table.filter(courses::name.eq(name))
        .select(Course::as_select())
        .first(conn)
}

pub fn delete_course(conn: &mut PgConnection, course_name: &str) -> QueryResult<usize> {
    let predicate = courses::name.eq(course_name);
    diesel::delete(courses::table.filter(predicate)).execute(conn)
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
            let new_course = NewCourse { name: String::from("mathematics"), ..Default::default() };
            let course = create_course(conn, new_course)?;
            assert_eq!("mathematics", course.name);
            Ok(())
        });
    }

    #[test]
    fn test_get_course() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let new_course = NewCourse { name: String::from("mathematics"), ..Default::default() };
            create_course(conn, new_course)?;
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
            let new_course = NewCourse { name: String::from("mathematics"), ..Default::default() };
            let course = create_course(conn, new_course)?;
            let _course = get_course(conn, "physics")?;
            Ok(())
        });
    }

    #[test]
    #[should_panic]
    fn test_create_course_not_unique_name() {
        let mut conn = db::establish_connection();
        conn.test_transaction::<_, Error, _>(|conn| {
            let new_course = NewCourse { name: String::from("mathematics"), ..Default::default() };
            create_course(conn, new_course.clone())?;
            create_course(conn, new_course)?;
            Ok(())
        });
    }
}