import { invoke } from "@tauri-apps/api/core";
import type { Entry } from "./config";

export const runCommand = (entry: Entry) => {
  invoke("run_command", {
    command: entry.command,
    terminal: entry.terminal,
  });
};
