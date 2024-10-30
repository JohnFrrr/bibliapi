# Builder Stage
FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev gcc

WORKDIR /workspace

COPY . .

# Build the Rust application
RUN cargo install --path .

# Final Stage
FROM alpine

RUN adduser -D app_user

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder --chown=app_user:app_user /usr/local/cargo/bin/bibliapi .

# Ensure the binary is executable
RUN chmod +x ./bibliapi

USER app_user

# Temporary command for debugging
# CMD ["sh", "-c", "ls -l /app && ./bibliapi"]

# Uncomment the line below when ready to run the application
CMD ["./bibliapi"]
