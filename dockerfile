# Use the official Rust image as a base for building the app
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/src/web_server

# Copy the Cargo.toml and Cargo.lock files for caching dependencies
COPY Cargo.toml Cargo.lock ./

# Fetch the dependencies, this helps cache them so they don't need to be fetched again if they haven't changed
RUN cargo build --release
RUN cargo clean

# Copy the source code into the container
COPY . .

# Build the project
RUN cargo build --release

# Use a smaller final image to run the application
FROM debian:bullseye-slim

# Install dependencies like OpenSSL, required by some crates (e.g., reqwest, rust-openssl)
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/web_server/target/release/web_server .

# Expose the port your app will run on
EXPOSE 8080

# Run the application
CMD ["./web_server"]
