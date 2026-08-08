<script lang="ts">
  import { Icon, IconProvider } from "@inflatable-cookie/poodle-svelte";
  import type { IconSet, IconNodes } from "@inflatable-cookie/poodle-svelte";
  import iconNodes from "lucide-static/icon-nodes.json";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  // A small custom icon set to demonstrate swappability
  const customIcons: IconSet = {
    "check": [["path", { "d": "M20 6 9 17l-5-5" }]],
    "star": [["polygon", { "points": "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }]],
    "heart": [["path", { "d": "M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" }]],
    "x": [["path", { "d": "M18 6 6 18" }], ["path", { "d": "m6 6 12 12" }]],
  };

  const sampleNames = ["rocket", "flame", "shield-check", "globe", "compass", "anchor", "cpu", "database"];
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Full Lucide set via IconProvider">
    <p class="poodle-hint">
      Wrap a subtree in <code>&lt;IconProvider icons={"{iconNodes}"}&gt;</code> to
      make all {Object.keys(iconNodes).length} Lucide icons available by name.
      Icons resolve from this set first, then fall back to the 35 built-in internals.
    </p>
    <IconProvider icons={iconNodes as unknown as IconSet}>
      <div class="poodle-icon-row">
        {#each sampleNames as name}
          <div class="poodle-labeled-icon">
            <Icon icon={name} />
            <span class="poodle-label">{name}</span>
          </div>
        {/each}
      </div>
    </IconProvider>
  </SpecimenGroup>

  <SpecimenGroup label="Custom icon set">
    <p class="poodle-hint">
      Any <code>Record&lt;string, IconNodes&gt;</code> works as an icon set.
      You can supply a Phosphor equivalent, a subset, or your own custom icons.
    </p>
    <IconProvider icons={customIcons}>
      <div class="poodle-icon-row">
        {#each Object.keys(customIcons) as name}
          <div class="poodle-labeled-icon">
            <Icon icon={name} />
            <span class="poodle-label">{name}</span>
          </div>
        {/each}
      </div>
    </IconProvider>
  </SpecimenGroup>

  <SpecimenGroup label="Without IconProvider — built-in internals">
    <p class="poodle-hint">
      Without any <code>IconProvider</code>, string names resolve to the 35
      built-in icons used for component chrome (chevrons, check, x, etc.).
    </p>
    <div class="poodle-icon-row">
      <div class="poodle-labeled-icon">
        <Icon icon="check" />
        <span class="poodle-label">check</span>
      </div>
      <div class="poodle-labeled-icon">
        <Icon icon="chevron-down" />
        <span class="poodle-label">chevron-down</span>
      </div>
      <div class="poodle-labeled-icon">
        <Icon icon="x" />
        <span class="poodle-label">x</span>
      </div>
      <div class="poodle-labeled-icon">
        <Icon icon="search" />
        <span class="poodle-label">search</span>
      </div>
      <div class="poodle-labeled-icon">
        <Icon icon="plus" />
        <span class="poodle-label">plus</span>
      </div>
    </div>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-hint {
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
    line-height: 1.5;
    margin: 0;
  }

  .poodle-hint code {
    font-family: var(--poodle-typography-code-family);
    font-size: 0.6875rem;
    padding: 0.0625rem 0.25rem;
    border-radius: 0.1875rem;
    background: color-mix(in srgb, var(--poodle-color-background-surface) 64%, transparent);
  }

  .poodle-icon-row {
    display: flex;
    gap: 1rem;
    align-items: flex-start;
    flex-wrap: wrap;
  }

  .poodle-labeled-icon {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
    min-width: 4rem;
  }

  .poodle-label {
    font-size: 0.5625rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    text-align: center;
    word-break: break-all;
    line-height: 1.3;
  }
</style>
