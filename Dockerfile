# Build Stage
# on mac install brew install filosottile/musl-cross/musl-cross
FROM ekidd/rust-musl-builder:latest AS builder
ADD --chown=rust:rust . ./
RUN cargo build --release

# RUN USER=root cargo new custom_k8s_scheduler
# WORKDIR /usr/src/custom_k8s_scheduler
# COPY Cargo.toml Cargo.lock ./
# RUN cargo build --release

# COPY src ./src
# RUN cargo install --target x86_64-unknown-linux-musl --path .


FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/scheduler \
    /usr/local/bin/
CMD /usr/local/bin/scheduler
# FROM scratch
# COPY --from=builder /usr/local/cargo/bin/scheduler .
# USER 1000
# EXPOSE 8080
# ENTRYPOINT ["/scheduler"]
