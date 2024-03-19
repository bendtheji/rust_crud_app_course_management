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

