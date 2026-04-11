<script lang="ts">
  import { Eyebrow, Pill, Separator } from "@poodle/svelte";
  import UsageDocs from "../components/UsageDocs.svelte";
  import type { ComponentEntry } from "../component-registry";
  import { componentDocsMap } from "../component-docs";

  export let entry: ComponentEntry;
  export let specimenComponent: any = null;

  $: docs = componentDocsMap[entry.slug] ?? null;
</script>

<article class="component-page">
  <header class="component-page__hero">
    <div class="component-page__hero-top">
      <Eyebrow>{entry.tier === "primitive" ? "Primitive" : "Composite"}</Eyebrow>
      <Pill size="lg">{entry.packageName}</Pill>
    </div>
    <h1 class="component-page__title">{entry.displayName}</h1>
    <p class="component-page__description">{entry.description}</p>
  </header>

  {#if specimenComponent}
    <section class="component-page__section">
      <svelte:component this={specimenComponent} />
    </section>
  {:else}
    <section class="component-page__section">
      <div class="component-page__placeholder">
        <p>Specimen not yet available for <strong>{entry.displayName}</strong>.</p>
        <p>Check back as we build out interactive demos for each component.</p>
      </div>
    </section>
  {/if}

  <section class="component-page__section">
    <h2 class="component-page__section-title">Import</h2>
    <pre class="component-page__code"><code>import {"{"} {entry.displayName} {"}"} from "{entry.packageName}";</code></pre>
  </section>

  {#if docs}
    <Separator />
    <UsageDocs {docs} />
  {/if}
</article>

<style>
  .component-page {
    padding: 1.5rem 2rem;
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
    color: var(--poodle-color-text-primary);
    margin: 0.25rem 0 0.75rem;
  }

  .component-page__description {
    font-size: 1rem;
    line-height: 1.6;
    color: var(--poodle-color-text-secondary);
    margin: 0;
  }

  .component-page__section {
    padding: 1.5rem 0;
  }

  .component-page__section-title {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--poodle-color-text-primary);
    margin: 0 0 1rem;
  }

  .component-page__placeholder {
    padding: 2rem;
    text-align: center;
    color: var(--poodle-color-text-secondary);
    border: 0.0625rem dashed color-mix(in srgb, var(--poodle-color-border-subtle) 60%, transparent);
    border-radius: var(--poodle-radius-surface);
  }

  .component-page__placeholder p {
    margin: 0.25rem 0;
    font-size: 0.875rem;
  }

  .component-page__code {
    padding: 0.75rem 1rem;
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-canvas) 90%, transparent);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 50%, transparent);
    font-family: "SF Mono", "Fira Code", monospace;
    font-size: 0.8125rem;
    color: var(--poodle-color-text-primary);
    overflow-x: auto;
    margin: 0;
  }
</style>
