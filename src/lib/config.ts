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

/** The launcher color scheme */
export interface Colors {
  background: string;
  hover: string;
  selected: string;
  text: string;
  textSelected: string;
  textDim: string;
}

/** The launcher configuration for a given screen */
export interface Config {
  daemon: boolean;
  placeholder: string | null;
  mode: "full" | "icon" | "lines" | "compact";
  entries: Entry[];
  colors: Colors;
}

/** Update the root color css variables from the new colors */
export const updateColors = (colors: Colors) => {
  const root = document.documentElement;

  root.style.setProperty("--bg-color", colors.background);
  root.style.setProperty("--bg-hover-color", colors.hover);
  root.style.setProperty("--bg-selected-color", colors.selected);
  root.style.setProperty("--text-color", colors.text);
  root.style.setProperty("--text-selected-color", colors.textSelected);
  root.style.setProperty("--text-dim-color", colors.textDim);
};
