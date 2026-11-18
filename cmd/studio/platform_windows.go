package main

import (
	"os/exec"
	"path/filepath"
	"syscall"
)

func policyCommand() string {
	syscall.NewLazyDLL("kernel32.dll").NewProc("SetConsoleOutputCP").Call(65001)
	syscall.NewLazyDLL("kernel32.dll").NewProc("SetConsoleCP").Call(65001)
	path, err := exec.LookPath("ucm.cmd")
	if err != nil {
		return "ucm"
	}
	return filepath.Join(filepath.Dir(path), "unison", "unison.exe")
}
