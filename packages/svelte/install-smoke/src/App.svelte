<script lang="ts">
  import { MetricTile } from "@poodle/svelte";
  import { Button, Pill } from "@poodle/svelte";
  import { applyThemeAttributes } from "@poodle/svelte-tokens/runtime";
  import { onMount } from "svelte";

  let shell: HTMLDivElement | null = null;

  onMount(() => {
    if (!shell) return;

    applyThemeAttributes(shell, {
      theme: "eclipse",
      density: "compact",
      controlSize: "md",
    });
  });
</script>

<div class="app-shell" bind:this={shell}>
  <Pill tone="success">Installed</Pill>
  <MetricTile label="Components" value="3" trend="up" trendLabel="resolved" />
  <Button variant="primary">Smoke</Button>
</div>

<style>
  .app-shell {
    display: grid;
    gap: 1rem;
    max-width: 20rem;
    margin: 2rem auto;
  }
</style>
