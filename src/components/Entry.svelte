<!-- CAL Menu Entry-->
<script lang="ts">
  import type { Entry } from "$lib/config";
  import { invoke } from "@tauri-apps/api/core";
  import Image from "./Image.svelte";
  import { appState } from "$lib/config.svelte";

  interface Props {
    entry: Entry;
  }

  const { entry }: Props = $props();
  const mode = $derived(appState.config?.mode);

  const iconcls = $derived.by(() => {
    switch (mode) {
      case "compact":
        return "icon-compact";
      default:
        return "icon";
    }
  });

  const onclick = () => {
    invoke("run_command", {
      command: entry.command,
      terminal: entry.terminal,
    });
  };
</script>

<button class="entry-row" {onclick}>
  <Image classes={iconcls} path={entry.icon}></Image>

  {#if mode !== "icon"}
    <div class="entry-content">
      <p class="name">{entry.name}</p>

      {#if mode !== "compact" && mode !== "lines"}
        <p class="description">{entry.description}</p>
      {/if}
    </div>
  {/if}
</button>
