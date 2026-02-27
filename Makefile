TARGET := vellum

.PHONY: all clean debug fmt fmtc gui lint release test tui

all: clean fmt lint release

clean:
	cargo clean

debug:
	cargo build

fmt:
	cargo fmt

fmtc:
	cargo fmt --check

gui:
	cargo run --release -p vellum-gui

lint:
	cargo clippy --release -- -D warnings

release:
	cargo build --release

test:
	cargo test --release

tui:
	cargo run --release -p vellum-tui