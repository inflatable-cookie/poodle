<script lang="ts">
  import { PickerShell } from "@flint/svelte-composites";
  import { Eyebrow, Surface } from "@flint/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Inline variant (ready)</Eyebrow>
    <PickerShell title="Select a component" description="Browse and select from available components." resultCount={12} variant="inline">
      <Surface padding="sm" border="subtle"><p>Component A</p></Surface>
      <Surface padding="sm" border="subtle"><p>Component B</p></Surface>
      <Surface padding="sm" border="subtle"><p>Component C</p></Surface>
    </PickerShell>
  </div>

  <div class="specimen__group">
    <Eyebrow>No results</Eyebrow>
    <PickerShell title="Select an item" state="no-results" stateTitle="No matches" stateMessage="Try a different search term." variant="inline" />
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

  p {
    margin: 0;
    font-size: 0.875rem;
  }
</style>
