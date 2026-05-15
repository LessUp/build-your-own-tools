package main

import (
	"compress/gzip"
	"flag"
	"fmt"
	"os"
	"runtime"
	"strings"
	"sync"

	gziplib "gzip-go/internal/gzip"
	"gzip-go/internal/walk"
)

type options struct {
	decompress bool
	recursive  bool
	stdout     bool
	force      bool
	keep       bool
	level      int
	workers    int
}

func main() {
	opts := parseFlags()

	args := flag.Args()
	if len(args) == 0 {
		// Default to streaming: stdin -> stdout
		opts.stdout = true
		args = []string{"-"}
	}

	// When using stdout, only one input is allowed
	if opts.stdout && len(args) != 1 {
		fmt.Fprintln(os.Stderr, "当指定 -stdout 时，只能处理一个输入（文件或 -）")
		os.Exit(2)
	}

	// Stream processing (- means stdin)
	if len(args) == 1 && args[0] == "-" {
		if opts.decompress {
			if err := gziplib.GunzipStream(os.Stdin, os.Stdout); err != nil {
				fmt.Fprintln(os.Stderr, "解压失败:", err)
				os.Exit(1)
			}
		} else {
			if err := gziplib.GzipStream(os.Stdin, os.Stdout, opts.level, "stdin"); err != nil {
				fmt.Fprintln(os.Stderr, "压缩失败:", err)
				os.Exit(1)
			}
		}
		return
	}

	// Single file to stdout (non-streaming)
	if opts.stdout {
		path := args[0]
		if opts.decompress {
			if !strings.HasSuffix(path, ".gz") {
				fmt.Fprintf(os.Stderr, "解压到标准输出时，输入应为 .gz 文件: %s\n", path)
				os.Exit(2)
			}
			if err := gziplib.GunzipToWriter(path, os.Stdout); err != nil {
				fmt.Fprintln(os.Stderr, "解压失败:", err)
				os.Exit(1)
			}
		} else {
			if strings.HasSuffix(path, ".gz") {
				fmt.Fprintf(os.Stderr, "已是 .gz 文件，跳过: %s\n", path)
				os.Exit(2)
			}
			if err := gziplib.GzipToWriter(path, os.Stdout, opts.level); err != nil {
				fmt.Fprintln(os.Stderr, "压缩失败:", err)
				os.Exit(1)
			}
		}
		return
	}

	// Build input file list (expand directories)
	inputs, err := walk.CollectInputs(args, opts.recursive, opts.decompress)
	if err != nil {
		fmt.Fprintln(os.Stderr, "收集输入失败:", err)
		os.Exit(1)
	}
	if len(inputs) == 0 {
		fmt.Fprintln(os.Stderr, "没有可处理的文件")
		return
	}

	if opts.workers < 1 {
		opts.workers = 1
	}

	errCh := make(chan error, len(inputs))
	var wg sync.WaitGroup
	sem := make(chan struct{}, opts.workers)

	for _, src := range inputs {
		sem <- struct{}{}
		wg.Add(1)
		go func(src string) {
			defer wg.Done()
			defer func() { <-sem }()

			if opts.decompress {
				dest := strings.TrimSuffix(src, ".gz")
				if dest == src {
					errCh <- fmt.Errorf("无法推断解压目标文件名: %s", src)
					return
				}
				if !opts.force {
					if _, err := os.Stat(dest); err == nil {
						errCh <- fmt.Errorf("目标已存在（使用 -f 覆盖）: %s", dest)
						return
					}
				}
				if err := gziplib.GunzipFile(src, dest); err != nil {
					errCh <- fmt.Errorf("解压失败 %s -> %s: %w", src, dest, err)
					return
				}
				if !opts.keep {
					if err := os.Remove(src); err != nil {
						fmt.Fprintf(os.Stderr, "警告: 删除源文件失败 %s: %v\n", src, err)
					}
				}
				fmt.Fprintf(os.Stderr, "完成: %s -> %s\n", src, dest)
			} else {
				dest := src + ".gz"
				if !opts.force {
					if _, err := os.Stat(dest); err == nil {
						errCh <- fmt.Errorf("目标已存在（使用 -f 覆盖）: %s", dest)
						return
					}
				}
				if err := gziplib.GzipFile(src, dest, opts.level); err != nil {
					errCh <- fmt.Errorf("压缩失败 %s -> %s: %w", src, dest, err)
					return
				}
				if !opts.keep {
					if err := os.Remove(src); err != nil {
						fmt.Fprintf(os.Stderr, "警告: 删除源文件失败 %s: %v\n", src, err)
					}
				}
				fmt.Fprintf(os.Stderr, "完成: %s -> %s\n", src, dest)
			}
		}(src)
	}

	wg.Wait()
	close(errCh)

	hadErr := false
	for e := range errCh {
		if e != nil {
			fmt.Fprintln(os.Stderr, "错误:", e)
			hadErr = true
		}
	}
	if hadErr {
		os.Exit(1)
	}
}

func parseFlags() options {
	var opts options
	flag.BoolVar(&opts.decompress, "d", false, "解压模式 (gunzip)")
	flag.BoolVar(&opts.recursive, "r", false, "递归处理目录")
	flag.BoolVar(&opts.stdout, "stdout", false, "输出到标准输出")
	flag.BoolVar(&opts.force, "f", false, "覆盖已存在的目标文件")
	flag.BoolVar(&opts.keep, "k", false, "保留源文件（不删除）")
	flag.IntVar(&opts.level, "l", gzip.DefaultCompression, "压缩级别 0-9 (默认-1)")
	flag.IntVar(&opts.workers, "p", runtime.NumCPU(), "并行 worker 数量")
	flag.Parse()
	return opts
}
