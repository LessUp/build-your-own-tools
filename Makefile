.PHONY: build-dos2unix build-gzip-rust build-htop-unix-rust build-htop-win-rust \
	build-gzip-go build-htop-win-go build-rust build-go build-all \
	lint-rust lint-go lint-all test-rust test-go test-all \
	fmt-rust fmt-go fmt-all clean

# ── Build ─────────────────────────────────────────────

build-dos2unix:
	cargo build --release --manifest-path dos2unix/Cargo.toml

build-gzip-rust:
	cargo build --release --manifest-path gzip/rust/Cargo.toml

build-htop-unix-rust:
	cargo build --release --manifest-path htop/unix/rust/Cargo.toml

build-htop-win-rust:
	cargo build --release --manifest-path htop/win/rust/Cargo.toml

build-gzip-go:
	$(MAKE) -C gzip/go build

build-htop-win-go:
	$(MAKE) -C htop/win/go build

build-rust: build-dos2unix build-gzip-rust build-htop-unix-rust build-htop-win-rust

build-go: build-gzip-go build-htop-win-go

build-all: build-rust build-go

# ── Format ────────────────────────────────────────────

fmt-rust:
	cargo fmt --all

fmt-go:
	gofmt -w gzip/go htop/win/go

fmt-all: fmt-rust fmt-go

# ── Lint ──────────────────────────────────────────────

lint-rust:
	cargo fmt --all -- --check
	cargo clippy --all-targets -- -D warnings

lint-go:
	@echo "==> gzip/go"
	$(MAKE) -C gzip/go fmt
	go vet -C gzip/go ./...
	@echo "==> htop/win/go"
	gofmt -w htop/win/go
	go vet -C htop/win/go ./...

lint-all: lint-rust lint-go

# ── Test ──────────────────────────────────────────────

test-rust:
	cargo test --all --all-features

test-go:
	go test -C gzip/go ./...
	go test -C htop/win/go ./...

test-all: test-rust test-go

# ── Clean ─────────────────────────────────────────────

clean:
	cargo clean
