.PHONY: build clean fmt

BIN_DIR := bin

build:
	mkdir -p $(BIN_DIR)
	go build -o $(BIN_DIR)/htop-win-go ./cmd/htop-win-go

clean:
	rm -rf $(BIN_DIR)

fmt:
	go fmt ./...
