FROM rust:1.65.0-slim-buster as builder
RUN apt-get update \
    && apt-get install -y libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/* 

FROM builder as build
COPY . /opt/
WORKDIR /opt/
RUN cargo install --path .

FROM debian:buster-slim as exec
RUN apt-get update \
    && apt-get install -y libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/* 
COPY --from=build /usr/local/cargo/bin/wifi-qr-rs /usr/local/bin/wifi-qr-rs
RUN /usr/local/bin/wifi-qr-rs -h
ENTRYPOINT ["/usr/local/bin/wifi-qr-rs"]
