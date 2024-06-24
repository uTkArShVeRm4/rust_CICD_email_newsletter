# Use the official Rust image
FROM rust:latest AS builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application
RUN cargo install --path .

# Use a smaller base image for runtime
FROM debian:buster-slim

# Copy the built binary from the previous stage
COPY --from=builder /usr/local/cargo/bin/axum_webserver /usr/local/bin/axum_webserver

# Expose the port the app runs on
EXPOSE 8080

# Command to run the executable
CMD ["axum_webserver"]
