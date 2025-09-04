FROM rust:1 AS builder
WORKDIR /usr/src/app

RUN set -ex \
  && apt-get update \
  && apt-get install curl unzip

RUN set -ex rustup target add wasm32-unknown-unknown
RUN set -ex && cargo install dioxus-cli

COPY --link Cargo.lock Cargo.toml Dioxus.toml package.json bun.lock tailwind.css ./
COPY --link ./assets/* ./assets/
COPY --link ./.cargo/* ./.cargo/
COPY --link ./src/main.rs ./src/
COPY --link ./src/components ./src/components/
COPY --link ./src/views ./src/views/

RUN set -ex \
  && rustup target add wasm32-unknown-unknown \
  && cargo install --path .

RUN set -ex \
  && curl -fsSL https://bun.com/install | bash -s "bun-v1.2.21" \
  && mv ~/.bun/bin/bun /usr/bin/

RUN set -ex \
  && printenv \
  && bun install --production \
  && bun x @tailwindcss/cli -i ./tailwind.css -o ./assets/tailwind.css --minify

RUN dx bundle --platform web
