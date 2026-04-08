FROM ubuntu:22.04

ARG BUILD_TIME

# Metadata labels
LABEL dockerfile_hash="fc19cbc0e949d171557fba424c4a4273595df4394953454b9ceb02b85cee42ed"
LABEL build_time="${BUILD_TIME}"

RUN apt update && apt upgrade -y && apt install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  gvfs gvfs-libs gvfs-backends \
  inkscape \
  && useradd -m -s /bin/bash appuser \
  && su - appuser -c 'curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable' \
  && su - appuser -c 'cargo install tauri-cli;' \
  && rm -rf /home/appuser/.cargo/registry/cache && rm -rf /home/appuser/.cargo/registry/src \
  && apt clean

USER appuser

WORKDIR /home/appuser

