FROM rust:1.56 as builder
WORKDIR /usr/src/rustapp
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/start_server /usr/local/bin/rustapp
RUN apt-get update -y; \
    apt-get install libsqlite3-dev -y;
CMD ["rustapp"]