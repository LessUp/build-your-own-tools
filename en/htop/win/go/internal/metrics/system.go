package metrics

import (
	"context"
	"fmt"

	"github.com/shirou/gopsutil/v3/cpu"
	"github.com/shirou/gopsutil/v3/mem"
)

// SystemStats 表示系统级指标
// KISS: 只保留最常用的数据
// CPU 百分比与内存使用情况

// SystemStats holds CPU and memory usage snapshot
// All percentages are 0-100
// Memory sizes are in bytes
// Keep it simple for UI rendering

type SystemStats struct {
	CPUPercent     float64
	MemTotal       uint64
	MemUsed        uint64
	MemUsedPercent float64
}

// GetSystemStats 采集系统 CPU 与内存使用情况
// 注意：cpu.Percent(0, false) 为即时返回，不会阻塞 1s
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
