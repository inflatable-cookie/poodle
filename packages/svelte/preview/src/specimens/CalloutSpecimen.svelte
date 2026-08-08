<script lang="ts">
  import { Callout } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let dismissed = $state(false);
</script>

<SpecimenLayout>
  <SpecimenGroup label="Tones" bare>
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

  <SpecimenGroup label="Message prop" bare>
    <Callout tone="info" title="Information" message="This is an informational callout using the message prop instead of slot content." />
  </SpecimenGroup>

  <SpecimenGroup label="Dismissible" bare>
    {#if !dismissed}
      <Callout
        tone="info"
        title="Dismissible callout"
        message="This callout can be dismissed by the user."
        dismissible
        onDismiss={() => (dismissed = true)}
      />
    {:else}
      <Callout tone="success" message="Dismiss callback fired." />
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="Without title" bare>
    <Callout tone="info">
      A simple inline callout without a title for brief contextual notes.
    </Callout>
  </SpecimenGroup>

  <SpecimenGroup label="With actions" bare>
    <Callout tone="warning" title="Quota warning" message="API usage is approaching the current workspace limit.">
      {#snippet actions()}
        <button type="button" class="poodle-callout-action">Review limits</button>
      {/snippet}
    </Callout>
  </SpecimenGroup>

  {#snippet sizes(size)}
    <Callout tone="info" {size} title="Callout at {size}">
      Text and icon chrome scale with the size prop.
    </Callout>
  {/snippet}

  {#snippet densities(density)}
    <Callout tone="info" {density} title="Callout at {density} density">
      Internal spacing adjusts with the density prop.
    </Callout>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-callout-action {
    min-height: 0;
    padding: 0.375rem 0.625rem;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-control);
    background: transparent;
    color: var(--poodle-color-text-primary);
    font: inherit;
    cursor: pointer;
  }
</style>
