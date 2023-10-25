# ========== CARGO CHEF ==========
FROM rust:1.73 as chef

# Install the `cargo-chef` tool
# We only pay the installation cost once, 
# it will be cached from the second build onwards
RUN cargo install cargo-chef

# Set the working directory inside the container
WORKDIR /usr/src/migrations


# ========== PLANNER ==========
FROM chef AS planner

# Copy the dependencies "recipe" from the previous stage
COPY . .

# Create a "recipe" for your project's dependencies
RUN cargo chef prepare --recipe-path recipe.json


# ========== BUILDER ==========
FROM chef AS builder

# Copy the dependencies "recipe" from the previous stage
COPY --from=planner /usr/src/migrations/recipe.json recipe.json

# Install system dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev pkg-config libpq-dev libcairo2-dev && \
    rm -rf /var/lib/apt/lists/*

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin migrations


# ========== RUNTIME ==========
FROM debian:bookworm-slim AS runtime

# Set the working directory inside the container
WORKDIR /usr/local/bin

# Copy the binary from the "builder" stage
COPY --from=builder /usr/src/migrations/target/release/migrations .

# Define the command to run your application
ENTRYPOINT ["./driver"]