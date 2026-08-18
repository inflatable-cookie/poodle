<script lang="ts">
  import { Icon, ModelConnectionPicker } from "@inflatable-cookie/poodle-svelte";
  import { MODEL_CONNECTION_PICKER_FIXTURES } from "@inflatable-cookie/poodle-core";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const options = MODEL_CONNECTION_PICKER_FIXTURES;
</script>

<SpecimenLayout showSizes={false} showDensities={false}>
  {#snippet children()}
    <div class="poodle-model-connection-picker-specimen">
      <SpecimenGroup label="Grouped catalogue">
        <p class="poodle-model-connection-picker-specimen__note">
          Many providers, one of them with several routes. Every availability
          posture — available, checking, unavailable, unsupported — appears on its
          matching option here.
        </p>
        <div class="poodle-model-connection-picker-specimen__panel">
          <ModelConnectionPicker {options} defaultValue="openai-responses" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Search results">
        <div class="poodle-model-connection-picker-specimen__stack">
          <div class="poodle-model-connection-picker-specimen__panel">
            <ModelConnectionPicker {options} defaultQuery="anthropic" defaultValue="anthropic-messages" />
          </div>
          <div class="poodle-model-connection-picker-specimen__panel">
            <ModelConnectionPicker {options} defaultQuery="zzzznothing" />
          </div>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Catalogue states and host lock">
        <div class="poodle-model-connection-picker-specimen__stack">
          <div class="poodle-model-connection-picker-specimen__panel">
            <ModelConnectionPicker {options} state="loading" />
          </div>
          <div class="poodle-model-connection-picker-specimen__panel">
            <ModelConnectionPicker {options} state="error" />
          </div>
          <div class="poodle-model-connection-picker-specimen__panel">
            <ModelConnectionPicker options={[]} state="empty" />
          </div>
          <!-- The catalogue is fine; the host has locked search and options. -->
          <div class="poodle-model-connection-picker-specimen__panel">
            <ModelConnectionPicker {options} defaultValue="openai-responses" isDisabled />
          </div>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Host provider marks and footer">
        <p class="poodle-model-connection-picker-specimen__note">
          A host mark keyed by option id. The generic connection icon stays on
          every option the host did not name.
        </p>
        <div class="poodle-model-connection-picker-specimen__panel">
          <ModelConnectionPicker {options} defaultValue="ollama-local">
            {#snippet leading({ option })}
              {#if option.id === "ollama-local"}
                <Icon name="terminal" />
              {/if}
            {/snippet}
            {#snippet footer()}
              Connections are managed by the host application.
            {/snippet}
          </ModelConnectionPicker>
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

  .poodle-model-connection-picker-specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-model-connection-picker-specimen__narrow {
    width: min(20rem, 100%);
  }

  .poodle-model-connection-picker-specimen__note {
    margin: 0 0 0.75rem;
    font-size: 0.875rem;
    opacity: 0.75;
  }
</style>
