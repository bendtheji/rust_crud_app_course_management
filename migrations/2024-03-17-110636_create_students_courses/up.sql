-- Your SQL goes here
CREATE TABLE students_courses (
    student_id INTEGER REFERENCES students (id),
    course_id  INTEGER REFERENCES courses (id),
    PRIMARY KEY (student_id, course_id)
);