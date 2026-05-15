// Package gzip provides pure compression and decompression functions.
package gzip

import (
	"compress/gzip"
	"io"
	"os"
	"path/filepath"
	"time"
)

// GzipFile compresses a file to a destination path with the given compression level.
func GzipFile(src, dest string, level int) error {
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
	return w.Close()
}

// GunzipFile decompresses a gzip file to a destination path.
func GunzipFile(src, dest string) error {
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

	// Try to restore modification time
	if !r.ModTime.IsZero() {
		_ = os.Chtimes(dest, time.Now(), r.ModTime)
	}
	return nil
}

// GzipStream compresses from a reader to a writer.
func GzipStream(in io.Reader, out io.Writer, level int, name string) error {
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

// GunzipStream decompresses from a reader to a writer.
func GunzipStream(in io.Reader, out io.Writer) error {
	r, err := gzip.NewReader(in)
	if err != nil {
		return err
	}
	defer r.Close()
	_, err = io.Copy(out, r)
	return err
}

// GunzipToWriter opens a gzip file and decompresses to a writer.
func GunzipToWriter(src string, out io.Writer) error {
	f, err := os.Open(src)
	if err != nil {
		return err
	}
	defer f.Close()
	return GunzipStream(f, out)
}

// GzipToWriter opens a file and compresses to a writer.
func GzipToWriter(src string, out io.Writer, level int) error {
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
