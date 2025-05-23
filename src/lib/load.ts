/** Load the launcher configuration from a channel */

import { Channel, invoke } from "@tauri-apps/api/core";
import { updateColors, type Config } from "./config";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import { appState } from "./config.svelte";
import removeAccents from "remove-accents";
import { getCounts } from "./counts";

/** Subscribe to the configuration channel */
export const subscribeConfig = async () => {
  const channel = new Channel<Config>();

  channel.onmessage = (config) => {
    // Load the counts in the background
    getCounts().then((counts) => {
      appState.counts = counts;
    });

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

    // Update the colors
    updateColors(config.colors);

    // Show the window
    getCurrentWindow().show();
  };

  await invoke("subscribe_config", { channel });
};
