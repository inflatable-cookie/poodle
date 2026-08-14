<script lang="ts">
  import { ModelConnectionPicker } from "@inflatable-cookie/poodle-svelte";
  import { MODEL_CONNECTION_PICKER_FIXTURES } from "@inflatable-cookie/poodle-core";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const options = MODEL_CONNECTION_PICKER_FIXTURES;
</script>

<SpecimenLayout showSizes={false} showDensities={false}>
  {#snippet children()}
    <div class="poodle-model-connection-picker-specimen">
      <SpecimenGroup label="Grouped catalogue (many providers, one provider with several routes)">
        <div class="poodle-model-connection-picker-specimen__panel">
          <ModelConnectionPicker {options} defaultValue="openai-responses" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Availability: available, checking, unavailable, unsupported">
        <p class="poodle-model-connection-picker-specimen__note">
          Each posture appears on its matching option in the grouped catalogue above.
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="Query with results">
        <div class="poodle-model-connection-picker-specimen__panel">
          <ModelConnectionPicker {options} defaultQuery="anthropic" defaultValue="anthropic-messages" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Query with no results">
        <div class="poodle-model-connection-picker-specimen__panel">
          <ModelConnectionPicker {options} defaultQuery="zzzznothing" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Loading">
        <div class="poodle-model-connection-picker-specimen__panel">
          <ModelConnectionPicker {options} state="loading" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Error">
        <div class="poodle-model-connection-picker-specimen__panel">
          <ModelConnectionPicker {options} state="error" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Empty catalogue">
        <div class="poodle-model-connection-picker-specimen__panel">
          <ModelConnectionPicker options={[]} state="empty" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Narrow layout">
        <div class="poodle-model-connection-picker-specimen__narrow">
          <ModelConnectionPicker {options} defaultValue="ollama-local" />
        </div>
      </SpecimenGroup>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-model-connection-picker-specimen {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .poodle-model-connection-picker-specimen__panel {
    width: min(42rem, 100%);
  }

  .poodle-model-connection-picker-specimen__narrow {
    width: min(20rem, 100%);
  }

  .poodle-model-connection-picker-specimen__note {
    margin: 0;
    font-size: 0.875rem;
    opacity: 0.75;
  }
</style>
