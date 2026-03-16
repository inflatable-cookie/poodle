<script lang="ts">
  import { Eyebrow, Pill, Separator, Surface } from "@pug/svelte-primitives";
  import type { ComponentEntry } from "../component-registry";

  export let entry: ComponentEntry;
  export let specimenComponent: any = null;
</script>

<article class="component-page">
  <header class="component-page__hero">
    <div class="component-page__hero-top">
      <Eyebrow>{entry.tier === "primitive" ? "Primitive" : entry.tier === "shell" ? "Shell" : "Composite"}</Eyebrow>
      <Pill size="sm">{entry.packageName}</Pill>
    </div>
    <h1 class="component-page__title">{entry.displayName}</h1>
    <p class="component-page__description">{entry.description}</p>
  </header>

  <Separator />

  {#if specimenComponent}
    <section class="component-page__section">
      <h2 class="component-page__section-title">Live demo</h2>
      <Surface tone="panel" border="subtle" padding="md">
        <svelte:component this={specimenComponent} />
      </Surface>
    </section>
    <Separator />
  {:else}
    <section class="component-page__section">
      <h2 class="component-page__section-title">Live demo</h2>
      <div class="component-page__placeholder">
        <p>Specimen not yet available for <strong>{entry.displayName}</strong>.</p>
        <p>Check back as we build out interactive demos for each component.</p>
      </div>
    </section>
    <Separator />
  {/if}

  <section class="component-page__section">
    <h2 class="component-page__section-title">Import</h2>
    <pre class="component-page__code"><code>import {"{"} {entry.displayName} {"}"} from "{entry.packageName}";</code></pre>
  </section>
</article>

<style>
  .component-page {
    padding: 1.5rem 2rem;
    max-width: 52rem;
  }

  .component-page__hero {
    margin-bottom: 1.5rem;
  }

  .component-page__hero-top {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 0.5rem;
  }

  .component-page__title {
    font-size: 2rem;
    font-weight: 700;
    color: var(--pug-color-text-primary);
    margin: 0.25rem 0 0.75rem;
  }

  .component-page__description {
    font-size: 1rem;
    line-height: 1.6;
    color: var(--pug-color-text-secondary);
    margin: 0;
  }

  .component-page__section {
    padding: 1.5rem 0;
  }

  .component-page__section-title {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--pug-color-text-primary);
    margin: 0 0 1rem;
  }

  .component-page__placeholder {
    padding: 2rem;
    text-align: center;
    color: var(--pug-color-text-secondary);
    border: 0.0625rem dashed color-mix(in srgb, var(--pug-color-border-subtle) 60%, transparent);
    border-radius: var(--pug-radius-surface);
  }

  .component-page__placeholder p {
    margin: 0.25rem 0;
    font-size: 0.875rem;
  }

  .component-page__code {
    padding: 0.75rem 1rem;
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-canvas) 90%, transparent);
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 50%, transparent);
    font-family: "SF Mono", "Fira Code", monospace;
    font-size: 0.8125rem;
    color: var(--pug-color-text-primary);
    overflow-x: auto;
    margin: 0;
  }
</style>
