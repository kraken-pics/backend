FROM rust:1.62.0 AS builder

WORKDIR /app

COPY . . 

RUN cargo build --release

FROM gcr.io/distroless/base-debian10

WORKDIR /

COPY --from=builder app/target/release /release

USER nonroot:nonroot

CMD ["./release/kraken"]