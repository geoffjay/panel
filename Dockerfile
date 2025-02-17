FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    bash \
    curl \
    libwebkit2gtk-4.1-dev \
    libappindicator3-dev \
    librsvg2-dev \
    patchelf \
    build-essential \
    libssl-dev \
    unzip
RUN curl -fsSL https://bun.sh/install | bash
RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain stable -y

ENV PATH="/root/.cargo/bin:/root/.bun/bin:${PATH}"

ADD . /app
WORKDIR /app

RUN bun install
