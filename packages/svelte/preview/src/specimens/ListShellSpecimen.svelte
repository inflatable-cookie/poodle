<script lang="ts">
  import { ListShell } from "@pug/svelte-composites";
  import { Eyebrow, Surface } from "@pug/svelte-primitives";

  const items = [
    "Configure CI pipeline",
    "Update dependencies",
    "Review pull requests",
    "Deploy staging build",
    "Write migration script",
  ];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Ready state</Eyebrow>
    <div class="shell-container">
      <ListShell state="ready" ariaLabel="Task list">
        {#each items as item}
          <Surface padding="sm" border="subtle">
            <p>{item}</p>
          </Surface>
        {/each}
      </ListShell>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>No results</Eyebrow>
    <div class="shell-container">
      <ListShell state="no-results" ariaLabel="Empty results" stateTitle="No results found" stateMessage="Try adjusting your search or filters." />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Loading</Eyebrow>
    <div class="shell-container">
      <ListShell state="loading" ariaLabel="Loading list" />
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

  .shell-container {
    height: 14rem;
    border: 1px solid var(--pug-color-border-default);
    border-radius: 4px;
    overflow: hidden;
  }

  p {
    margin: 0;
    font-size: 0.875rem;
  }
</style>
