// @generated automatically by Diesel CLI.

diesel::table! {
    courses (id) {
        id -> Int4,
        name -> Varchar,
        course_desc -> Nullable<Varchar>,
    }
}

diesel::table! {
    students (id) {
        id -> Int4,
        email -> Varchar,
        phone_number -> Nullable<Varchar>,
    }
}

diesel::table! {
    students_courses (student_id, course_id) {
        student_id -> Int4,
        course_id -> Int4,
    }
}

diesel::joinable!(students_courses -> courses (course_id));
diesel::joinable!(students_courses -> students (student_id));

diesel::allow_tables_to_appear_in_same_query!(
    courses,
    students,
    students_courses,
);
