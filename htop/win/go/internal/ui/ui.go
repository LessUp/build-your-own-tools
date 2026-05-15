package ui

import (
	"context"
	"fmt"
	"math"
	"runtime"
	"sort"
	"strings"
	"time"

	"github.com/gdamore/tcell/v2"
	"github.com/rivo/tview"

	"htop-shared-go/metrics"
)

// UI 是简单的 TUI 壳，显示系统指标和进程表
// KISS: 仅提供按 CPU% 排序的列表与退出按键

type UI struct {
	app    *tview.Application
	header *tview.Flex
	title  *tview.TextView
	cpuBar *tview.TextView
	memBar *tview.TextView
	procs  *tview.TextView
	table  *tview.Table
	footer *tview.TextView

	collector *metrics.Collector
	ctx       context.Context
	cancel    context.CancelFunc
	sort      sortMode
}

func colorByPercent(p float64) string {
	if p >= 85 {
		return "red"
	}
	if p >= 60 {
		return "yellow"
	}
	return "green"
}

func renderBar(prefix string, pct float64, width int) string {
	if width <= 0 {
		width = 20
	}
	val := pct
	if val < 0 {
		val = 0
	}
	if val > 100 {
		val = 100
	}
	filled := int(math.Round(val * float64(width) / 100.0))
	if filled < 0 {
		filled = 0
	}
	if filled > width {
		filled = width
	}
	bar := fmt.Sprintf("[%s]%s[-]%s", colorByPercent(val), strings.Repeat("#", filled), strings.Repeat("-", width-filled))
	return fmt.Sprintf("[white::b]%s:[-] %5.1f%% %s", prefix, pct, bar)
}

type sortMode int

const (
	sortByCPU sortMode = iota
	sortByMem
	sortByPID
	sortByName
)

func New(collector *metrics.Collector) *UI {
	app := tview.NewApplication()

	title := tview.NewTextView().
		SetDynamicColors(true).
		SetTextAlign(tview.AlignLeft).
		SetText("[white::b]htop-win-go")

	cpuBar := tview.NewTextView().
		SetDynamicColors(true).
		SetTextAlign(tview.AlignLeft)

	memBar := tview.NewTextView().
		SetDynamicColors(true).
		SetTextAlign(tview.AlignLeft)

	procs := tview.NewTextView().
		SetDynamicColors(true).
		SetTextAlign(tview.AlignLeft)

	gaugeRow := tview.NewFlex().
		SetDirection(tview.FlexColumn).
		AddItem(cpuBar, 0, 1, false).
		AddItem(memBar, 0, 1, false).
		AddItem(procs, 14, 0, false)

	header := tview.NewFlex().
		SetDirection(tview.FlexRow).
		AddItem(title, 1, 0, false).
		AddItem(gaugeRow, 1, 0, false)

	table := tview.NewTable().
		SetBorders(false).
		SetSelectable(true, false)

	footer := tview.NewTextView().
		SetDynamicColors(true).
		SetTextAlign(tview.AlignLeft)

	layout := tview.NewFlex().
		SetDirection(tview.FlexRow).
		AddItem(header, 2, 0, false).
		AddItem(table, 0, 1, true).
		AddItem(footer, 1, 0, false)

	ui := &UI{
		app:       app,
		header:    header,
		title:     title,
		cpuBar:    cpuBar,
		memBar:    memBar,
		procs:     procs,
		table:     table,
		footer:    footer,
		collector: collector,
		sort:      sortByCPU,
	}

	app.SetRoot(layout, true)
	ui.bindKeys()
	ui.initTableHeader()

	return ui
}

func (u *UI) bindKeys() {
	u.app.SetInputCapture(func(event *tcell.EventKey) *tcell.EventKey {
		switch event.Key() {
		case tcell.KeyEscape:
			if u.cancel != nil {
				u.cancel()
			}
			u.app.Stop()
			return nil
		}
		switch strings.ToLower(string(event.Rune())) {
		case "q":
			if u.cancel != nil {
				u.cancel()
			}
			u.app.Stop()
			return nil
		case "c":
			u.setSort(sortByCPU)
			return nil
		case "m":
			u.setSort(sortByMem)
			return nil
		case "p":
			u.setSort(sortByPID)
			return nil
		case "n":
			u.setSort(sortByName)
			return nil
		}
		return event
	})
}

func (u *UI) setSort(mode sortMode) {
	if u.sort == mode {
		return
	}
	u.sort = mode
	go u.refresh()
}

func (u *UI) initTableHeader() {
	headers := []string{"PID", "Name", "CPU%", "Mem RSS", "Mem%", "Start"}
	for i, h := range headers {
		cell := tview.NewTableCell(fmt.Sprintf("[white::b]%s", h)).
			SetSelectable(false)
		u.table.SetCell(0, i, cell)
	}
}

func (u *UI) Run() error {
	u.ctx, u.cancel = context.WithCancel(context.Background())
	u.footer.SetText(u.footerMessage(nil))

	go u.loop()
	return u.app.Run()
}

func (u *UI) loop() {
	ticker := time.NewTicker(1 * time.Second)
	defer ticker.Stop()

	// 立即刷新一次
	u.refresh()

	for {
		select {
		case <-u.ctx.Done():
			return
		case <-ticker.C:
			u.refresh()
		}
	}
}

func (u *UI) refresh() {
	ctx, cancel := context.WithTimeout(u.ctx, 1500*time.Millisecond)
	defer cancel()

	st, err1 := metrics.GetSystemStats(ctx)
	procs, err2 := u.collector.Snapshot(ctx)

	u.app.QueueUpdateDraw(func() {
		if err1 == nil {
			u.cpuBar.SetText(renderBar("CPU", st.CPUPercent, 24))
			u.memBar.SetText(fmt.Sprintf("%s  %s/%s (%.1f%%)", renderBar("Mem", st.MemUsedPercent, 24), bytesHuman(st.MemUsed), bytesHuman(st.MemTotal), st.MemUsedPercent))
			u.procs.SetText(fmt.Sprintf("[green::b]Procs:[-] %d", len(procs)))
			u.title.SetText("[white::b]htop-win-go")
		} else {
			u.title.SetText(fmt.Sprintf("[red]系统指标错误: %v", err1))
			u.cpuBar.SetText("CPU: -")
			u.memBar.SetText("Mem: -")
			u.procs.SetText("")
		}

		// table rows
		// 保持第一行表头
		for r := u.table.GetRowCount() - 1; r >= 1; r-- {
			u.table.RemoveRow(r)
		}

		rows := append([]metrics.ProcessInfo(nil), procs...)
		u.sortProcesses(rows)

		maxRows := 100
		if len(rows) < maxRows {
			maxRows = len(rows)
		}
		for i := 0; i < maxRows; i++ {
			p := rows[i]
			u.table.SetCell(i+1, 0, tview.NewTableCell(fmt.Sprintf("%d", p.PID)))
			u.table.SetCell(i+1, 1, tview.NewTableCell(truncate(p.Name, 40)))
			u.table.SetCell(i+1, 2, tview.NewTableCell(fmt.Sprintf("%.1f", p.CPUPercent)))
			u.table.SetCell(i+1, 3, tview.NewTableCell(bytesHuman(p.MemRSS)))
			u.table.SetCell(i+1, 4, tview.NewTableCell(fmt.Sprintf("%.1f", p.MemPercent)))
			u.table.SetCell(i+1, 5, tview.NewTableCell(fmtTime(p.CreateTime)))
		}

		u.footer.SetText(u.footerMessage(err2))
	})
}

func (u *UI) footerMessage(err error) string {
	msg := fmt.Sprintf("[yellow]q/Esc 退出  •  刷新:1s  •  排序:%s (C/M/P/N)  •  Go:%s", u.sort.label(), runtime.Version())
	if err != nil {
		msg += fmt.Sprintf("  [yellow]进程数据可能不完整: %v", err)
	}
	return msg
}

func (u *UI) sortProcesses(list []metrics.ProcessInfo) {
	switch u.sort {
	case sortByMem:
		sort.SliceStable(list, func(i, j int) bool {
			if list[i].MemPercent == list[j].MemPercent {
				return list[i].CPUPercent > list[j].CPUPercent
			}
			return list[i].MemPercent > list[j].MemPercent
		})
	case sortByPID:
		sort.SliceStable(list, func(i, j int) bool {
			if list[i].PID == list[j].PID {
				return list[i].CPUPercent > list[j].CPUPercent
			}
			return list[i].PID < list[j].PID
		})
	case sortByName:
		sort.SliceStable(list, func(i, j int) bool {
			li := strings.ToLower(list[i].Name)
			lj := strings.ToLower(list[j].Name)
			if li == lj {
				return list[i].CPUPercent > list[j].CPUPercent
			}
			return li < lj
		})
	default:
		sort.SliceStable(list, func(i, j int) bool {
			if list[i].CPUPercent == list[j].CPUPercent {
				return list[i].PID < list[j].PID
			}
			return list[i].CPUPercent > list[j].CPUPercent
		})
	}
}

func (s sortMode) label() string {
	switch s {
	case sortByMem:
		return "Mem%"
	case sortByPID:
		return "PID"
	case sortByName:
		return "Name"
	default:
		return "CPU%"
	}
}

func bytesHuman(n uint64) string {
	const (
		KB = 1024
		MB = 1024 * KB
		GB = 1024 * MB
	)
	switch {
	case n >= GB:
		return fmt.Sprintf("%.1f GB", float64(n)/float64(GB))
	case n >= MB:
		return fmt.Sprintf("%.1f MB", float64(n)/float64(MB))
	case n >= KB:
		return fmt.Sprintf("%.1f KB", float64(n)/float64(KB))
	default:
		return fmt.Sprintf("%d B", n)
	}
}

func truncate(s string, max int) string {
	if len(s) <= max {
		return s
	}
	return s[:max-1] + "…"
}

func fmtTime(ms int64) string {
	if ms <= 0 {
		return "-"
	}
	t := time.Unix(0, ms*int64(time.Millisecond))
	return t.Format("01-02 15:04")
}
