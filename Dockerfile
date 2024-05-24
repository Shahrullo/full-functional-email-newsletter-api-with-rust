# Builder stage
# Get official RUST image
FROM lukemathwalker/cargo-chef:latest-rust-1.78.0 AS chef
# Switch working directory to `app`
WORKDIR /app
# Install the required system dependencies
RUN apt update && apt install lld clang -y

FROM chef as planner
# Copy all files
COPY . .
# Compute a lock-like file for the project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# BUild project dependencesi, nor the application
RUN cargo chef cook --release --recipe-path recipe.json
# If dependency tree stays the same, all layers should be cached
COPY . .
# Define environment variable for sqlx
ENV SQLX_OFFLINE true
# Build the binary
RUN cargo build --release --bin email_newsletter

# Runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Install dependencies
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder
COPY --from=builder /app/target/release/email_newsletter email_newsletter
# Configuration file is needed at runtime
COPY configuration configuration
ENV APP_ENVIRONMENT production
# Launch the binary
ENTRYPOINT ["./email_newsletter"]