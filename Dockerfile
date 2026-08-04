FROM rust:1.95-bookworm AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && printf 'fn main() {}\n' > src/main.rs && cargo build --release

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates curl \
    && groupadd --gid 1000 ma \
    && useradd --uid 1000 --gid ma --create-home --shell /usr/sbin/nologin ma \
    && install -d --owner=ma --group=ma --mode=0700 /home/ma/.config/ma \
    && install -d --owner=ma --group=ma --mode=0700 /home/ma/.local/share \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/ma /usr/local/bin/ma
COPY docker/entrypoint.sh /usr/local/bin/ma-entrypoint

RUN chmod 0755 /usr/local/bin/ma-entrypoint

VOLUME ["/home/ma/.config/ma"]

USER ma

ENV HOME=/home/ma \
    XDG_CONFIG_HOME=/home/ma/.config \
    XDG_DATA_HOME=/home/ma/.local/share

ENTRYPOINT ["ma-entrypoint"]
CMD ["--kubo-rpc-url", "http://kubo:5001", "--status-bind", "0.0.0.0:5003"]