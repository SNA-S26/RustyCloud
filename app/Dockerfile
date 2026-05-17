FROM rust:1.85-slim AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

# Install libcap2-bin to allow port 80 binding without root
RUN apt-get update && apt-get install -y libcap2-bin && rm -rf /var/lib/apt/lists/*

WORKDIR /app
# Templates are required by your Axum handlers
COPY templates ./templates
COPY --from=builder /app/target/release/RustyCloud ./

# Grant the capability to bind to privileged ports (<1024)
RUN setcap 'cap_net_bind_service=+ep' ./RustyCloud

EXPOSE 80

# Run the binary and pass '80' as the first argument for main.rs
CMD ["./RustyCloud", "80"]