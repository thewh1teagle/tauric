# Building

### Prerequisites

[Bun](https://bun.sh/) | [Cargo](https://www.rust-lang.org/tools/install)

## Build

1. Install dependencies

```console
bun install
```

2. Build icons

```console
bunx tauri icon assets/logo.png
```

Build the library by execute

```console
cargo build
```

## Create shared library with headers

Install
```console
brew install cbindgen
```

Then execute
```console
cbindgen > tauric.h
```

## Bundle python app

```console
pip install -U pyinstaller
pyinstaller --noconsole --onefile --add-binary "venv/Lib/site-packages/tauripy/tauric.dll;." --add-data "dist;dist" --distpath "./build" main.py
```