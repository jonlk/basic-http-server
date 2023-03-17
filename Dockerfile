FROM rust:1.68.0 as builder

WORKDIR /usr/src/base-http-server

COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim

COPY --from=builder /usr/local/cargo/bin/base-http-server /usr/local/bin/base-http-server

CMD ["base-http-server"]
