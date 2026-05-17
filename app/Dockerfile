FROM rust:1.75-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
# 1. Install libcap2-bin to modify binary permissions
RUN apt-get update && apt-get install -y libcap2-bin && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY templates ./templates
COPY --from=builder /app/target/release/RustyCloud ./

# 2. Give the binary permission to bind to port 80
RUN setcap 'cap_net_bind_service=+ep' ./RustyCloud

EXPOSE 80
# 3. Ensure it runs with the port argument
CMD ["./RustyCloud", "80"]