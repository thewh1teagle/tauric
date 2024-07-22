package main

import (
	"fmt"
	"runtime"

	"github.com/ebitengine/purego"
)

func getLibrary() string {
	switch runtime.GOOS {
	case "darwin":
		return "libtauric.dylib"
	case "linux":
		return "libtauric.so"
	case "windows":
		return "libtauric.dll"
	default:
		panic(fmt.Errorf("GOOS=%s is not supported", runtime.GOOS))
	}
}

func main() {
	lib, err := purego.Dlopen(getLibrary(), purego.RTLD_NOW|purego.RTLD_GLOBAL)
	if err != nil {
		panic(err)
	}

	onReadycb := purego.NewCallback(func() {
		var createWindow func(label string, url string)
		purego.RegisterLibFunc(&createWindow, lib, "create_window")
		createWindow("main", "https://example.org")
	})

	var registerReadyCallback func(p uintptr)
	purego.RegisterLibFunc(&registerReadyCallback, lib, "on_ready")
	registerReadyCallback(onReadycb)

	// Define and register the run function
	var run func()
	purego.RegisterLibFunc(&run, lib, "run")
	run()
}
