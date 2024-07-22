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
