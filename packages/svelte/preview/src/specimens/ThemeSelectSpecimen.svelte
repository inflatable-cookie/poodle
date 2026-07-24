<script lang="ts">
  import { ThemeSelect } from "@poodle/svelte";
  import { themeOptions } from "@poodle/svelte-tokens";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  // Standalone mode: explicit theme catalogue + local value (does not apply the
  // theme to the document — that is the controller's job, see usage docs).
  const themes = themeOptions();

  let value = $state("dark");
  let sizeValue = $state("nord");
  let densityValue = $state("rose");
  const selected = $derived(themes.find((t) => t.value === value));
</script>

<SpecimenLayout>
  <SpecimenGroup label="Theme selector (standalone, live value)">
    <ThemeSelect {themes} bind:value onChange={(v) => (value = v)} />
    <pre>selected: {value} — {selected?.label}</pre>
  </SpecimenGroup>

  <SpecimenGroup label="Compact trigger (no label)">
    <ThemeSelect {themes} value="midnight" showLabel={false} />
  </SpecimenGroup>

  <SpecimenGroup label="Four columns">
    <ThemeSelect {themes} value="solarized" columns={4} />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <ThemeSelect {themes} value="forest" disabled />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <ThemeSelect {themes} {size} bind:value={sizeValue} />
  {/snippet}

  {#snippet densities(density)}
    <ThemeSelect {themes} {density} bind:value={densityValue} />
  {/snippet}
</SpecimenLayout>

<style>
  pre {
    margin: 0;
    font-size: 0.75rem;
  }
</style>
