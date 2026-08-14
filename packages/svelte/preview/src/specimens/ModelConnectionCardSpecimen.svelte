<script lang="ts">
  import {
    ModelCatalogueEditor,
    ModelConnectionCard,
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
</script>

<SpecimenLayout showSizes={false} showDensities={false}>
  {#snippet children()}
    <div class="poodle-model-connection-card-specimen">
      <SpecimenGroup label="Ready and enabled">
        <ModelConnectionCard
          id={work.id}
          title={work.title}
          providerLabel={work.providerLabel}
          routeLabel={work.routeLabel}
          version={work.version}
          accessSummary={work.accessSummary}
          readiness={work.readiness}
          readinessLabel={work.readinessLabel}
          isEnabled={work.enabled}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Ready and disabled">
        <ModelConnectionCard
          id={personal.id}
          title={personal.title}
          providerLabel={personal.providerLabel}
          routeLabel={personal.routeLabel}
          version={personal.version}
          accessSummary={personal.accessSummary}
          readiness={personal.readiness}
          readinessLabel={personal.readinessLabel}
          isEnabled={personal.enabled}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Checking">
        <ModelConnectionCard
          id={codex.id}
          title={codex.title}
          providerLabel={codex.providerLabel}
          routeLabel={codex.routeLabel}
          version={codex.version}
          accessSummary={codex.accessSummary}
          readiness={codex.readiness}
          readinessLabel={codex.readinessLabel}
          isEnabled={codex.enabled}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Needs attention">
        <ModelConnectionCard
          id={anthropic.id}
          title={anthropic.title}
          providerLabel={anthropic.providerLabel}
          routeLabel={anthropic.routeLabel}
          version={anthropic.version}
          accessSummary={anthropic.accessSummary}
          readiness={anthropic.readiness}
          readinessLabel={anthropic.readinessLabel}
          isEnabled={anthropic.enabled}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Unavailable">
        <ModelConnectionCard
          id={ollama.id}
          title={ollama.title}
          providerLabel={ollama.providerLabel}
          routeLabel={ollama.routeLabel}
          version={ollama.version}
          accessSummary={ollama.accessSummary}
          readiness={ollama.readiness}
          readinessLabel={ollama.readinessLabel}
          isEnabled={ollama.enabled}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Two OpenAI instances">
        <div class="poodle-model-connection-card-specimen__stack">
          <ModelConnectionCard
            id={work.id}
            title={work.title}
            providerLabel={work.providerLabel}
            routeLabel={work.routeLabel}
            version={work.version}
            accessSummary={work.accessSummary}
            readiness={work.readiness}
            readinessLabel={work.readinessLabel}
            isEnabled={work.enabled}
          />
          <ModelConnectionCard
            id={personal.id}
            title={personal.title}
            providerLabel={personal.providerLabel}
            routeLabel={personal.routeLabel}
            version={personal.version}
            accessSummary={personal.accessSummary}
            readiness={personal.readiness}
            readinessLabel={personal.readinessLabel}
            isEnabled={personal.enabled}
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Closed UpdateCenter accessory">
        <ModelConnectionCard
          id={work.id}
          title={work.title}
          providerLabel={work.providerLabel}
          routeLabel={work.routeLabel}
          version={work.version}
          accessSummary={work.accessSummary}
          readiness={work.readiness}
          readinessLabel={work.readinessLabel}
          isEnabled={work.enabled}
        >
          {#snippet closedAccessory()}
            <UpdateCenter presence="attention" {...updateOffer} />
          {/snippet}
        </ModelConnectionCard>
      </SpecimenGroup>

      <SpecimenGroup label="Open details with catalogue">
        <ModelConnectionCard
          id={work.id}
          title={work.title}
          providerLabel={work.providerLabel}
          routeLabel={work.routeLabel}
          version={work.version}
          accessSummary={work.accessSummary}
          readiness={work.readiness}
          readinessLabel={work.readinessLabel}
          isEnabled={work.enabled}
          defaultOpen
        >
          {#snippet details()}
            <ModelCatalogueEditor items={MODEL_CATALOGUE_FIXTURES} />
          {/snippet}
        </ModelConnectionCard>
      </SpecimenGroup>

      <SpecimenGroup label="Narrow summary wrapping">
        <div class="poodle-model-connection-card-specimen__narrow">
          <ModelConnectionCard
            id={anthropic.id}
            title={anthropic.title}
            providerLabel={anthropic.providerLabel}
            routeLabel={anthropic.routeLabel}
            version={anthropic.version}
            accessSummary={anthropic.accessSummary}
            readiness={anthropic.readiness}
            readinessLabel={anthropic.readinessLabel}
            isEnabled={anthropic.enabled}
          />
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
</style>
