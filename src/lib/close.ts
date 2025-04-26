/** Closing window logic */

import { getCurrentWindow } from "@tauri-apps/api/window";
import { appState } from "./config.svelte";

export const handleClose = () => {
  if (appState.config?.daemon) {
    getCurrentWindow().hide();
  } else {
    getCurrentWindow().close();
  }
};
