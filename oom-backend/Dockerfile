FROM rust:latest as builder

WORKDIR /app

COPY Cargo.* ./
COPY src ./src
COPY oom-api ./oom-api
COPY oom-db ./oom-db
COPY oom-model ./oom-model
COPY oom-schema ./oom-schema

RUN cargo build --release

FROM debian:stretch as runner

WORKDIR /app

RUN apt-get update -y && apt-get install default-libmysqlclient-dev libssl-dev ca-certificates -y
RUN mkdir /app/bin
RUN mkdir /app/var
ENV PATH $PATH:/app/bin

COPY --from=builder \
  /app/target/release/oom-backend \
  /app/bin/oom-backend

EXPOSE 8080

CMD ["oom-backend", "-p", "8080"]
