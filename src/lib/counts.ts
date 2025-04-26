/** Manage entry selection counts to sort entries by relevance when in app launcher mode */

import { invoke } from "@tauri-apps/api/core";
import type { Entry } from "./config";
import { appState } from "./config.svelte";

/** Read the counts from file */
export const getCounts = async () =>
  await invoke<Record<string, number>>("get_counts");

/** Write the counts to file */
export const updateCounts = async (counts: Record<string, number>) =>
  await invoke("update_counts", { counts });

/** Sort the entries by selection count */
export const sortEntries = (entries: Entry[]) => {
  const counts = appState.counts;

  return entries.sort((a, b) => {
    const countA = counts[a.name] || 0;
    const countB = counts[b.name] || 0;
    return countB - countA; // Sort in descending order
  });
};

/** Increment the count for an entry */
export const incrementCount = async (entry: Entry) => {
  const name = entry.name;
  const counts = appState.counts;
  counts[name] = (counts[name] || 0) + 1;
  await updateCounts(counts);
};
