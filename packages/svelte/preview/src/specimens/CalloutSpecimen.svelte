<script lang="ts">
  import { Callout } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;
</script>

<div class="specimen">
  <SpecimenGroup label="Tones">
    <Callout tone="neutral" title="Neutral callout">
      This is a general informational message with no specific severity.
    </Callout>
    <Callout tone="info" title="Info">
      Your changes have been saved and will take effect on next deploy.
    </Callout>
    <Callout tone="success" title="Success">
      All tests passed. The build is ready for production.
    </Callout>
    <Callout tone="warning" title="Warning">
      This API key expires in 7 days. Rotate it to avoid service interruption.
    </Callout>
    <Callout tone="danger" title="Error">
      Unable to connect to the database. Check your credentials and try again.
    </Callout>
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <Callout tone="info" title="Callout at {size}" {size}>
          Text and icon chrome scale with the size prop.
        </Callout>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <Callout tone="info" title="Callout at {density} density" {density}>
          Internal spacing adjusts with the density prop.
        </Callout>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Message prop">
    <Callout tone="info" title="Information" message="This is an informational callout using the message prop instead of slot content." />
  </SpecimenGroup>

  <SpecimenGroup label="Dismissible">
    <Callout
      tone="info"
      title="Dismissible callout"
      message="This callout can be dismissed by the user."
      dismissible
    />
  </SpecimenGroup>

  <SpecimenGroup label="Without title">
    <Callout tone="info">
      A simple inline callout without a title for brief contextual notes.
    </Callout>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
