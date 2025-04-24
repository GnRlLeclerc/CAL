/** Utilities */

import removeAccents from "remove-accents";
import type { Entry } from "./config";

/** Normalize a string for comparison */
export const normalize = (s: string) => removeAccents(s.toLowerCase());

/** Filter an entry by comparing a list of substrings to its keywords.
 * The subbstrings must be normalized first
 */
export const filterEntry = (entry: Entry, substrings: string[]): boolean => {
  return substrings.every((substring) =>
    entry.allKeywords.some((keyword) => keyword.includes(substring)),
  );
};
