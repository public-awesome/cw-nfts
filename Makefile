.PHONY: lint schema

lint:
	cargo clippy --all-targets -- -D warnings

schema:
	sh scripts/schema.sh