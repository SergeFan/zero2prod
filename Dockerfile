FROM lukemathwalker/cargo-chef:latest-rust-1.76.0 as chef

WORKDIR /app

RUN apt update && apt install mold clang -y

# Planner stage
FROM chef as planner

COPY . .

# Compile a lock-like file for the project
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage
FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

# Build project dependencies instead of application
RUN cargo chef cook --release --recipe-path recipe.json

# At this point, if the dependency tree stays the same, all layer should be cached.
COPY . .

# Enable sqlx offline mode
ENV SQLX_OFFLINE true

# Build binary with release profile
RUN cargo build --release --bin zero2prod


# Runtime stage
FROM debian:bookworm-slim AS runtime
LABEL authors="Jaeson Fan"

WORKDIR /app

# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder environment to runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod

# Configuration file is also needed at runtime
COPY configuration configuration

ENV APP_ENVIRONMENT production

# When `docker run` is executed, launch the binary
ENTRYPOINT ["./zero2prod"]
