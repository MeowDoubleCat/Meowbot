ARG BASE_IMAGE=ekidd/rust-musl-builder
FROM ${BASE_IMAGE} as builder
COPY --chown=rust:rust . ./
RUN cargo build --release

FROM alpine
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/meowbot /usr/local/bin/
CMD /usr/local/bin/meowbot