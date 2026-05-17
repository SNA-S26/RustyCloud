# ... (Builder stage remains the same)

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libcap2-bin && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/RustyCloud ./
COPY --from=builder /app/templates ./templates

# Grant permission to bind to low ports (like 80) if needed, 
# though 8080 doesn't strictly require this.
RUN setcap 'cap_net_bind_service=+ep' ./RustyCloud

# Inform Docker that the container listens on 8080 by default
EXPOSE 8080

ENTRYPOINT ["./RustyCloud"]

# Change the default CMD to 8080 to match your test suite
CMD ["8080"]