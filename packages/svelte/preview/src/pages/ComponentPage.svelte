<script lang="ts">
  import { Eyebrow, Pill, Separator } from "@inflatable-cookie/poodle-svelte";
  import UsageDocs from "../components/UsageDocs.svelte";
  import type { ComponentEntry } from "../component-registry";
  import { componentDocsMap } from "../component-docs";
let { entry, specimenComponent = null }: { entry: ComponentEntry; specimenComponent?: any } = $props();
  let docs = $derived(componentDocsMap[entry.slug] ?? null);</script>

<article class="poodle-component-page">
  <header class="poodle-component-page__hero">
    <div class="poodle-component-page__hero-top">
      <Pill size="lg">{entry.packageName}</Pill>
    </div>
    <h1 class="poodle-component-page__title">{entry.displayName}</h1>
    <p class="poodle-component-page__description">{entry.description}</p>
  </header>

  {#if specimenComponent}
    {@const Specimen = specimenComponent}
    <section class="poodle-component-page__section">
      <Specimen slug={entry.slug} />
    </section>
  {:else}
    <section class="poodle-component-page__section">
      <div class="poodle-component-page__placeholder">
        <p>Specimen not yet available for <strong>{entry.displayName}</strong>.</p>
        <p>Check back as we build out interactive demos for each component.</p>
      </div>
    </section>
  {/if}

  <section class="poodle-component-page__section">
    <h2 class="poodle-component-page__section-title">Import</h2>
    <pre class="poodle-component-page__code"><code>import {"{"} {entry.displayName} {"}"} from "{entry.packageName}";</code></pre>
  </section>

  {#if docs}
    <Separator />
    <UsageDocs {docs} />
  {/if}
</article>

<style>
  .poodle-component-page {
    padding: 1.5rem 2rem;
  }

  .poodle-component-page__hero {
    margin-bottom: 1.5rem;
  }

  .poodle-component-page__hero-top {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 0.5rem;
  }

  .poodle-component-page__title {
    font-size: 2rem;
    font-weight: 700;
    color: var(--poodle-color-text-primary);
    margin: 0.25rem 0 0.75rem;
  }

  .poodle-component-page__description {
    font-size: 1rem;
    line-height: 1.6;
    color: var(--poodle-color-text-secondary);
    margin: 0;
  }

  .poodle-component-page__section {
    padding: 1.5rem 0;
  }

  .poodle-component-page__section-title {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--poodle-color-text-primary);
    margin: 0 0 1rem;
  }

  .poodle-component-page__placeholder {
    padding: 2rem;
    text-align: center;
    color: var(--poodle-color-text-secondary);
    border: 0.0625rem dashed color-mix(in srgb, var(--poodle-color-border-subtle) 60%, transparent);
    border-radius: var(--poodle-radius-surface);
  }

  .poodle-component-page__placeholder p {
    margin: 0.25rem 0;
    font-size: 0.875rem;
  }

  .poodle-component-page__code {
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
