/** Config state */

import type { Config } from "./config";

export interface State {
  config: Config | null;
}

export const appState: State = $state({ config: null });
