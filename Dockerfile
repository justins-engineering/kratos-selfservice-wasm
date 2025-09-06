FROM rust:1 AS build
WORKDIR /usr/src/app

RUN set -ex \
  && apt-get update \
  && apt-get install curl unzip

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

RUN set -ex \
  && dx bundle --platform web \
  && echo "E404:index.html\n.wasm:application/wasm" > httpd.conf

FROM lipanski/docker-static-website:latest
COPY --from=build /usr/src/app/httpd.conf /etc/httpd.conf
COPY --from=build /usr/src/app/dist/public/index.html .
COPY --from=build /usr/src/app/dist/public/assets/* ./assets/
CMD ["/busybox-httpd", "-f", "-v", "-p", "4455"]
