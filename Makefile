TARGET := vellum

.PHONY: all clean tui gui test fmt clippy

all:
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

clippy:
	cargo clippy --release -- -D warnings
