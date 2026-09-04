.PHONY: check mdbook

check:
	cargo fmt --check
	cargo clippy --all-targets --locked -- -D warnings
	cargo check --all-targets --locked
	cargo test --all-targets --locked
	$(MAKE) -C book compile

mdbook:
	$(MAKE) -C book serve
