FROM rust:slim-buster AS buildstage
WORKDIR /build
RUN /bin/sh -c set -eux;\
    rustup component add rustfmt;\
    apt-get update;\
    apt-get install -y --no-install-recommends git librocksdb-dev libssl-dev pkg-config clang;\
    rm -rf /var/lib/apt/lists/*;
COPY . /build/
RUN cargo build --release
FROM debian:buster-slim
COPY --from=buildstage /build/target/release/cloud-op /usr/bin/
CMD ["controller"]
