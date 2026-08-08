<script lang="ts">
  import { Button, Select, Surface, TextInput, UiPresentationProvider } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const options = [
    { value: "alpha", label: "Alpha" },
    { value: "beta", label: "Beta" },
  ];
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Compact small scope">
    <UiPresentationProvider density="compact" sizeScale="sm">
      <Surface border="subtle" padding="md">
        <div class="poodle-specimen__controls">
          <Button>Save</Button>
          <TextInput id="provider-compact-name" value="Compact" />
          <Select id="provider-compact-select" {options} value="alpha" />
        </div>
      </Surface>
    </UiPresentationProvider>
  </SpecimenGroup>

  <SpecimenGroup label="Comfortable large scope">
    <UiPresentationProvider density="comfortable" sizeScale="lg">
      <Surface border="subtle" padding="md">
        <div class="poodle-specimen__controls">
          <Button variant="primary">Save</Button>
          <TextInput id="provider-comfortable-name" value="Comfortable" />
          <Select id="provider-comfortable-select" {options} value="beta" />
        </div>
      </Surface>
    </UiPresentationProvider>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__controls {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--poodle-space-stack-md);
  }
</style>
