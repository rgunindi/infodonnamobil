FROM rust:1 AS chef

# Gerekli araçları yükle
RUN apt-get update && apt-get install -y \
    software-properties-common \
    gnupg2 \
    curl \
    nano && \
    apt-get clean

# Eksik GPG anahtarını ekle
RUN curl -fsSL https://keyserver.ubuntu.com/pks/lookup?op=get&search=0x871920D1991BC93C | gpg --dearmor -o /usr/share/keyrings/ubuntu-archive-keyring.gpg && \
    echo "deb [signed-by=/usr/share/keyrings/ubuntu-archive-keyring.gpg] http://archive.ubuntu.com/ubuntu jammy main restricted universe multiverse" >> /etc/apt/sources.list && \
    echo "deb [signed-by=/usr/share/keyrings/ubuntu-archive-keyring.gpg] http://archive.ubuntu.com/ubuntu jammy-security main restricted universe multiverse" >> /etc/apt/sources.list && \
    apt-get update && \
    apt-get install -y libwebkit2gtk-4.0-dev pkg-config && \
    apt-get clean

# Cargo Chef yükle
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:${PATH}"

RUN dx bundle --platform web

FROM chef AS runtime
COPY --from=builder /app/target/dx/infodonnaclient/release/web/ /usr/local/app

ENV PORT=8080
ENV IP=0.0.0.0

EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/server", "--port", "${PORT}" ]