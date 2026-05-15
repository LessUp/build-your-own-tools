package metrics

import (
	"context"
	"fmt"

	"github.com/shirou/gopsutil/v3/cpu"
	"github.com/shirou/gopsutil/v3/mem"
)

// SystemStats holds CPU and memory usage snapshot.
// All percentages are 0-100. Memory sizes are in bytes.
type SystemStats struct {
	CPUPercent     float64
	MemTotal       uint64
	MemUsed        uint64
	MemUsedPercent float64
}

// GetSystemStats collects system CPU and memory usage.
func GetSystemStats(ctx context.Context) (SystemStats, error) {
	var st SystemStats

	// CPU
	cpus, err := cpu.PercentWithContext(ctx, 0, false)
	if err != nil {
		return st, fmt.Errorf("cpu percent: %w", err)
	}
	if len(cpus) > 0 {
		st.CPUPercent = cpus[0]
	}

	// Memory
	vm, err := mem.VirtualMemoryWithContext(ctx)
	if err != nil {
		return st, fmt.Errorf("virtual memory: %w", err)
	}
	st.MemTotal = vm.Total
	st.MemUsed = vm.Used
	st.MemUsedPercent = vm.UsedPercent

	return st, nil
}
