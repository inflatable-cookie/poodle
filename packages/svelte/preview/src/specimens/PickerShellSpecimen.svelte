<script lang="ts">
  import { PickerShell, Surface, UiPresentationProvider } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout showSizes={false} showDensities={true} bareVariants>
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

  {#snippet densities(density)}
    <UiPresentationProvider {density}>
      <div class="poodle-specimen__density-demo">
        <PickerShell
          title="Select a component"
          description="Browse and select from available components."
          resultCount={12}
          selectionCount={2}
          variant="inline"
        >
          <Surface padding="sm" border="subtle"><p>Component A</p></Surface>
          <Surface padding="sm" border="subtle"><p>Component B</p></Surface>
        </PickerShell>
      </div>
    </UiPresentationProvider>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen__density-demo {
    width: min(100%, 28rem);
  }

  p {
    margin: 0;
    font-size: 0.875rem;
  }
</style>
