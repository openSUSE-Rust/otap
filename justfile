#!/usr/bin/just

dev:
    zellij -l .zellij/layout.kdl

build:
    cargo build --release --all-features

debug:
    cargo build --all-features

check-format:
    cargo fmt -- --check

test:
    cargo test --all-features

publish:
    cargo publish --token "${CARGO_REGISTRY_TOKEN}"
