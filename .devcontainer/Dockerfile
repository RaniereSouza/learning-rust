FROM mcr.microsoft.com/devcontainers/rust:bullseye

# the `cargo generate` command accused that $USER was missing
ENV USER=root

RUN apt-get update

# installing bat, 'cause it's fun :D
RUN apt-get install wget -y
RUN wget -P /tmp https://github.com/sharkdp/bat/releases/download/v0.12.1/bat_0.12.1_amd64.deb
RUN dpkg -i /tmp/bat_0.12.1_amd64.deb

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
## pkg-config required for wasm-bindgen-cli
RUN apt-get install pkg-config -y
## libssl-dev required for wasm-bindgen-cli
RUN apt-get install libssl-dev -y
RUN cargo install --locked wasm-bindgen-cli
## using cargo-watch as fallback for trunk
RUN cargo install cargo-watch
