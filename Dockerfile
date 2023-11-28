FROM rust:1.74-buster as builder

WORKDIR /app
COPY ./central_bank . 

WORKDIR /protocol
COPY ./protocol .


WORKDIR /app

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/cbapi .

CMD ["./cbapi"]

