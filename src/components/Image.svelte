<!-- Display an image from Disk using an absolute path -->
<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/core";

  interface Props {
    path: string | null;
    classes?: string;
  }

  const { path, classes }: Props = $props();
  const filename =
    path !== null
      ? path.split("/").pop()!.split(".").slice(0, -1).join(".")
      : null;
  const src = path !== null ? convertFileSrc(path) : null;
</script>

{#if src === null}
  <div class="default-icon"></div>
{:else}
  <img
    {src}
    alt={`${filename} icon`}
    class={["icon", classes]}
    draggable="false"
  />
{/if}
