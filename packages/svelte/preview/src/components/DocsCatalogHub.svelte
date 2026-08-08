<script lang="ts">
  import { PageHeader } from "@inflatable-cookie/poodle-svelte";
  import { Accordion, Button, Card, Collapsible, Eyebrow, Stack, Pill } from "@inflatable-cookie/poodle-svelte";

  import type { DocsFamilyEntry, DocsSectionEntry } from "../catalog";
  let {
    docsFamilies = [],
    docsAdoptionChecklist = [],
    catalogEntries = [],
    onSelectSection = () => {},
    brandProofCards = [],
  }: {
    docsFamilies?: DocsFamilyEntry[];
    docsAdoptionChecklist?: string[];
    catalogEntries?: DocsSectionEntry[];
    onSelectSection?: (sectionId: string) => void;
    brandProofCards?: Array<{
      id: string;
      eyebrow: string;
      title: string;
      summary: string;
      variant: "elevated" | "outlined";
    }>;
  } = $props();
  const catalogEntryMap = Object.fromEntries(catalogEntries.map((entry) => [entry.id, entry]));
</script>

<section id="catalog-hub" class="poodle-panel poodle-token-summary" aria-labelledby="catalog-heading">
  <div class="poodle-section-header">
    <div>
      <Eyebrow>Catalog hub</Eyebrow>
      <h2 id="catalog-heading">Information architecture, coverage, and adoption bar</h2>
    </div>
  </div>
  <div class="poodle-docs-overview-stack">
    <div class="poodle-docs-overview-top">
      <article class="poodle-demo-card poodle-docs-overview-card">
        <div class="poodle-card-header">
          <h3>Family directory</h3>
          <p>
            The docs shell still groups examples by the same package and contract
            layers adopters actually consume.
          </p>
        </div>
        <div class="poodle-docs-family-grid">
          {#each docsFamilies as family}
            <article class="poodle-docs-family-card">
              <div class="poodle-docs-family-card__header">
                <div>
                  <Eyebrow>{family.eyebrow}</Eyebrow>
                  <h4>{family.title}</h4>
                </div>
                <span class="poodle-command-shortcut-hint">{family.packageName}</span>
              </div>
              <p class="poodle-detail-card-meta">{family.summary}</p>
              <div class="poodle-docs-family-meta">
                <span class="poodle-token-path">{family.contractRoot}</span>
                <strong>{family.adoptionBar}</strong>
              </div>
              <div class="poodle-docs-link-row">
                {#each family.sectionIds as sectionId}
                  {#if catalogEntryMap[sectionId]}
                    <Button
                      className="docs-link-chip"
                      variant="ghost"
                      size="sm"
                      onclick={() => onSelectSection(sectionId)}
                    >
                      {catalogEntryMap[sectionId].title}
                    </Button>
                  {/if}
                {/each}
              </div>
            </article>
          {/each}
        </div>
      </article>

      <article class="poodle-demo-card poodle-docs-overview-card poodle-docs-overview-card--narrow">
        <div class="poodle-card-header">
          <h3>Adoption-ready minimum</h3>
          <p>
            The docs shell still freezes what must be visible before wider rollout,
            not just what exists somewhere in the repo.
          </p>
        </div>
        <div class="poodle-behavior-list">
          {#each docsAdoptionChecklist as item}
            <div class="poodle-behavior-item">
              <strong>Required</strong>
              <p>{item}</p>
            </div>
          {/each}
        </div>
      </article>

      <article class="poodle-demo-card poodle-docs-overview-card poodle-docs-overview-card--narrow">
        <div class="poodle-card-header">
          <h3>Disclosure primitives</h3>
          <p>
            Catalog chrome now proves grouped and single-block disclosure through
            real foundation primitives.
          </p>
        </div>
        <div class="poodle-demo-stack">
          <Accordion
            items={[
              {
                value: "accordion-foundation",
                label: "Accordion",
                description: "Grouped disclosure for repeated docs or settings sections.",
              },
              {
                value: "accordion-boundary",
                label: "Boundary",
                description: "Single-block reveal belongs to Collapsible rather than grouped disclosure.",
              },
            ]}
            defaultValue="accordion-foundation"
            ariaLabel="Disclosure primitive example"
          >
            {#snippet children(item)}
            <p class="poodle-detail-card-meta">
              {item.value === "accordion-foundation"
                ? "Foundation-safe grouped disclosure now exists for more web-oriented products and docs surfaces."
                : "Single-block reveal belongs to Collapsible; grouped disclosure belongs to Accordion."}
            </p>
            {/snippet}
          </Accordion>

          <Collapsible
            title="Collapsible"
            description="Single revealable content block for compact notes, diagnostics, or settings groups."
            defaultOpen={true}
          >
            <p class="poodle-detail-card-meta">
              This surface owns one trigger and one revealable region without
              pretending to be grouped navigation.
            </p>
          </Collapsible>
        </div>
      </article>
    </div>

    <article class="poodle-demo-card poodle-docs-overview-card">
      <div class="poodle-card-header">
        <h3>Example directory</h3>
        <p>
          The docs shell can still launch every section directly, but the shared
          demo target now sits behind a cleaner screen model.
        </p>
      </div>
      <div class="poodle-docs-section-list">
        {#each catalogEntries as entry}
          <Button
            className="docs-section-card"
            variant="ghost"
            ariaLabel={`Open ${entry.title}`}
            onclick={() => onSelectSection(entry.id)}
          >
            <div>
              <Eyebrow>{entry.eyebrow}</Eyebrow>
              <strong>{entry.title}</strong>
            </div>
            <p>{entry.summary}</p>
            <div class="poodle-docs-section-meta">
              <span class="poodle-token-path">{entry.contractRoot}</span>
              <span class="poodle-command-shortcut-hint">{entry.packageName}</span>
            </div>
            <div class="poodle-docs-tag-row">
              {#each entry.exampleTypes as exampleType}
                <Pill appearance="subtle">{exampleType}</Pill>
              {/each}
            </div>
          </Button>
        {/each}
      </div>
    </article>

    <article class="poodle-demo-card poodle-docs-overview-card">
      <div class="poodle-card-header">
        <h3>Scoped brand proof</h3>
        <p>
          Brand expression still belongs to app-owned wrappers and scoped recipes,
          not the default system chrome.
        </p>
      </div>
      <div class="poodle-brand-proof-scope">
        <PageHeader
          title="Make room for brand styling without rebuilding the system"
          eyebrow="Website-style wrapper"
          subtitle="This proof uses app-owned composition plus scoped appearance recipes so cards, header framing, and CTA chrome can shift together."
        >
          {#snippet actions()}
<Stack direction="row" justify="end" wrap gap="md">
            <Button variant="secondary">Read pattern notes</Button>
            <Button variant="primary">Launch branded preview</Button>
          </Stack>
{/snippet}
        </PageHeader>

        <div class="poodle-brand-proof-grid">
          {#each brandProofCards as card}
            <Card variant={card.variant}>
              {#snippet header()}
                <Eyebrow>{card.eyebrow}</Eyebrow>
              {/snippet}
              <strong class="poodle-detail-card-value">{card.title}</strong>
              <p class="poodle-detail-card-meta">{card.summary}</p>
            </Card>
          {/each}
        </div>
      </div>
    </article>
  </div>
</section>
