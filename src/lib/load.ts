/** Load the launcher configuration from a channel */

import { Channel, invoke } from "@tauri-apps/api/core";
import { type Config } from "./config";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import { appState } from "./config.svelte";
import removeAccents from "remove-accents";

/** Subscribe to the configuration channel */
export const subscribeConfig = async () => {
  const channel = new Channel<Config>();

  channel.onmessage = (config) => {
    // Compute the keywords
    for (const entry of config.entries) {
      entry.allKeywords = removeAccents(entry.name.toLowerCase()).split(" ");
      if (entry.keywords) {
        entry.allKeywords.push(...entry.keywords.map((k) => k.toLowerCase()));
      }
    }

    if (config.mode === "icon") {
      // Resize the window to go into row mode
      getCurrentWindow().setSize(new LogicalSize(600, 200));
    } else {
      getCurrentWindow().setSize(new LogicalSize(600, 400));
    }

    // Update the config
    appState.config = config;

    // Show the window
    getCurrentWindow().show();
  };

  await invoke("subscribe_config", { channel });
};
