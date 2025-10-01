ARG RUST_VERSION=1.82.0
ARG APP_NAME=arti_whois

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git openssl-dev openssl libssl3 libcrypto3 libgcrypt openssl-libs-static ca-certificates

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
cargo build --locked --release && \
cp ./target/release/$APP_NAME /bin/server

FROM alpine:3.22 AS final

USER root
COPY --from=build /bin/server /bin/

EXPOSE 8016

ENV RUST_BACKTRACE=1
ENV RUST_LOG=debug

CMD ["/bin/server"]
