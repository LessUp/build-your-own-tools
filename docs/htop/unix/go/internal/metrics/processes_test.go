package metrics

import (
	"sort"
	"testing"
)

func TestNewCollector(t *testing.T) {
	c := NewCollector()
	if c == nil {
		t.Fatal("NewCollector() returned nil")
	}
	if c.cache == nil {
		t.Error("cache not initialized")
	}
	if c.numCPU <= 0 {
		t.Error("numCPU should be positive")
	}
}

func TestProcessInfoSorting(t *testing.T) {
	// Test ProcessInfo sorting by CPU descending
	infos := []ProcessInfo{
		{PID: 1, Name: "low", CPUPercent: 10.0},
		{PID: 2, Name: "high", CPUPercent: 50.0},
		{PID: 3, Name: "mid", CPUPercent: 30.0},
	}

	// Sort by CPU descending (same logic as in Snapshot)
	sort.Slice(infos, func(i, j int) bool {
		return infos[i].CPUPercent > infos[j].CPUPercent
	})

	if infos[0].PID != 2 {
		t.Errorf("expected PID 2 at index 0, got PID %d", infos[0].PID)
	}
	if infos[1].PID != 3 {
		t.Errorf("expected PID 3 at index 1, got PID %d", infos[1].PID)
	}
	if infos[2].PID != 1 {
		t.Errorf("expected PID 1 at index 2, got PID %d", infos[2].PID)
	}
}

func TestProcessInfoFields(t *testing.T) {
	// Test ProcessInfo struct fields
	info := ProcessInfo{
		PID:        1234,
		Name:       "test-process",
		CPUPercent: 25.5,
		MemRSS:     1024000,
		MemPercent: 5.0,
		CreateTime: 1704067200000,
	}

	if info.PID != 1234 {
		t.Errorf("expected PID 1234, got %d", info.PID)
	}
	if info.Name != "test-process" {
		t.Errorf("expected Name 'test-process', got %s", info.Name)
	}
	if info.CPUPercent != 25.5 {
		t.Errorf("expected CPUPercent 25.5, got %f", info.CPUPercent)
	}
}
