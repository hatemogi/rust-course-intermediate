.PHONY: check

check:
	cargo fmt --check
	cargo clippy --all-targets --locked -- -D warnings
	cargo test --all-targets --locked
	python3 test_mdbook.py
