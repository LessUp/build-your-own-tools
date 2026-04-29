package main

import (
	"log"

	"htop-go/internal/metrics"
	"htop-go/internal/ui"
)

func main() {
	collector := metrics.NewCollector()
	app := ui.New(collector)
	if err := app.Run(); err != nil {
		log.Fatalf("Application exited: %v", err)
	}
}
