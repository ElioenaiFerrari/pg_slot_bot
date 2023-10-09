# Use the official Rust image as the base image
FROM rust:1.72.1-alpine

# Set the working directory in the container
WORKDIR /app

# Install any necessary system dependencies
RUN apk update && apk add --no-cache \
  build-base \
  musl-dev \
  openssl \
  libressl-dev

# Copy the Cargo.toml and src/ files into the container
COPY Cargo.toml .
COPY src/ ./src/
COPY assets/ ./assets/

# Build your Rust application
RUN cargo build --release

# Expose any ports your application may use (replace with your actual port)
EXPOSE 4001

# Start your application (replace with your actual binary name)
CMD ["./target/release/pg_slot_bot"]
