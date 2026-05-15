package main

import (
	"log"

	"htop-shared-go/metrics"
	"htop-win-go/internal/ui"
)

func main() {
	collector := metrics.NewCollector()
	u := ui.New(collector)
	if err := u.Run(); err != nil {
		log.Fatalf("应用退出: %v", err)
	}
}
