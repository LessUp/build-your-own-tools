package main

import (
	"bytes"
	"compress/gzip"
	"io"
	"os"
	"path/filepath"
	"testing"
)

func TestGzipStream(t *testing.T) {
	input := []byte("hello gzip stream test")
	var buf bytes.Buffer

	if err := gzipStream(bytes.NewReader(input), &buf, gzip.DefaultCompression, "test"); err != nil {
		t.Fatalf("gzipStream failed: %v", err)
	}
	if buf.Len() == 0 {
		t.Fatal("gzipStream produced empty output")
	}

	// decompress and verify
	var out bytes.Buffer
	if err := gunzipStream(&buf, &out); err != nil {
		t.Fatalf("gunzipStream failed: %v", err)
	}
	if !bytes.Equal(out.Bytes(), input) {
		t.Fatalf("roundtrip mismatch: got %q, want %q", out.Bytes(), input)
	}
}

func TestGzipFile(t *testing.T) {
	dir := t.TempDir()
	src := filepath.Join(dir, "test.txt")
	dest := filepath.Join(dir, "test.txt.gz")
	data := []byte("file compression test data\n")

	if err := os.WriteFile(src, data, 0644); err != nil {
		t.Fatal(err)
	}

	if err := gzipFile(src, dest, gzip.DefaultCompression); err != nil {
		t.Fatalf("gzipFile failed: %v", err)
	}

	// verify compressed file exists
	if _, err := os.Stat(dest); err != nil {
		t.Fatalf("compressed file not found: %v", err)
	}
}

func TestGunzipFile(t *testing.T) {
	dir := t.TempDir()
	src := filepath.Join(dir, "test.txt")
	gz := filepath.Join(dir, "test.txt.gz")
	out := filepath.Join(dir, "test_out.txt")
	data := []byte("decompression roundtrip test\n")

	if err := os.WriteFile(src, data, 0644); err != nil {
		t.Fatal(err)
	}

	if err := gzipFile(src, gz, gzip.DefaultCompression); err != nil {
		t.Fatal(err)
	}

	if err := gunzipFile(gz, out); err != nil {
		t.Fatalf("gunzipFile failed: %v", err)
	}

	result, err := os.ReadFile(out)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(result, data) {
		t.Fatalf("roundtrip mismatch: got %q, want %q", result, data)
	}
}

func TestCollectInputs_SkipDir(t *testing.T) {
	dir := t.TempDir()

	// without recursive, directories should be skipped
	inputs, err := collectInputs([]string{dir}, false, false)
	if err != nil {
		t.Fatalf("collectInputs failed: %v", err)
	}
	if len(inputs) != 0 {
		t.Fatalf("expected 0 inputs for non-recursive dir, got %d", len(inputs))
	}
}

func TestCollectInputs_SkipGz(t *testing.T) {
	dir := t.TempDir()
	gz := filepath.Join(dir, "test.gz")
	if err := os.WriteFile(gz, []byte("fake"), 0644); err != nil {
		t.Fatal(err)
	}

	// compress mode should skip .gz files
	inputs, err := collectInputs([]string{gz}, false, false)
	if err != nil {
		t.Fatalf("collectInputs failed: %v", err)
	}
	if len(inputs) != 0 {
		t.Fatalf("expected .gz to be skipped in compress mode, got %d", len(inputs))
	}
}

func TestGzipToWriter(t *testing.T) {
	dir := t.TempDir()
	src := filepath.Join(dir, "input.txt")
	data := []byte("gzipToWriter test")
	if err := os.WriteFile(src, data, 0644); err != nil {
		t.Fatal(err)
	}

	var buf bytes.Buffer
	if err := gzipToWriter(src, &buf, gzip.DefaultCompression); err != nil {
		t.Fatalf("gzipToWriter failed: %v", err)
	}

	// decompress
	r, err := gzip.NewReader(&buf)
	if err != nil {
		t.Fatal(err)
	}
	defer r.Close()
	result, err := io.ReadAll(r)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(result, data) {
		t.Fatalf("roundtrip mismatch: got %q, want %q", result, data)
	}
}
