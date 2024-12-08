//go:build darwin || freebsd || linux || windows

package main

import (
	"fmt"
	"os"
	"path/filepath"
	"runtime"

	"github.com/ebitengine/purego"
)

func getExecutablePath() (string, error) {
	ex, err := os.Executable()
	if err != nil {
		return "", err
	}
	return filepath.Dir(ex), nil
}

func getLibrary() string {
	switch runtime.GOOS {
	case "darwin":
		return "libtauric.dylib"
	case "linux":
		return "libtauric.so"
	case "windows":
		return "tauric.dll"
	default:
		panic(fmt.Errorf("GOOS=%s is not supported", runtime.GOOS))
	}
}

func main() {
	lib, err := openLibrary(getLibrary())
	if err != nil {
		panic(err)
	}

	onReadycb := purego.NewCallback(func() {
		var createWindow func(label string, url string)
		purego.RegisterLibFunc(&createWindow, lib, "create_window")
		createWindow("main", "local://index.html")
	})
	var registerReadyCallback func(p uintptr)
	purego.RegisterLibFunc(&registerReadyCallback, lib, "on_ready")
	registerReadyCallback(onReadycb)

	var mountFrontend func(path string)
	purego.RegisterLibFunc(&mountFrontend, lib, "mount_frontend")

	// Define and register the run function
	var run func()
	purego.RegisterLibFunc(&run, lib, "run")

	executableDir, err := getExecutablePath()
	if err != nil {
		panic(err)
	}

	// Construct the path to the dist folder
	distPath := filepath.Join(executableDir, "dist")

	// Main
	mountFrontend(distPath)
	run()
}
