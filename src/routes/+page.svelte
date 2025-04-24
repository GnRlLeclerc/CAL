<script lang="ts">
  import { appState } from "$lib/config.svelte";
  import { normalize, filterEntry } from "$lib/utils";
  import "../app.css";
  import Entry from "../components/Entry.svelte";
  import SearchIcon from "../components/SearchIcon.svelte";

  let filter = $state("");
  let keywords = $derived(normalize(filter).split(" "));
  let directioncls = $derived(
    appState.config?.mode === "icon" ? "row" : "column",
  );
  let max = $derived(directioncls === "row" ? 5 : 15);
</script>

<main>
  <div class="input-row">
    <SearchIcon />
    <input type="text" placeholder="Search for apps..." bind:value={filter} />
  </div>

  <div class="separator"></div>

  <div class={["scroll", directioncls]}>
    {#if appState.config !== null}
      {#each appState.config.entries
        .filter((entry) => filterEntry(entry, keywords))
        .slice(0, max) as entry (entry.name)}
        <Entry {entry}></Entry>
      {/each}
    {/if}
  </div>
</main>
