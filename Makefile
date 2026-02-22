TARGET := vellum

.PHONY: all build clean tui gui test fmt fmtc lint

all: clean fmt lint build

build:
	cargo build --release

clean:
	cargo clean

tui:
	cargo run --release -p vellum-tui

gui:
	cargo run --release -p vellum-gui

test:
	cargo test --release

fmt:
	cargo fmt

fmtc:
	cargo fmt --check

lint:
	cargo clippy --release -- -D warnings
