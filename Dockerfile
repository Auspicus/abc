FROM rust:1.56 as builder
WORKDIR /usr/src/rustapp
COPY . .
RUN cargo install --path .
RUN cargo install diesel_cli --no-default-features --features="sqlite"
RUN diesel migration run

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/start_server /usr/local/bin/rustapp
COPY --from=builder /usr/src/rustapp/database.db /var/local/database.db
RUN apt-get update -y; \
    apt-get install libsqlite3-dev -y;
CMD ["rustapp"]