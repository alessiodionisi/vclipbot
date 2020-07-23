FROM rust:latest AS builder
WORKDIR /usr/src
COPY . .
RUN cargo build --release

FROM debian:latest
COPY --from=builder /usr/src/target/release/vclipbot /usr/local/bin
RUN apt-get update
RUN apt-get install openssl ca-certificates -y
CMD ["vclipbot"]
