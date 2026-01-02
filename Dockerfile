FROM rust:alpine3.22 AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

# Copy source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM alpine:3.22

# Copy binary from builder
COPY --from=builder /app/target/release/lingua-service /usr/local/bin/

# Non-root user
RUN addgroup -g 1000 lingua && \
  adduser -D -u 1000 -G lingua lingua

USER lingua

EXPOSE 3030

CMD ["lingua-service"]
