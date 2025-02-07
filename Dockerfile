FROM rust:latest AS builder

WORKDIR /usr/src/nebula

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim AS runtime

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/nebula

COPY --from=builder /usr/src/nebula/target/release/nebulacrypto /usr/bin/nebula

EXPOSE 8080

ENTRYPOINT ["nebula"]
