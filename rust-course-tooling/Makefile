MSRV ?= 1.85
CLIPPY_FLAGS = -D warnings -A clippy::map-all-any-identity

.PHONY: check msrv mdbook slides mdslide

check:
	cargo fmt --check
	# 07_clippy.rs는 강의에서 진단을 읽기 위해 이 lint를 의도적으로 남깁니다.
	cargo clippy --all-targets --no-default-features --locked -- $(CLIPPY_FLAGS)
	cargo clippy --all-targets --all-features --locked -- $(CLIPPY_FLAGS)
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
	$(MAKE) -C book compile
	$(MAKE) -C ../mdslide check COURSE=$(CURDIR)

msrv:
	cargo +$(MSRV) check --all-targets --no-default-features --locked
	cargo +$(MSRV) check --all-targets --all-features --locked
	cargo +$(MSRV) test --lib --tests --examples --no-default-features --locked
	cargo +$(MSRV) test --lib --tests --examples --all-features --locked
	cargo +$(MSRV) test --doc --no-default-features --locked
	cargo +$(MSRV) test --doc --all-features --locked

mdbook:
	$(MAKE) -C book serve

slides:
	$(MAKE) -C slides serve

mdslide:
	$(MAKE) -C ../mdslide serve COURSE=$(CURDIR)
