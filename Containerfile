FROM rust:1.79-alpine AS builder

# Install protoc and other deps
RUN apk add --no-cache protobuf-dev build-base curl openssl binaryen jq

# Required for building the WASM module
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Compile and build the web-assembly project
WORKDIR /usr/src/mypkg
COPY . .

RUN wasm-pack build --target nodejs --out-name extension --no-pack

# fix main
RUN mv /usr/src/mypkg/pkg/extension.js /usr/src/mypkg/pkg/extension.cjs

FROM scratch

COPY --from=builder /usr/src/mypkg/pkg/ /extension/pkg
COPY --from=builder /usr/src/mypkg/package.json /extension/
COPY --from=builder /usr/src/mypkg/icon.png /extension/
COPY --from=builder /usr/src/mypkg/README.md /extension/


LABEL org.opencontainers.image.title="Podman WASM" \
        org.opencontainers.image.description="Podman WASM Extension" \
        org.opencontainers.image.vendor="axel7083" \
        io.podman-desktop.api.version=">= 1.14.0"
