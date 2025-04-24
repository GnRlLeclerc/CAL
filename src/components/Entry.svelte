<!-- CAL Menu Entry-->
<script lang="ts">
  import type { Entry } from "$lib/config";
  import { invoke } from "@tauri-apps/api/core";
  import Image from "./Image.svelte";

  interface Props {
    entry: Entry;
  }

  const { entry }: Props = $props();

  const onclick = () => {
    invoke("run_command", {
      command: entry.command,
    });
  };
</script>

<button class="entry-row" {onclick}>
  <Image path={entry.icon}></Image>
  <div class="entry-content">
    <h3>{entry.name}</h3>
    <p>{entry.description}</p>
  </div>
</button>
