<script lang="ts">
  import { runCommand } from "$lib/command";
  import { appState } from "$lib/config.svelte";
  import { normalize, filterEntry } from "$lib/utils";
  import { handleClose } from "$lib/close";
  import "../app.css";
  import Entry from "../components/Entry.svelte";
  import SearchIcon from "../components/SearchIcon.svelte";
  import { incrementCount, sortEntries, updateCounts } from "$lib/counts";

  let filter = $state("");
  let keywords = $derived(normalize(filter).split(" "));
  let directioncls = $derived(
    appState.config?.mode === "icon" ? "row" : "column",
  );
  let max = $derived(directioncls === "row" ? 5 : 15);

  let selected: number | null = $state(null);
  let entries = $derived(
    appState.config?.entries.filter((entry) => filterEntry(entry, keywords)),
  );

  const selectNext = () => {
    selected =
      selected === null
        ? 0
        : Math.min(selected + 1, entries?.length ?? Number.MAX_VALUE - 1);
  };

  const selectPrevious = () => {
    if (selected === null || selected <= 0) {
      selected = null;
    } else {
      selected = Math.max(selected - 1, 0);
    }
  };
</script>

<main>
  <div class="input-row">
    <SearchIcon />
    <input
      type="text"
      placeholder={appState.config?.placeholder}
      bind:value={filter}
    />
  </div>

  <div class="separator"></div>

  <div class={["scroll", directioncls]}>
    {#if entries !== undefined}
      {#each sortEntries(entries).slice(0, max) as entry, i (entry.name)}
        <Entry {entry} selected={i === selected}></Entry>
      {/each}
    {/if}
  </div>
</main>

<svelte:window
  on:keydown={(event) => {
    switch (event.key) {
      case "ArrowDown":
        event.preventDefault();
        selectNext();
        break;
      case "ArrowUp":
        event.preventDefault();
        selectPrevious();
        break;
      case "ArrowLeft":
        event.preventDefault();
        selectPrevious();
        break;
      case "ArrowRight":
        event.preventDefault();
        selectNext();
        break;
      case "Tab":
        event.preventDefault();
        selectNext();
        break;
      case "Enter":
        event.preventDefault();
        if (
          selected !== null &&
          entries !== undefined &&
          selected < entries.length
        ) {
          incrementCount(entries[selected]);
          runCommand(entries[selected]);
        } else if (
          selected === null &&
          entries !== undefined &&
          entries.length > 0
        ) {
          incrementCount(entries[0]);
          runCommand(entries[0]);
        }
        filter = "";
        handleClose();
        break;
      case "Escape":
        event.preventDefault();
        filter = "";
        handleClose();
        break;

      default:
        selected = null;
        break;
    }
  }}
/>
