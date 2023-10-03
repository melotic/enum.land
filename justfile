alias b := build

build:
  cargo build

func: build
  cp target/debug/enumland func/handler
  cd func && func start

func-dev:
  cargo watch -w src -s 'just func'

func-build:
  cargo build --release --target x86_64-unknown-linux-musl
  cp target/x86_64-unknown-linux-musl/release/enumland func/handler
