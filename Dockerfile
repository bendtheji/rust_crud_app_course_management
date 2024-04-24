# Use the official Rust image from Docker Hub
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Build dependencies without the source code to utilize Docker's layer caching
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

# Copy the source code into the container
COPY . .

# Build the Rust application
RUN cargo build --release


# Start a new stage and use a smaller base image
FROM ubuntu:latest

# Install OpenSSL which is required by Actix Web
RUN apt-get update && \
    apt-get install -y libssl-dev libpq-dev && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the built executable from the previous stage
COPY --from=builder /usr/src/app/target/release/rust_crud_app_course_management .

# Expose the port your Actix Web application listens on
EXPOSE 8080

# Command to run the application
CMD ["./rust_crud_app_course_management"]
