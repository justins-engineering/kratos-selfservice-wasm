FROM rust:1 AS build
WORKDIR /usr/src/app

RUN set -ex \
  && apt-get update \
  && apt-get install curl unzip

RUN set -ex \
  && curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
  && cargo binstall dioxus-cli

COPY --link Cargo.lock Cargo.toml Dioxus.toml package.json bun.lock tailwind.css ./
COPY --link ./assets ./assets
COPY --link ./.cargo ./.cargo
COPY --link ./src ./src

# Install bun
RUN set -ex \
  && curl -fsSL https://bun.com/install | bash \
  && mv ~/.bun/bin/bun /usr/bin/

# Install js deps and process tailwind.css
RUN set -ex \
  && printenv \
  && bun install --production \
  && bun x @tailwindcss/cli -i ./tailwind.css -o ./assets/tailwind.css --minify

# Add wasm32 rust target and caro install
RUN set -ex \
  && rustup target add wasm32-unknown-unknown \
  && cargo install --path .

# Dioxus bundle/build
RUN set -ex && dx bundle --web --release

# Link favicon.ico to assets/favicon.ico
RUN set -ex \
  && ln /usr/src/app/dist/public/assets/favicon.ico /usr/src/app/dist/public/favicon.ico

# Add httpd rules
RUN set -ex \
  && echo "E404:index.html" > httpd.conf \
  && echo ".wasm:application/wasm" >> httpd.conf

FROM lipanski/docker-static-website:latest
COPY --from=build /usr/src/app/httpd.conf /etc/httpd.conf
COPY --from=build /usr/src/app/dist/public .

CMD ["/busybox-httpd", "-f", "-v", "-p", "4455"]
