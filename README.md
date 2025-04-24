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

- [ ] TOML layout configuration
- [ ] Custom CSS + integration with Base16 themes
- [ ] Run a daemon in the background for fast startup + an option to stop it
- [ ] In app launcher mode, store the choice counts for every option. Use it to rank them
- [ ] Integrate other toml configurations to be used as a choice menu for anything
