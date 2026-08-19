<script lang="ts">
  import {
    IconButton,
    Icon,
    ModelCatalogueEditor,
    ModelConnectionCard,
    Pill,
    UpdateCenter,
  } from "@inflatable-cookie/poodle-svelte";
  import {
    MODEL_CATALOGUE_FIXTURES,
    MODEL_CONNECTION_CARD_FIXTURES,
  } from "@inflatable-cookie/poodle-core";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const [work, personal, codex, anthropic, ollama] = MODEL_CONNECTION_CARD_FIXTURES;

  const updateOffer = {
    status: { kind: "ready" } as const,
    availability: {
      state: "offer",
      version: "1.4.0",
      reason: "staged",
      notes: "Faster renders, a rebuilt automation pass, and two crash fixes.",
    } as const,
  };

  function cardProps(fixture: (typeof MODEL_CONNECTION_CARD_FIXTURES)[number]) {
    return {
      id: fixture.id,
      title: fixture.title,
      providerLabel: fixture.providerLabel,
      routeLabel: fixture.routeLabel,
      version: fixture.version,
      accessSummary: fixture.accessSummary,
      readiness: fixture.readiness,
      readinessLabel: fixture.readinessLabel,
      isEnabled: fixture.enabled,
    };
  }
</script>

<SpecimenLayout showSizes={false} showDensities={false}>
  {#snippet children()}
    <div class="poodle-model-connection-card-specimen">
      <SpecimenGroup label="Ready and enabled">
        <p class="poodle-model-connection-card-specimen__note">
          Two configured connections of one provider. They differ only by instance
          label and opaque id; the second is switched off by host preference, not
          by readiness.
        </p>
        <div class="poodle-model-connection-card-specimen__stack">
          <ModelConnectionCard {...cardProps(work)} />
          <ModelConnectionCard {...cardProps(personal)} />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Readiness and preference states">
        <div class="poodle-model-connection-card-specimen__stack">
          <ModelConnectionCard {...cardProps(codex)} />
          <ModelConnectionCard {...cardProps(anthropic)} />
          <ModelConnectionCard {...cardProps(ollama)} />
          <!-- The whole card is inert; readiness copy stays readable. -->
          <ModelConnectionCard {...cardProps(work)} isDisabled />
          <!-- Only the enable Switch is locked; the card still opens. -->
          <ModelConnectionCard {...cardProps(codex)} isEnableDisabled />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Host mark, badges, actions, and closed accessory">
        <div class="poodle-model-connection-card-specimen__stack">
          <ModelConnectionCard {...cardProps(work)}>
            {#snippet leading()}
              <Icon name="star" />
            {/snippet}
            {#snippet badges()}
              <Pill tone="info" appearance="subtle">Preview</Pill>
            {/snippet}
            {#snippet actions()}
              <IconButton
                icon="ellipsis"
                variant="ghost"
                ariaLabel="More actions for OpenAI · Work"
              />
            {/snippet}
          </ModelConnectionCard>
          <ModelConnectionCard {...cardProps(work)}>
            {#snippet closedAccessory()}
              <UpdateCenter presence="attention" {...updateOffer} />
            {/snippet}
          </ModelConnectionCard>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Open details with catalogue">
        <ModelConnectionCard {...cardProps(work)} defaultOpen>
          {#snippet details()}
            <ModelCatalogueEditor items={MODEL_CATALOGUE_FIXTURES} />
          {/snippet}
        </ModelConnectionCard>
      </SpecimenGroup>

      <SpecimenGroup label="Narrow summary wrapping">
        <div class="poodle-model-connection-card-specimen__narrow">
          <ModelConnectionCard {...cardProps(anthropic)} />
        </div>
      </SpecimenGroup>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-model-connection-card-specimen {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .poodle-model-connection-card-specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .poodle-model-connection-card-specimen__narrow {
    width: min(18rem, 100%);
  }

  .poodle-model-connection-card-specimen__note {
    margin: 0 0 0.75rem;
    font-size: 0.875rem;
    opacity: 0.75;
  }
</style>
