# Build stage
FROM rust:1.85 AS builder
# Install dependencies for OpenSSL
RUN apt-get update && apt-get install -y pkg-config libssl-dev openssl
WORKDIR /src
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm
RUN apt-get update && apt-get install -y openssl ca-certificates libssl3 libssl-dev pkg-config openssl

COPY ca.pem /usr/local/share/ca-certificates/

# Update the certificate store
RUN update-ca-certificates

COPY --from=builder /src/target/release/todo /app/todo

EXPOSE 8000
ENTRYPOINT ["/app/todo"]