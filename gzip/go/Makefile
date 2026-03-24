.PHONY: build build-linux install clean fmt fmt-check

BIN_DIR := bin

build:
	mkdir -p $(BIN_DIR)
	go build -o $(BIN_DIR)/gzip-go ./cmd/gzip-go

# Cross-compile for Linux amd64 (from any host)
build-linux:
	mkdir -p $(BIN_DIR)
	GOOS=linux GOARCH=amd64 go build -o $(BIN_DIR)/gzip-go ./cmd/gzip-go

install: build
	install -m 0755 $(BIN_DIR)/gzip-go /usr/local/bin/gzip-go

clean:
	rm -rf $(BIN_DIR)

fmt:
	gofmt -w .

fmt-check:
	@changed=$$(gofmt -l .); if [ -n "$$changed" ]; then \
		echo "Go files not formatted:"; \
		echo "$$changed"; \
		exit 1; \
	fi
