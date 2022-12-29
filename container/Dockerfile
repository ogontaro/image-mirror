FROM rust:1.66 AS base
ENV LANG="C.UTF-8" \
    TZ="Asia/Tokyo"
RUN ln -sf /usr/share/zoneinfo/Asia/Tokyo /etc/localtime
ENV APP_HOME /workspace
WORKDIR ${APP_HOME}
COPY Cargo.* ./


FROM base AS workspace
COPY Cargo.* ./
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build


FROM base AS builder
COPY Cargo.* ./
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release && cargo clean # for crates.io caching
COPY ../rust ./
RUN cargo build --release


FROM ubuntu:22.04 AS release
ENV LANG="C.UTF-8" \
    TZ="Asia/Tokyo"
RUN ln -sf /usr/share/zoneinfo/Asia/Tokyo /etc/localtime
USER nobody
COPY ./container/entrypoint.sh /entrypoint.sh
COPY --from=builder /workspace/target/release/main /bin/rust-utils
ENTRYPOINT ["/entrypoint.sh"]
