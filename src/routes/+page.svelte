<script lang="ts">
  import { appState } from "$lib/config.svelte";
  import { normalize, filterEntry } from "$lib/utils";
  import { subscribeConfig } from "$lib/load";
  import "../app.css";
  import Entry from "../components/Entry.svelte";

  subscribeConfig();

  let filter = $state("");
  let keywords = $derived(normalize(filter).split(" "));
</script>

<main>
  <input type="text" bind:value={filter} />

  {#if appState.config === null}
    <p>NO CONFIG</p>
  {:else}
    {#each appState.config.entries
      .filter((entry) => filterEntry(entry, keywords))
      .slice(0, 15) as entry (entry.name)}
      <Entry {entry}></Entry>
    {/each}
  {/if}
</main>
