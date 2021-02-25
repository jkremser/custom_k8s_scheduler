# Build Stage
# on mac install brew install filosottile/musl-cross/musl-cross
FROM rust:1.50.0 AS builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new custom_k8s_scheduler
WORKDIR /usr/src/custom_k8s_scheduler
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .


# FROM gcr.io/distroless/static:nonroot
# COPY --chown=nonroot:nonroot ./scheduler /app/
FROM scratch
COPY --from=builder /usr/local/cargo/bin/scheduler .
USER 1000
EXPOSE 8080
ENTRYPOINT ["/scheduler"]
