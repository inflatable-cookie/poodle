<script lang="ts">
  import { PickerShell } from "@poodle/svelte";
  import { Surface } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
  <SpecimenGroup label="Inline variant (ready)">
    <PickerShell title="Select a component" description="Browse and select from available components." resultCount={12} variant="inline">
      <Surface padding="sm" border="subtle"><p>Component A</p></Surface>
      <Surface padding="sm" border="subtle"><p>Component B</p></Surface>
      <Surface padding="sm" border="subtle"><p>Component C</p></Surface>
    </PickerShell>
  </SpecimenGroup>

  <SpecimenGroup label="No results">
    <PickerShell title="Select an item" state="no-results" stateTitle="No matches" stateMessage="Try a different search term." variant="inline" />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  p {
    margin: 0;
    font-size: 0.875rem;
  }
</style>
