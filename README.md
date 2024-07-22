# tauric

C-API for [tauri](https://tauri.app) written in Rust.

Potential: Use tauri from `Python`, `Go`, `TypeScript` etc..

## Usage

See [BUILDING.md](BUILDING.md)
After building it, just run

```console
python bindings/python/main.py
```

Then you can paste in the console

```js
invoke("command", { args: { hello: "world" } });
```

## Todo

Goal: No need for Cargo / Rust!

- [ ] Dynamic icon from Python
- [ ] Dynamic app name from Python
- [ ] Dynamic identifier from Python
- [ ] Dev server with hot reload
- [ ] Bundle including frontend with Go into a single executable
- [ ] Better IPC
- [ ] Dynamic capabilities
- [ ] PyPI package
- [ ] Go package
- [ ] NPM package
- [ ] CI to build cross platform packages with static / dynamic lib
- [ ] Tauri plugins from shared libraries
- [ ] Load local files using custom protocl **ONLY** after bindings enabled it.
