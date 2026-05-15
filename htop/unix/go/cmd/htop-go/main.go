package main

import (
	"log"

	"htop-go/internal/ui"
	"htop-shared-go/metrics"
)

func main() {
	collector := metrics.NewCollector()
	app := ui.New(collector)
	if err := app.Run(); err != nil {
		log.Fatalf("Application exited: %v", err)
	}
}
