# Stage 1: Build
FROM rustlang/rust:nightly-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Run
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libcap2-bin && rm -rf /var/lib/apt/lists/*
WORKDIR /app
# These paths are correct AS LONG AS templates/ is in build context
COPY --from=builder /app/target/release/RustyCloud ./
COPY --from=builder /app/templates ./templates

RUN setcap 'cap_net_bind_service=+ep' ./RustyCloud
EXPOSE 8080
ENTRYPOINT ["./RustyCloud"]
CMD ["8080"]