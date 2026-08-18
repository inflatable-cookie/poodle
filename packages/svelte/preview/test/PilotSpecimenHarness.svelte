<script lang="ts">
  import { IconProvider, type IconSet } from "@inflatable-cookie/poodle-svelte";
  import iconNodes from "lucide-static/icon-nodes.json";
  import type { Component } from "svelte";

  // The preview app wraps the whole catalogue in this provider (App.svelte),
  // so a specimen rendered without it reports icons the real page resolves.
  let { specimen }: { specimen: Component<Record<string, never>> } = $props();
</script>

<IconProvider icons={iconNodes as unknown as IconSet}>
  {@const Specimen = specimen}
  <Specimen />
</IconProvider>
