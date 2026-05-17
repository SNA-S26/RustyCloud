# --- Build Stage ---
FROM rust:1.81-slim AS builder
WORKDIR /app
# Copy everything from the 'app' folder
COPY . .
RUN cargo build --release

# --- Run Stage ---
FROM debian:bookworm-slim
# Install capability manager to allow port 80 binding
RUN apt-get update && apt-get install -y libcap2-bin && rm -rf /var/lib/apt/lists/*

WORKDIR /app
# Copy binary and templates
COPY --from=builder /app/target/release/RustyCloud ./
COPY --from=builder /app/templates ./templates

# Grant permission to use port 80
RUN setcap 'cap_net_bind_service=+ep' ./RustyCloud

EXPOSE 80
# Use the JSON form for CMD to ensure signals are passed correctly
CMD ["./RustyCloud", "80"]