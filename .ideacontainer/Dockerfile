FROM rust:1.66 AS base
ARG IDEURL=https://download.jetbrains.com/idea/ideaIU-2022.3.1.tar.gz

ENV LANG="C.UTF-8" \
  TZ="Asia/Tokyo"
ENV APP_HOME /workspace
RUN mkdir $APP_HOME
RUN ln -sf /usr/share/zoneinfo/Asia/Tokyo /etc/localtime

# install idea server
RUN mkdir /idea
WORKDIR /idea
RUN apt-get update && apt-get install -y openjdk-11-jdk git curl unzip procps time
RUN curl -fsSL -o ide.tar.gz https://download.jetbrains.com/idea/ideaIU-2022.3.1.tar.gz && \
  mkdir ide && \
  tar xfz ide.tar.gz --strip-components=1 -C ide && \
  rm ide.tar.gz

# install idea plugin
# RUN time ide/bin/remote-dev-server.sh installPlugins $APP_HOME com.intellij.ja # 日本語化
RUN time ide/bin/remote-dev-server.sh installPlugins $APP_HOME org.rust.lang # rust
RUN time ide/bin/remote-dev-server.sh installPlugins $APP_HOME com.github.copilot # copilot
RUN time ide/bin/remote-dev-server.sh installPlugins $APP_HOME name.kropp.intellij.makefile # Makefile
RUN time ide/bin/remote-dev-server.sh installPlugins $APP_HOME org.intellij.plugins.markdown # Markdown
RUN time ide/bin/remote-dev-server.sh installPlugins $APP_HOME com.jetbrains.sh # shellscript
RUN time ide/bin/remote-dev-server.sh installPlugins $APP_HOME izhangzhihao.rainbow.brackets # Rainbow Brackets
RUN time ide/bin/remote-dev-server.sh installPlugins $APP_HOME com.intellij.plugins.watcher # File Watcher

# setup rust
## Installed separately because of slow compilation.
RUN rustup component add clippy rustfmt rust-src rust-analysis
RUN cargo install cargo-edit
RUN cargo install cargo-watch
RUN cargo install cargo-expand
RUN cargo install cargo-script
RUN cargo install cargo-outdated
RUN cargo install cargo-audit
RUN cargo install cargo-make

# setup rust workspace
WORKDIR ${APP_HOME}
COPY Cargo.* ./
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build

CMD /idea/ide/bin/remote-dev-server.sh run $APP_HOME --listenOn 0.0.0.0 --port 5993
