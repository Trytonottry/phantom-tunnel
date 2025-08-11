FROM rust:1.78-slim AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/client /usr/local/bin/client
COPY --from=builder /app/target/release/gui /usr/local/bin/gui

COPY certs /app/certs
COPY keys /app/keys
COPY config /app/config

EXPOSE 8443 4001 1080 1420

CMD ["client"]