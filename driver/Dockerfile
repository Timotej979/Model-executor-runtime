# Use a Rust base image with Cargo and Rust pre-installed
FROM rust:1.73

# Set the working directory inside the container
WORKDIR /app

# Copy the entire project folder into the container at /app
COPY . .

# Build your Rust project
RUN cargo build --release

# Define the command to run your Rust application
CMD ["./target/release/driver"]