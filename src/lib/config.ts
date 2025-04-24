/** Configuration types */

/** A launcher entry */
export interface Entry {
  name: string;
  command: string;
  picture: string | null;
  description: string | null;
  keywords: string[] | null;

  // Utility (computed when fetched)
  allKeywords: string[];
}

/** The launcher configuration for a given screen */
export interface Config {
  entries: Entry[];
}
