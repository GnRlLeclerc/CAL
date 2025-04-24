/** Configuration types */

/** A launcher entry */
export interface Entry {
  name: string;
  command: string;
  icon: string | null;
  description: string | null;
  keywords: string[] | null;
  terminal: boolean;

  // Utility (computed when fetched)
  allKeywords: string[];
}

/** The launcher configuration for a given screen */
export interface Config {
  entries: Entry[];
}
