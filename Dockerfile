FROM rust:1.77.2-bullseye

WORKDIR /code

RUN apt update
RUN apt install libmariadb-dev -y
RUN cargo install diesel_cli@2.0.1

COPY *.toml .
COPY *.lock .

COPY src src
COPY migrations migrations

RUN cargo build --release

RUN touch .env

COPY docker-run.sh .
RUN chmod u+x docker-run.sh
CMD ["./docker-run.sh"]