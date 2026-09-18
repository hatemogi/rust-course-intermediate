.PHONY: check check-examples mdbook

check:
	cargo fmt --check
	cargo clippy --all-targets --locked -- -D warnings
	cargo check --all-targets --locked
	cargo test --all-targets --locked
	$(MAKE) check-examples
	$(MAKE) -C book compile

check-examples:
	@set -e; for source in examples/*.rs; do \
		example=$$(basename "$$source" .rs); \
		echo "예제 실행: $$example"; \
		cargo run --quiet --locked --example "$$example"; \
	done

mdbook:
	$(MAKE) -C book serve
