<script lang="ts">
  import { Button, ErrorBoundary, Surface, Text } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  let shouldThrow = $state(true);

  function throwRenderError(): string {
    throw new Error("Preview child failed during render.");
  }
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Normal children">
    <ErrorBoundary>
      <Surface border="subtle" padding="md">
        <Text>Stable child content renders without boundary chrome.</Text>
      </Surface>
    </ErrorBoundary>
  </SpecimenGroup>

  <SpecimenGroup label="Caught render error">
    <div class="poodle-specimen__actions">
      <Button variant="secondary" size="sm" onClick={() => (shouldThrow = true)}>Throw again</Button>
    </div>
    <ErrorBoundary title="Preview failed" retryLabel="Reset boundary">
      {#if shouldThrow}
        {throwRenderError()}
      {:else}
        <Text>Recovered child content.</Text>
      {/if}
    </ErrorBoundary>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__actions {
    margin-bottom: 0.75rem;
  }
</style>
