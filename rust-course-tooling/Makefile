MSRV ?= 1.85

.PHONY: check msrv mdbook slides bench

check:
	cargo fmt --check
	cargo clippy --all-targets --no-default-features --locked -- -D warnings
	cargo clippy --all-targets --all-features --locked -- -D warnings
	cargo check --all-targets --locked
	cargo check --all-targets --no-default-features --locked
	cargo check --all-targets --all-features --locked
	cargo test --lib --tests --examples --locked
	cargo test --lib --tests --examples --no-default-features --locked
	cargo test --lib --tests --examples --all-features --locked
	cargo test --doc --no-default-features --locked
	cargo test --doc --all-features --locked
	cargo doc --no-deps --no-default-features --locked
	cargo doc --no-deps --all-features --locked
	cargo bench --no-run --locked
	$(MAKE) -C book compile

msrv:
	cargo +$(MSRV) check --all-targets --no-default-features --locked
	cargo +$(MSRV) check --all-targets --all-features --locked
	cargo +$(MSRV) test --lib --tests --examples --no-default-features --locked
	cargo +$(MSRV) test --lib --tests --examples --all-features --locked
	cargo +$(MSRV) test --doc --no-default-features --locked
	cargo +$(MSRV) test --doc --all-features --locked

bench:
	cargo bench --bench search --locked

mdbook:
	$(MAKE) -C book serve

slides:
	$(MAKE) -C slides serve
