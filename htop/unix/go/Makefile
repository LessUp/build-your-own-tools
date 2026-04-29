.PHONY: build build-linux build-darwin install clean fmt fmt-check test

BIN_DIR := bin

build:
	mkdir -p $(BIN_DIR)
	go build -o $(BIN_DIR)/htop-go ./cmd/htop-go

build-linux:
	GOOS=linux GOARCH=amd64 go build -o $(BIN_DIR)/htop-go ./cmd/htop-go

build-darwin:
	GOOS=darwin GOARCH=amd64 go build -o $(BIN_DIR)/htop-go ./cmd/htop-go

install: build
	install -m 0755 $(BIN_DIR)/htop-go /usr/local/bin/htop-go

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

test:
	go test ./...
