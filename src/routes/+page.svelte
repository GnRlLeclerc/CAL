<script lang="ts">
  import { runCommand } from "$lib/command";
  import { appState } from "$lib/config.svelte";
  import { normalize, filterEntry } from "$lib/utils";
  import { handleClose } from "$lib/close";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import "../app.css";
  import Entry from "../components/Entry.svelte";
  import SearchIcon from "../components/SearchIcon.svelte";

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
    <input type="text" placeholder="Search for apps..." bind:value={filter} />
  </div>

  <div class="separator"></div>

  <div class={["scroll", directioncls]}>
    {#if entries !== undefined}
      {#each entries.slice(0, max) as entry, i (entry.name)}
        <Entry {entry} selected={i === selected}></Entry>
      {/each}
    {/if}
  </div>
</main>

<svelte:window
  on:keydown={(event) => {
    console.log(event.key);
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
          runCommand(entries[selected]);
        } else if (
          selected === null &&
          entries !== undefined &&
          entries.length > 0
        ) {
          runCommand(entries[0]);
        }
        break;
      case "Escape":
        event.preventDefault();
        handleClose();
        break;

      default:
        selected = null;
        break;
    }
  }}
/>
