<script lang="ts">
  import {
    Button,
    Field,
    ModelConnectionSetup,
    TextInput,
  } from "@inflatable-cookie/poodle-svelte";
  import { MODEL_CONNECTION_PICKER_FIXTURES } from "@inflatable-cookie/poodle-core";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const options = MODEL_CONNECTION_PICKER_FIXTURES;
  const interactiveOptions = options.map((option) =>
    option.id === "codex-app"
      ? { ...option, availability: "available" as const, availabilityLabel: "Available", isDisabled: false }
      : option,
  );
  const missingOptions = options.map((option) =>
    option.id === "codex-app"
      ? {
          ...option,
          availability: "unavailable" as const,
          availabilityLabel: "Not detected",
          isDisabled: true,
        }
      : option,
  );

  let apiKey = $state("");
  let endpoint = $state("http://127.0.0.1:11434");
</script>

<SpecimenLayout showSizes={false} showDensities={false}>
  {#snippet children()}
    <div class="poodle-model-connection-setup-specimen">
      <SpecimenGroup label="Choose a connection">
        <div class="poodle-model-connection-setup-specimen__panel">
          <ModelConnectionSetup
            options={interactiveOptions}
            defaultValue="openai-responses"
            canSubmit={true}
          >
            {#snippet configuration({ option })}
              {#if option.id === "ollama-local"}
                <Field id="mcs-choose-endpoint" label="Endpoint URL">
                  <TextInput
                    id="mcs-choose-endpoint"
                    value={endpoint}
                    placeholder="http://127.0.0.1:11434"
                    onValueChange={(value) => (endpoint = value)}
                  />
                </Field>
              {:else if option.id === "anthropic-messages"}
                <Button variant="secondary">Sign in with browser</Button>
              {:else if option.id !== "codex-app"}
                <Field id="mcs-choose-api-key" label="API key">
                  <TextInput
                    id="mcs-choose-api-key"
                    type="password"
                    value={apiKey}
                    placeholder="sk-demo-placeholder"
                    onValueChange={(value) => (apiKey = value)}
                  />
                </Field>
              {/if}
            {/snippet}
          </ModelConnectionSetup>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Configure: API key">
        <div class="poodle-model-connection-setup-specimen__panel">
          <ModelConnectionSetup
            {options}
            defaultStage="configure"
            defaultValue="openai-responses"
            canSubmit={true}
          >
            {#snippet configuration()}
              <Field id="mcs-api-key" label="API key">
                <TextInput
                  id="mcs-api-key"
                  type="password"
                  value={apiKey}
                  placeholder="sk-demo-placeholder"
                  onValueChange={(value) => (apiKey = value)}
                />
              </Field>
            {/snippet}
          </ModelConnectionSetup>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Auto-detected local route">
        <p class="poodle-model-connection-setup-specimen__note">
          This route needs no credentials, so there is no configuration step to
          emit. Both examples stay on <em>choose</em>: the action reads Add, not
          Continue, and there is no Back. Detection is the host's — Poodle only
          renders the outcome it was handed.
        </p>
        <div class="poodle-model-connection-setup-specimen__stack">
          <div class="poodle-model-connection-setup-specimen__panel">
            <ModelConnectionSetup
              options={interactiveOptions}
              defaultValue="codex-app"
              canSubmit={true}
            />
          </div>
          <!-- Nothing was found, so the option says so and Add stays disabled. -->
          <div class="poodle-model-connection-setup-specimen__panel">
            <ModelConnectionSetup
              options={missingOptions}
              defaultValue="codex-app"
            />
          </div>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="OAuth in progress">
        <div class="poodle-model-connection-setup-specimen__panel">
          <ModelConnectionSetup
            {options}
            defaultStage="configure"
            defaultValue="anthropic-messages"
            isPending={true}
            pendingLabel="Waiting for browser sign-in"
          >
            {#snippet configuration()}
              <Button variant="secondary">Sign in with browser</Button>
            {/snippet}
          </ModelConnectionSetup>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Local endpoint">
        <div class="poodle-model-connection-setup-specimen__panel">
          <ModelConnectionSetup
            {options}
            defaultStage="configure"
            defaultValue="ollama-local"
            canSubmit={true}
          >
            {#snippet configuration()}
              <Field id="mcs-endpoint" label="Endpoint URL">
                <TextInput
                  id="mcs-endpoint"
                  value={endpoint}
                  placeholder="http://127.0.0.1:11434"
                  onValueChange={(value) => (endpoint = value)}
                />
              </Field>
            {/snippet}
          </ModelConnectionSetup>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Validation and pending">
        <div class="poodle-model-connection-setup-specimen__stack">
          <div class="poodle-model-connection-setup-specimen__panel">
            <ModelConnectionSetup
              {options}
              defaultStage="configure"
              defaultValue="openai-responses"
              canSubmit={false}
              error="API key format is invalid."
            >
              {#snippet configuration()}
                <Field id="mcs-invalid-key" label="API key">
                  <TextInput
                    id="mcs-invalid-key"
                    type="password"
                    value="••••••••"
                    placeholder="sk-demo-placeholder"
                    readOnly
                  />
                </Field>
              {/snippet}
            </ModelConnectionSetup>
          </div>
          <div class="poodle-model-connection-setup-specimen__panel">
            <ModelConnectionSetup
              {options}
              defaultStage="configure"
              defaultValue="openai-responses"
              canSubmit={true}
              isPending={true}
              pendingLabel="Checking connection"
            >
              {#snippet configuration()}
                <Field id="mcs-pending-key" label="API key">
                  <TextInput
                    id="mcs-pending-key"
                    type="password"
                    value="••••••••"
                    placeholder="sk-demo-placeholder"
                    readOnly
                  />
                </Field>
              {/snippet}
            </ModelConnectionSetup>
          </div>
        </div>
      </SpecimenGroup>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-model-connection-setup-specimen {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .poodle-model-connection-setup-specimen__panel {
    width: min(42rem, 100%);
  }

  .poodle-model-connection-setup-specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-model-connection-setup-specimen__note {
    margin: 0 0 0.75rem;
    font-size: 0.875rem;
    opacity: 0.75;
  }
</style>
