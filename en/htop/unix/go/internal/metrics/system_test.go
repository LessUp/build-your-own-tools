package metrics

import (
	"context"
	"testing"
)

func TestGetSystemStats(t *testing.T) {
	ctx := context.Background()
	stats, err := GetSystemStats(ctx)
	if err != nil {
		t.Skipf("GetSystemStats failed (may need permissions): %v", err)
	}

	if stats.MemTotal <= 0 {
		t.Error("MemTotal should be positive")
	}
	if stats.MemUsedPercent < 0 || stats.MemUsedPercent > 100 {
		t.Errorf("MemUsedPercent out of range: %f", stats.MemUsedPercent)
	}
	if stats.CPUPercent < 0 || stats.CPUPercent > 100 {
		t.Errorf("CPUPercent out of range: %f", stats.CPUPercent)
	}
}

func TestSystemStatsStruct(t *testing.T) {
	// Test SystemStats struct default values
	var stats SystemStats

	if stats.MemTotal != 0 {
		t.Error("default MemTotal should be 0")
	}
	if stats.MemUsed != 0 {
		t.Error("default MemUsed should be 0")
	}
	if stats.MemUsedPercent != 0 {
		t.Error("default MemUsedPercent should be 0")
	}
	if stats.CPUPercent != 0 {
		t.Error("default CPUPercent should be 0")
	}
}
