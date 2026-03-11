package metrics

import (
	"context"
	"runtime"
	"sort"
	"sync"
	"time"

	"github.com/shirou/gopsutil/v3/process"
)

// ProcessInfo 表示单个进程快照
// KISS: 只展示最常用字段

type ProcessInfo struct {
	PID        int32
	Name       string
	CPUPercent float64
	MemRSS     uint64
	MemPercent float32
	CreateTime int64 // milliseconds since epoch
}

type cpuCache struct {
	Total float64   // user + system seconds
	TS    time.Time // wall clock
}

type Collector struct {
	mu     sync.Mutex
	cache  map[int32]cpuCache
	lastTS time.Time
	numCPU int
}

func NewCollector() *Collector {
	return &Collector{
		cache:  make(map[int32]cpuCache),
		numCPU: getNumCPU(),
	}
}

// getNumCPU 单独成函数，便于将来替换/测试
func getNumCPU() int {
	return runtime.NumCPU()
}

// Snapshot 扫描当前进程并计算 CPU%, 内存等
func (c *Collector) Snapshot(ctx context.Context) ([]ProcessInfo, error) {
	procs, err := process.ProcessesWithContext(ctx)
	if err != nil {
		return nil, err
	}
	now := time.Now()

	c.mu.Lock()
	defer c.mu.Unlock()

	infos := make([]ProcessInfo, 0, len(procs))

	for _, p := range procs {
		pid := p.Pid

		name, err := p.NameWithContext(ctx)
		if err != nil || name == "" {
			// 忽略取名失败的进程（权限或瞬态进程）
			continue
		}

		// Memory
		var rss uint64
		var memPct float32
		if mi, err := p.MemoryInfoWithContext(ctx); err == nil && mi != nil {
			rss = mi.RSS
		}
		if mp, err := p.MemoryPercentWithContext(ctx); err == nil {
			memPct = mp
		}

		// CPU percent by delta
		cpuPct := 0.0
		if ct, err := p.TimesWithContext(ctx); err == nil && ct != nil {
			curTotal := ct.User + ct.System
			prev, ok := c.cache[pid]
			if ok {
				deltaProc := curTotal - prev.Total
				deltaWall := now.Sub(prev.TS).Seconds()
				if deltaProc > 0 && deltaWall > 0 && c.numCPU > 0 {
					cpuPct = (deltaProc / (deltaWall * float64(c.numCPU))) * 100.0
					if cpuPct < 0 {
						cpuPct = 0
					}
				}
			}
			// update cache
			c.cache[pid] = cpuCache{Total: curTotal, TS: now}
		}

		ctm, _ := p.CreateTimeWithContext(ctx)

		infos = append(infos, ProcessInfo{
			PID:        pid,
			Name:       name,
			CPUPercent: cpuPct,
			MemRSS:     rss,
			MemPercent: memPct,
			CreateTime: ctm,
		})
	}

	// 排序：默认按 CPU% 降序
	sort.Slice(infos, func(i, j int) bool { return infos[i].CPUPercent > infos[j].CPUPercent })

	// 清理已退出进程的缓存，避免无限增长
	alive := make(map[int32]struct{}, len(infos))
	for _, info := range infos {
		alive[info.PID] = struct{}{}
	}
	for pid := range c.cache {
		if _, ok := alive[pid]; !ok {
			delete(c.cache, pid)
		}
	}

	c.lastTS = now
	return infos, nil
}
