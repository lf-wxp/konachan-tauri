## Description
Konachan tauri version, use tauri as backend;


## Screenshot
![screenshot](./screenshot.gif)

## Installation

```bash
pnpm i

git submodule init

git submodule update --recursive
```
### install tauri
```bash
cargo install tauri-cli
```


### frontend
the frontend link to [konachan-front](https://github.com/lf-wxp/konachan-front) as a submodule.

### backend
reference the [tauri document](https://tauri.studio/docs/getting-started/setting-up-macos) to install tauri.

## Run the app

```bash
# development
$ cargo tauri dev

# production mode
$ cargo tauri build
```
