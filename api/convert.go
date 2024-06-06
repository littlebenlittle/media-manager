package main

// import (
// 	"bufio"
// 	"bytes"
// 	"errors"
// 	"fmt"
// 	"log"
// 	"os"
// 	"os/exec"
// 	"path/filepath"
// )

// type ConvertOptions struct {
// 	MediaID   string
// 	Format    string
// 	Hardsub   bool
// 	Overwrite bool
// }

// func convert(
// 	path string,
// 	format string,
// 	hardsub bool,
// 	overwrite bool,
// ) (logs chan LogEntry, err_ch chan error, err error) {
// 	path, err = filepath.EvalSymlinks(path)
// 	if err != nil {
// 		return
// 	}
// 	root := "/dir"
// 	if filepath.Dir(path) != root {
// 		return nil, nil, fmt.Errorf("file is not in root `%s`: %s", root, path)
// 	}
// 	in_basename := filepath.Base(path)
// 	out_basename := in_basename[:len(in_basename)-len(filepath.Ext(path))] + "." + format
// 	out := filepath.Join(root, out_basename)
// 	_, err = os.Stat(out)
// 	if !os.IsNotExist(err) && !overwrite {
// 		return nil, nil, errors.New("file exists and `overwrite` set to `false`")
// 	}
// 	log.Printf("%s", out)
// 	var cmd *exec.Cmd
// 	if hardsub {
// 		cmd = exec.Command(
// 			"ffmpeg",
// 			"-y",
// 			"-i", path,
// 			"-c:a", "libopus",
// 			"-vf", fmt.Sprintf("subtitles='%s'", path),
// 			out,
// 		)
// 	} else {
// 		cmd = exec.Command(
// 			"ffmpeg",
// 			"-y",
// 			"-i", path,
// 			"-c:a", "libopus",
// 			out,
// 		)
// 	}
// 	cmd.Stdout = scan_logs(logs)
// 	cmd.Stderr = scan_logs(logs)
// 	err_ch = make(chan error)
// 	go func() { err_ch <- cmd.Run() }()
// 	return
// }

// func scan_logs(logs chan LogEntry) *bytes.Buffer {
// 	buf := bytes.Buffer{}
// 	go func() {
// 		scanner := bufio.NewScanner(&buf)
// 		for scanner.Scan() {
// 			logs <- LogEntry{
// 				message: scanner.Text(),
// 			}
// 		}
// 	}()
// 	return &buf
// }
