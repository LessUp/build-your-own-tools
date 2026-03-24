.PHONY: build clean fmt fmt-check

BIN_DIR := bin

build:
	mkdir -p $(BIN_DIR)
	go build -o $(BIN_DIR)/htop-win-go ./cmd/htop-win-go

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
