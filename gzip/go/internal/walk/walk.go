// Package walk provides file collection utilities.
package walk

import (
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

// CollectInputs collects files from paths, optionally recursively.
// For decompress mode, only .gz files are collected.
// For compress mode, non-.gz files are collected.
func CollectInputs(paths []string, recursive bool, decompress bool) ([]string, error) {
	var out []string
	for _, p := range paths {
		info, err := os.Stat(p)
		if err != nil {
			return nil, err
		}
		if info.IsDir() {
			if !recursive {
				// Skip directories when not recursive
				continue
			}
			err := filepath.WalkDir(p, func(path string, d fs.DirEntry, err error) error {
				if err != nil {
					return nil // skip inaccessible paths
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
						return nil // skip already compressed
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

		// Regular file
		if decompress {
			if !strings.HasSuffix(info.Name(), ".gz") {
				continue // skip non-.gz files
			}
			out = append(out, p)
		} else {
			if strings.HasSuffix(info.Name(), ".gz") {
				continue // skip already compressed
			}
			out = append(out, p)
		}
	}
	return out, nil
}
