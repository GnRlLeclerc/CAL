/** Config state */

import type { Colors, Config } from "./config";
import { subscribeConfig } from "./load";

export interface State {
  config: Config | null;
  colors: Colors | null;
}

subscribeConfig();

export const appState: State = $state({ config: null, colors: null });
