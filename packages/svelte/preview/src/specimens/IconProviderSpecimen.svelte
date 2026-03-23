<script lang="ts">
  import { Icon, IconProvider, Eyebrow } from "@flint/svelte-primitives";
  import type { IconSet, IconNodes } from "@flint/svelte-primitives";
  import iconNodes from "lucide-static/icon-nodes.json";

  // A small custom icon set to demonstrate swappability
  const customIcons: IconSet = {
    "check": [["path", { "d": "M20 6 9 17l-5-5" }]],
    "star": [["polygon", { "points": "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }]],
    "heart": [["path", { "d": "M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" }]],
    "x": [["path", { "d": "M18 6 6 18" }], ["path", { "d": "m6 6 12 12" }]],
  };

  const sampleNames = ["rocket", "flame", "shield-check", "globe", "compass", "anchor", "cpu", "database"];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Full Lucide set via IconProvider</Eyebrow>
    <p class="hint">
      Wrap a subtree in <code>&lt;IconProvider icons={"{iconNodes}"}&gt;</code> to
      make all {Object.keys(iconNodes).length} Lucide icons available by name.
      Icons resolve from this set first, then fall back to the 35 built-in internals.
    </p>
    <IconProvider icons={iconNodes}>
      <div class="icon-row">
        {#each sampleNames as name}
          <div class="labeled-icon">
            <Icon icon={name} size="md" />
            <span class="label">{name}</span>
          </div>
        {/each}
      </div>
    </IconProvider>
  </div>

  <div class="specimen__group">
    <Eyebrow>Custom icon set</Eyebrow>
    <p class="hint">
      Any <code>Record&lt;string, IconNodes&gt;</code> works as an icon set.
      You can supply a Phosphor equivalent, a subset, or your own custom icons.
    </p>
    <IconProvider icons={customIcons}>
      <div class="icon-row">
        {#each Object.keys(customIcons) as name}
          <div class="labeled-icon">
            <Icon icon={name} size="md" />
            <span class="label">{name}</span>
          </div>
        {/each}
      </div>
    </IconProvider>
  </div>

  <div class="specimen__group">
    <Eyebrow>Without IconProvider — built-in internals</Eyebrow>
    <p class="hint">
      Without any <code>IconProvider</code>, string names resolve to the 35
      built-in icons used for component chrome (chevrons, check, x, etc.).
    </p>
    <div class="icon-row">
      <div class="labeled-icon">
        <Icon icon="check" size="md" />
        <span class="label">check</span>
      </div>
      <div class="labeled-icon">
        <Icon icon="chevron-down" size="md" />
        <span class="label">chevron-down</span>
      </div>
      <div class="labeled-icon">
        <Icon icon="x" size="md" />
        <span class="label">x</span>
      </div>
      <div class="labeled-icon">
        <Icon icon="search" size="md" />
        <span class="label">search</span>
      </div>
      <div class="labeled-icon">
        <Icon icon="plus" size="md" />
        <span class="label">plus</span>
      </div>
    </div>
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .hint {
    font-size: 0.75rem;
    color: var(--flint-color-text-secondary);
    line-height: 1.5;
    margin: 0;
  }

  .hint code {
    font-family: var(--flint-typography-code-family);
    font-size: 0.6875rem;
    padding: 0.0625rem 0.25rem;
    border-radius: 0.1875rem;
    background: color-mix(in srgb, var(--flint-color-background-surface) 64%, transparent);
  }

  .icon-row {
    display: flex;
    gap: 1rem;
    align-items: flex-start;
    flex-wrap: wrap;
  }

  .labeled-icon {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
    min-width: 4rem;
  }

  .label {
    font-size: 0.5625rem;
    font-family: var(--flint-typography-code-family);
    color: var(--flint-color-text-muted);
    text-align: center;
    word-break: break-all;
    line-height: 1.3;
  }
</style>
