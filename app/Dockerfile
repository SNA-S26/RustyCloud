FROM rust:1.95-slim as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:13-slim

WORKDIR /app
COPY templates ./templates
COPY --from=builder /app/target/release/RustyCloud ./

EXPOSE 80
CMD ["./RustyCloud", "80"]
