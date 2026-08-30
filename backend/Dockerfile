FROM rust:1-slim-bookworm AS builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /build/target/release/trading_lab_backend ./trading_lab_backend
EXPOSE 8000
CMD ["./trading_lab_backend"]
