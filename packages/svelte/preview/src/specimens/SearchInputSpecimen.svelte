<script lang="ts">
  import { SearchInput, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let query = "";
  let lastSubmit = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <SearchInput
      id="search-default"
      value={query}
      placeholder="Search components..."
      ariaLabel="Search components"
      on:valueChange={(e) => (query = e.detail.value)}
      on:submit={(e) => (lastSubmit = e.detail.value)}
      on:clear={() => { query = ""; lastSubmit = ""; }}
    />
    {#if query}
      <p>Filtering: <strong>{query}</strong></p>
    {/if}
    {#if lastSubmit}
      <p>Submitted: <strong>{lastSubmit}</strong></p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <SearchInput id={"size-" + size} placeholder={size.toUpperCase()} ariaLabel={"Search at " + size} {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each ["compact", "default", "comfortable"] as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <SearchInput id={"density-" + density} placeholder="Search" {density} />
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <SearchInput
      id="search-disabled"
      value="locked query"
      ariaLabel="Disabled search"
      disabled
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Read-only</Eyebrow>
    <SearchInput
      id="search-readonly"
      value="active filter"
      ariaLabel="Read-only search"
      readOnly
    />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 24rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }
</style>
