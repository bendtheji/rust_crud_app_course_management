# Course Management API

## Technologies used
1) Rust
2) Actix-web
3) Diesel
4) Postgresql
5) Docker

## Running on docker-compose

Run the following command and Docker will spin up two containers:
1) Actix web server for REST API
2) Postgresql for storing data

```
docker compose up -d
```
This will run the application in detached mode.

To stop the application:
```
docker compose down
```

## Running on your local machine
### Setup
You'll need to install some tools beforehand in order to run it locally on your machine.

Rust: https://www.rust-lang.org/tools/install

PostgreSQL: https://www.postgresql.org/download/

Before proceeding to setup Diesel, ensure your PostgreSQL instance is started.

You may to install the diesel CLI using this command:
```
cargo install diesel_cli --no-default-features --features postgres
```

Then you may need to run this command to create the DB and its tables in your PostgreSQL instance:
```
diesel setup
```

Once the DB and its tables are setup, then run this command:
```
cargo run
```

## APIs

The APIs endpoints for fulfilling the uses cases stated in the assignment description can be found under the `/students-courses` resource section.


### Postman
Import the "Course Management Minden AI.postman_collection.json" file into Postman to get the list of available endpoints.


### Students

`GET /students`

Query Params:
 - `email`

Response Body:
```
{ "id": 1, "email": "taylor.swift@gmail.com"}
```

Example:
```
curl GET 'http://127.0.0.1:8080/students?email=taylor.swift@gmail.com'
```


`POST /students`

Request Body:
```
{ "email": "taylor.swift@gmail.com" }
```
Response Body:
```
{ "id": 1, "email": "taylor.swift@gmail.com"}
```

Example:
```
curl POST -H 'Content-Type: application/json' -d '{"email": "taylor.swift@gmail.com"}' http://127.0.0.1:8080/students
```

### Courses
`GET /courses`

Query Params:
- `name`

Response Body:
```
{ "id": 1, "name": "mathematics"}
```

Example:
```
curl GET 'http://127.0.0.1:8080/courses?name=mathematics'
```


`POST /courses`

Request Body:
```
{ "name": "mathematics"}
```
Response Body:
```
{ "id": 1, "name": "mathematics"}
```

Example:
```
curl POST -H 'Content-Type: application/json' -d '{"name": "mathematics"}' http://127.0.0.1:8080/courses
```

### Student Courses
`GET /students-courses/student`

Query Params:
- `student_email`

Response Body:
```
["mathematics", "physics"]
```

Example:
```
curl GET 'http://127.0.0.1:8080/students-courses/student?student_email=taylor.swift@gmail.com'
```

`GET /students-courses/course`

Query Params:
- `course_name`

Response Body:
```
["taylor.swift@gmail.com", "kanye.west@gmail.com"]
```

Example:
```
curl GET 'http://127.0.0.1:8080/students-courses/course?course_name=physics'
```

`POST /students-courses`

Request Body:
```
{ "student_email": "kanye.west@gmail.com", "course_name": "physics" }
```
Response Body:
```
"student sign up successful"
```

Example:
```
curl POST -H 'Content-Type: application/json' -d '{"student_email": "kanye.west@gmail.com", "course_name": "physics"}' http://127.0.0.1:8080/students-courses
```

`DELETE /students-courses`

Request Body:
```
{ "student_email": "kanye.west@gmail.com", "course_name": "physics" }
```
Response Body:
```
"sign-up deleted successfully"
```

Example:
```
curl DELETE -H 'Content-Type: application/json' -d '{"student_email": "kanye.west@gmail.com", "course_name": "physics"}' http://127.0.0.1:8080/students-courses
```

## Tests

To run the tests, you'll need to start PostgreSQL, and you'll need Rust and Cargo as well.

Once PostgreSQL is started, you can run:
```
cargo test
```

### Testing for API endpoints
Currently the testing used for API endpoints are not the best solution for testing the different flows.

A better alternative would be to use mock objects for the DAO objects so that we don't have a heavy dependency on the DB.