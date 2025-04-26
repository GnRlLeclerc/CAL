/** Config state */

import type { Config } from "./config";
import { subscribeConfig } from "./load";

export interface State {
  config: Config | null;
}

subscribeConfig();

export const appState: State = $state({ config: null });
