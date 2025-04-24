<script lang="ts">
  import { appState } from "$lib/config.svelte";
  import { normalize, filterEntry } from "$lib/utils";
  import { subscribeConfig } from "$lib/load";

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
      .slice(0, 5) as entry}
      <p>{entry.name}: {entry.description}: {entry.picture}</p>
    {/each}
  {/if}
</main>
