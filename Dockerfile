FROM rust:alpine3.20 AS builder

ARG TARGETPLATFORM
ARG BUILDPLATFORM

RUN apk add --no-cache musl-dev openssl-dev

COPY src /src/
COPY Cargo.toml /Cargo.toml

RUN cargo build --release


FROM alpine:3.20.2 AS runner

# Copy the appliation binary from the builder stage
COPY --from=builder /target/release/app /usr/local/bin/

# Create a new user 'appuser' so we don't run the application as root
RUN addgroup -S appuser && adduser -S appuser -G appuser

# Change ownership of the application binary to 'service'
RUN chown appuser:appuser /usr/local/bin/app

# Switch to the 'service' user
USER appuser

ENTRYPOINT ["app"]
CMD ["app"]
