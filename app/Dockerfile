# --- Build Stage ---
FROM rust:1.89-slim AS builder
WORKDIR /app

# Context is 'app/', so these files are in the current directory
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY templates ./templates

RUN cargo build --release

# --- Run Stage ---
FROM debian:bookworm-slim
# Install libcap2-bin for port 80 permissions
RUN apt-get update && apt-get install -y libcap2-bin && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary and templates from the builder
COPY --from=builder /app/target/release/RustyCloud ./
COPY --from=builder /app/templates ./templates

# Set permissions for port 80
RUN setcap 'cap_net_bind_service=+ep' ./RustyCloud

EXPOSE 80

# This ensures the binary runs from /app, so it sees /app/templates
ENTRYPOINT ["./RustyCloud"]
CMD ["80"]