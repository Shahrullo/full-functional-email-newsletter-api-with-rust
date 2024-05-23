# Get official RUST image
FROM rust:1.78.0

# Switch working directory to `app`
WORKDIR /app

# Install the required system dependencies
RUN apt update && apt install lld clang -y

# Copy all files
COPY . .

# Build the binary
RUN cargo build --release

# Launch the binary
ENTRYPOINT ["./target/release/email_newsletter"]