# Tauri + Leptos

This template should help get you started developing with Tauri and Leptos.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Build APK (Android)

Use the following command to build an APK:

```bash
env -u NO_COLOR cargo tauri android build --debug --apk --target aarch64
```

If `NO_COLOR=1` is set in your environment, `trunk build` may fail, so run with `env -u NO_COLOR` as shown above.

## Architecture

- [Frontend/Backend 責務分担ガイド](docs/frontend-backend-boundary.md)
