<script lang="ts">
  import { Combobox, Eyebrow, type ComboboxOption } from "@poodle/svelte-primitives";

  const frameworks: ComboboxOption[] = [
    { value: "svelte", label: "Svelte" },
    { value: "react", label: "React" },
    { value: "vue", label: "Vue" },
    { value: "angular", label: "Angular" },
    { value: "solid", label: "SolidJS" },
    { value: "lit", label: "Lit" },
  ];

  const withDescriptions: ComboboxOption[] = [
    { value: "postgres", label: "PostgreSQL", description: "Relational database" },
    { value: "mongo", label: "MongoDB", description: "Document database" },
    { value: "redis", label: "Redis", description: "In-memory key-value store" },
    { value: "sqlite", label: "SQLite", description: "Embedded relational database" },
  ];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let selected = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <Combobox
      options={frameworks}
      placeholder="Choose a framework…"
      ariaLabel="Framework"
      on:valueChange={(e) => (selected = e.detail.value)}
    />
    {#if selected}
      <p>Selected: <strong>{selected}</strong></p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>With descriptions</Eyebrow>
    <Combobox options={withDescriptions} placeholder="Choose a database…" ariaLabel="Database" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <Combobox options={frameworks} placeholder={size.toUpperCase()} ariaLabel={"Framework at " + size} {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each ["compact", "default", "comfortable"] as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <Combobox options={frameworks} placeholder="Choose a framework…" ariaLabel={"Framework at " + density + " density"} {density} />
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <Combobox options={frameworks} placeholder="Disabled" ariaLabel="Disabled combobox" disabled />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 20rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
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
