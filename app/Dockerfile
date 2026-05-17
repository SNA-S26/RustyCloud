# Use nightly to support Edition 2024
FROM rustlang/rust:nightly-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Run Stage (remains the same)
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libcap2-bin && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/RustyCloud ./
COPY --from=builder /app/templates ./templates
RUN setcap 'cap_net_bind_service=+ep' ./RustyCloud
EXPOSE 80
# Change this in your Dockerfile
ENTRYPOINT ["./RustyCloud"]
# This acts as the default if no number is provided
CMD ["80"]