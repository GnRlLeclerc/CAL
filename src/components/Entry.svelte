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
      case "full":
        return "icon-full";
      case "icon":
        return "icon-large";
      default:
        return "icon";
    }
  });

  const radiuscls = $derived.by(() => {
    switch (mode) {
      case "compact":
        return "radius-compact";
      case "full":
        return "radius-full";
      case "lines":
        return "radius-lines";
      default:
        return "radius-compact";
    }
  });

  const heightcls = $derived.by(() => {
    switch (mode) {
      case "compact":
        return "height-compact";
      case "full":
        return "height-full";
      case "lines":
        return "height-lines";
      case "icon":
        return undefined;
      default:
        return "height-compact";
    }
  });

  const onclick = () => {
    invoke("run_command", {
      command: entry.command,
      terminal: entry.terminal,
    });
  };
</script>

<button class={["entry-row", radiuscls, heightcls]} {onclick}>
  {#if mode !== "lines"}
    <Image classes={iconcls} path={entry.icon}></Image>
  {/if}

  {#if mode !== "icon"}
    <div class={["entry-content", heightcls]}>
      <p class="name">{entry.name}</p>

      {#if mode !== "compact" && mode !== "lines"}
        <p class="description">{entry.description}</p>
      {/if}
    </div>
  {/if}
</button>
