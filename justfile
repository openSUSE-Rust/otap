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

license-lint:
    reuse lint

push-git +EXTRA_ARGS:
    git push origin {{EXTRA_ARGS}}
    git push github {{EXTRA_ARGS}}
    git push sourcehut {{EXTRA_ARGS}}

publish:
    cargo publish --token "${CARGO_REGISTRY_TOKEN}"
