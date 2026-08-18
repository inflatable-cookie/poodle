<script lang="ts">
  import { PickerShell, Surface } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout showSizes={false} bareVariants>
  {#snippet children()}
    <SpecimenGroup label="Inline variant (ready)" bare>
      <PickerShell
        title="Select a component"
        description="Browse and select from available components."
        resultCount={12}
        variant="inline"
      >
        <Surface padding="sm" border="subtle"><p>Component A</p></Surface>
        <Surface padding="sm" border="subtle"><p>Component B</p></Surface>
        <Surface padding="sm" border="subtle"><p>Component C</p></Surface>
      </PickerShell>
    </SpecimenGroup>

    <SpecimenGroup label="No results" bare>
      <PickerShell
        title="Select an item"
        state="no-results"
        stateTitle="No matches"
        stateMessage="Try a different search term."
        variant="inline"
      />
    </SpecimenGroup>
  {/snippet}
</SpecimenLayout>

<style>
  p {
    margin: 0;
    font-size: 0.875rem;
  }
</style>
