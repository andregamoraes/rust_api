# Use the official Rust image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy Cargo files first (to leverage Docker caching)
COPY Cargo.toml Cargo.lock ./

# Install Diesel CLI
RUN cargo install diesel_cli --no-default-features --features postgres

# Copy the entire project
COPY . .

# Build the Rust application
RUN cargo build --release

# Run the application
CMD ["./target/release/rust_api"]
