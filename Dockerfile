FROM rust:alpine3.20 AS builder

RUN apk add --no-cache musl-dev openssl-dev

COPY src /src/
COPY Cargo.toml /Cargo.toml

RUN cargo build --release


FROM alpine:latest AS runner

COPY --from=builder /target/release/app /usr/local/bin/

ENTRYPOINT ["app"]
CMD ["app"]
