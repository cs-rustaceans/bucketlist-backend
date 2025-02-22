FROM rust:1.85.0-bullseye

WORKDIR /code

RUN apt update
RUN apt install libmariadb-dev -y
RUN cargo install diesel_cli@2.0.1

COPY *.toml .
COPY *.lock .

COPY src src

RUN cargo build --release

CMD ["./target/release/bucketlist-backend"]