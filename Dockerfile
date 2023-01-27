FROM rust:1.66 AS base
ENV LANG="C.UTF-8" \
  TZ="Asia/Tokyo"
RUN ln -sf /usr/share/zoneinfo/Asia/Tokyo /etc/localtime
ENV APP_HOME /workspace
WORKDIR ${APP_HOME}
RUN rustup component add clippy rustfmt rust-src rust-analysis
RUN cargo install cargo-edit
RUN cargo install cargo-watch
RUN cargo install cargo-expand
RUN cargo install cargo-script
RUN cargo install cargo-outdated
RUN cargo install cargo-audit
RUN cargo install cargo-make


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
COPY ./src ./src
RUN cargo build --release

FROM quay.io/skopeo/stable AS release
ENV LANG="C.UTF-8" \
  TZ="Asia/Tokyo"
RUN ln -sf /usr/share/zoneinfo/Asia/Tokyo /etc/localtime
RUN yum update -y libselinux; \
    yum install -y jq curl git gh; \
    yum clean all
COPY container/entrypoint.sh /entrypoint.sh
COPY --from=builder /workspace/target/release/image-mirror /bin/image-mirror
ENTRYPOINT ["/entrypoint.sh"]
