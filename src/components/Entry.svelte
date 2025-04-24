<!-- CAL Menu Entry-->
<script lang="ts">
  import type { Entry } from "$lib/config";
  import Image from "./Image.svelte";

  interface Props {
    entry: Entry;
  }

  const { entry }: Props = $props();
</script>

<div class="entry-row">
  <Image path={entry.icon}></Image>
  <div class="entry-content">
    <h3>{entry.name}</h3>
    <p>{entry.description}</p>
  </div>
</div>
