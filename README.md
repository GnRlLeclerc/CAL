# CAL (Configurable App Launcher)

A configurable app launcher for Linux, inspired by [Rofi](https://github.com/davatorium/rofi).

Built with [Tauri](https://v2.tauri.app) and [Svelte](https://svelte.dev/).

Run the app in dev mode:

```bash
bun tauri dev
```

Build the app:

```bash
bun tauri build
```

## Todolist

- [ ] In app launcher mode, store the choice counts for every option. Use it to rank them (store as toml, shouldn't be too hard.)
      Should be rewritten each time. Maybe store in json, then ? Do it from the frontend, with proper permissions.
