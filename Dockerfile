FROM rust:1.76.0
LABEL authors="Jaeson Fan"

# Switch working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created by Docker in case it does not exit already
WORKDIR /app

# Install system dependencies for linking configuration
RUN apt update && apt install lld clang -y

# Copy all files from working environment to Docker image
COPY . .

# Enable sqlx offline mode
ENV SQLX_OFFLINE true

# Build binary with release profile
RUN cargo build --release

# When `docker run` is executed, launch the binary
ENTRYPOINT ["./target/release/zero2prod"]
