use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateStudentCourseRequest {
    pub student_email: String,
    pub course_name: String,
}

#[derive(Deserialize)]
pub struct DeleteStudentCourseRequest {
    pub student_email: String,
    pub course_name: String,
}
