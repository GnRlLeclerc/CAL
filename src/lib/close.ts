/** Closing window logic */

import { getCurrentWindow } from "@tauri-apps/api/window";

export const handleClose = () => {
  // TODO: get daemon status from state, and really close the window if not daemonized
  getCurrentWindow().hide();
};
