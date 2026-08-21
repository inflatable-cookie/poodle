<script lang="ts">
  import { Button, ErrorBoundary, Surface, Text } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import ErrorBoundaryCrashOnce from "./ErrorBoundaryCrashOnce.svelte";

  let crashKey = $state(0);
  let crashToken = $state({});

  function throwAgain() {
    crashKey += 1;
    crashToken = {};
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
      <Button variant="secondary" size="sm" onClick={throwAgain}>Throw again</Button>
    </div>
    <ErrorBoundary title="Preview failed" retryLabel="Reset boundary">
      {#key crashKey}
        <ErrorBoundaryCrashOnce token={crashToken} />
      {/key}
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
