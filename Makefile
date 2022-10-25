.PHONY: build lint schema

build:
	sh scripts/build.sh

lint:
	cargo clippy --all-targets -- -D warnings

schema:
	sh scripts/schema.sh
