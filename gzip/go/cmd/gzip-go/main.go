package main

import (
	"compress/gzip"
	"flag"
	"fmt"
	"io"
	"io/fs"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"sync"
	"time"
)

type options struct {
	decompress bool
	recursive  bool
	stdout    bool
	force     bool
	level     int
	workers   int
}

func main() {
	opts := parseFlags()

	args := flag.Args()
	if len(args) == 0 {
		// 无参时默认走流式：stdin -> stdout
		opts.stdout = true
		args = []string{"-"}
	}

	// 标准输出模式时，仅允许单一输入（文件或-）
	if opts.stdout && len(args) != 1 {
		fmt.Fprintln(os.Stderr, "当指定 -stdout 时，只能处理一个输入（文件或 -）")
		os.Exit(2)
	}

	// 流式处理（- 表示 stdin）
	if len(args) == 1 && args[0] == "-" {
		if opts.decompress {
			if err := gunzipStream(os.Stdin, os.Stdout); err != nil {
				fmt.Fprintln(os.Stderr, "解压失败:", err)
				os.Exit(1)
			}
		} else {
			if err := gzipStream(os.Stdin, os.Stdout, opts.level, "stdin"); err != nil {
				fmt.Fprintln(os.Stderr, "压缩失败:", err)
				os.Exit(1)
			}
		}
		return
	}

	// 单文件到 stdout（非流式）
	if opts.stdout {
		path := args[0]
		if opts.decompress {
			if !strings.HasSuffix(path, ".gz") {
				fmt.Fprintf(os.Stderr, "解压到标准输出时，输入应为 .gz 文件: %s\n", path)
				os.Exit(2)
			}
			if err := gunzipToWriter(path, os.Stdout); err != nil {
				fmt.Fprintln(os.Stderr, "解压失败:", err)
				os.Exit(1)
			}
		} else {
			if strings.HasSuffix(path, ".gz") {
				fmt.Fprintf(os.Stderr, "已是 .gz 文件，跳过: %s\n", path)
				os.Exit(2)
			}
			if err := gzipToWriter(path, os.Stdout, opts.level); err != nil {
				fmt.Fprintln(os.Stderr, "压缩失败:", err)
				os.Exit(1)
			}
		}
		return
	}

	// 构建输入文件列表（展开目录）
	inputs, err := collectInputs(args, opts.recursive, opts.decompress)
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
		go func() {
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
				if err := gunzipFile(src, dest); err != nil {
					errCh <- fmt.Errorf("解压失败 %s -> %s: %w", src, dest, err)
					return
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
				if err := gzipFile(src, dest, opts.level); err != nil {
					errCh <- fmt.Errorf("压缩失败 %s -> %s: %w", src, dest, err)
					return
				}
				fmt.Fprintf(os.Stderr, "完成: %s -> %s\n", src, dest)
			}
		}()
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
	flag.IntVar(&opts.level, "l", gzip.DefaultCompression, "压缩级别 0-9 (默认-1)")
	flag.IntVar(&opts.workers, "p", runtime.NumCPU(), "并行 worker 数量")
	flag.Parse()
	return opts
}

func collectInputs(paths []string, recursive bool, decompress bool) ([]string, error) {
	var out []string
	for _, p := range paths {
		info, err := os.Stat(p)
		if err != nil {
			return nil, err
		}
		if info.IsDir() {
			if !recursive {
				fmt.Fprintf(os.Stderr, "跳过目录（未启用 -r）: %s\n", p)
				continue
			}
			err := filepath.WalkDir(p, func(path string, d fs.DirEntry, err error) error {
				if err != nil {
					fmt.Fprintf(os.Stderr, "访问失败: %s: %v\n", path, err)
					return nil
				}
				if d.IsDir() {
					return nil
				}
				if decompress {
					if strings.HasSuffix(d.Name(), ".gz") {
						out = append(out, path)
					}
				} else {
					if strings.HasSuffix(d.Name(), ".gz") {
						// 避免重复压缩 .gz
						return nil
					}
					out = append(out, path)
				}
				return nil
			})
			if err != nil {
				return nil, err
			}
			continue
		}

		// 普通文件
		if decompress {
			if !strings.HasSuffix(info.Name(), ".gz") {
				fmt.Fprintf(os.Stderr, "跳过非 .gz 文件: %s\n", p)
				continue
			}
			out = append(out, p)
		} else {
			if strings.HasSuffix(info.Name(), ".gz") {
				fmt.Fprintf(os.Stderr, "跳过已为 .gz 的文件: %s\n", p)
				continue
			}
			out = append(out, p)
		}
	}
	return out, nil
}

func gzipFile(src, dest string, level int) error {
	inf, err := os.Open(src)
	if err != nil {
		return err
	}
	defer inf.Close()

	fi, err := inf.Stat()
	if err != nil {
		return err
	}

	out, err := os.OpenFile(dest, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0644)
	if err != nil {
		return err
	}
	defer out.Close()

	w, err := gzip.NewWriterLevel(out, level)
	if err != nil {
		return err
	}
	w.Name = filepath.Base(src)
	w.ModTime = fi.ModTime()

	if _, err := io.Copy(w, inf); err != nil {
		_ = w.Close()
		return err
	}
	if err := w.Close(); err != nil {
		return err
	}
	return nil
}

func gunzipFile(src, dest string) error {
	inf, err := os.Open(src)
	if err != nil {
		return err
	}
	defer inf.Close()

	r, err := gzip.NewReader(inf)
	if err != nil {
		return err
	}
	defer r.Close()

	out, err := os.OpenFile(dest, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0644)
	if err != nil {
		return err
	}
	defer out.Close()

	if _, err := io.Copy(out, r); err != nil {
		return err
	}

	// 尝试恢复修改时间
	if !r.ModTime.IsZero() {
		_ = os.Chtimes(dest, time.Now(), r.ModTime)
	}
	return nil
}

func gzipStream(in io.Reader, out io.Writer, level int, name string) error {
	w, err := gzip.NewWriterLevel(out, level)
	if err != nil {
		return err
	}
	if name != "" && name != "-" {
		w.Name = filepath.Base(name)
	}
	w.ModTime = time.Now()
	if _, err := io.Copy(w, in); err != nil {
		_ = w.Close()
		return err
	}
	return w.Close()
}

func gunzipStream(in io.Reader, out io.Writer) error {
	r, err := gzip.NewReader(in)
	if err != nil {
		return err
	}
	defer r.Close()
	_, err = io.Copy(out, r)
	return err
}

func gunzipToWriter(src string, out io.Writer) error {
	f, err := os.Open(src)
	if err != nil {
		return err
	}
	defer f.Close()
	return gunzipStream(f, out)
}

func gzipToWriter(src string, out io.Writer, level int) error {
	f, err := os.Open(src)
	if err != nil {
		return err
	}
	defer f.Close()

	fi, err := f.Stat()
	if err != nil {
		return err
	}
	w, err := gzip.NewWriterLevel(out, level)
	if err != nil {
		return err
	}
	w.Name = filepath.Base(src)
	w.ModTime = fi.ModTime()
	if _, err := io.Copy(w, f); err != nil {
		_ = w.Close()
		return err
	}
	return w.Close()
}
