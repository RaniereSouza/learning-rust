# at the date of this comment (2024-09-03),
# Rust 1.79 is the most recent version available in the Microsoft devcontainers registry
FROM mcr.microsoft.com/devcontainers/rust:1.0.14-bullseye

# the `cargo generate` command accused that $USER was missing
ENV USER=root

RUN apt-get update

# installing bat, 'cause it's fun :D
RUN apt-get install wget -y
RUN wget -P /tmp https://github.com/sharkdp/bat/releases/download/v0.12.1/bat_0.12.1_amd64.deb
RUN dpkg -i /tmp/bat_0.12.1_amd64.deb

# istalling The Silver Searcher 'cause it's awesome
RUN apt-get install silversearcher-ag

# preparing for WASM web application bundling
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
# pkg-config required for wasm-bindgen-cli
RUN apt-get install pkg-config -y
# libssl-dev required for wasm-bindgen-cli
RUN apt-get install libssl-dev -y
RUN cargo install --locked wasm-bindgen-cli
# installing cargo-watch as fallback for trunk watch mode
RUN cargo install cargo-watch

# installing X11 graphic library for Slint to use as fallback in place of Wayland
# (this is probably a specific problem for Slint on WSL Ubuntu, but registering it here anyway)
RUN apt-get install libxkbcommon-x11-0
