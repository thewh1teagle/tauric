# tauric

[![PyPi Version](https://img.shields.io/pypi/v/tauripy?color=36719F&logo=python)](https://pypi.org/project/tauripy)

C-API for [tauri](https://tauri.app) written in Rust.

Potential: Use tauri from `Python`, `Go`, `TypeScript` etc..

## Features

- 🔗 Easy integration with C/C++ (or any other language) via C API
- 🐍 Python support
- 📦 Precompiled library and C header files available in the [releases](https://github.com/thewh1teagle/tauric/releases/latest)
- 🖥️ Support for Windows (x86-64), Linux (x86-64), macOS (x86/arm64)

# Install

Python 🐍

```console
pip install tauripy
```

## Examples

```python
from tauripy import Tauri
from pathlib import Path
import json

def on_command(message: bytes):
    print(f"Received: {message.decode('utf-8')}")
    return json.dumps({'message': 'Hello from Python!'}).encode('utf-8')

tauric = Tauri("com.tauric.dev", "tauric")
tauric.mount_frontend('./dist') # you should create dist folder with index.html
tauric.on_command(on_command)
tauric.run(lambda: tauric.create_window("example_window", "tauric", "fs://index.html"))
```

See also [examples](examples)

## Roadmap

See [Roadmap](https://github.com/thewh1teagle/tauric/issues/2)
