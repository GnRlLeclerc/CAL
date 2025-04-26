/** Config state */

import type { Config } from "./config";
import { subscribeConfig } from "./load";

export interface State {
  config: Config | null;
  counts: Record<string, number>;
}

subscribeConfig();

export const appState: State = $state({ config: null, counts: {} });
