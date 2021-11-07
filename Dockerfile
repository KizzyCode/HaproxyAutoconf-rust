# Build the daemon
FROM rust:alpine AS buildenv
COPY ./ /buildroot
RUN cargo build --release --manifest-path /buildroot/Cargo.toml


# Build the real container
FROM alpine
COPY --from=buildenv /buildroot/target/release/haproxy_autoconf /usr/local/bin/haproxy_autoconf

USER nobody
CMD ["/usr/local/bin/haproxy_autoconf"]
