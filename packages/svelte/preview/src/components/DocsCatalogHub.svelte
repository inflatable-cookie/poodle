<script lang="ts">
  import { PageHeader } from "@pug/svelte-composites";
  import { Accordion, Button, Card, Collapsible, Eyebrow, Stack, Pill, Toggle } from "@pug/svelte-primitives";

  import type { DocsFamilyEntry, DocsSectionEntry } from "../catalog";

  export let docsFamilies: DocsFamilyEntry[] = [];
  export let docsAdoptionChecklist: string[] = [];
  export let catalogEntries: DocsSectionEntry[] = [];
  export let brandProofCards: Array<{
    id: string;
    eyebrow: string;
    title: string;
    summary: string;
    variant: "elevated" | "outlined";
  }> = [];
  export let onSelectSection: (sectionId: string) => void = () => {};

  const catalogEntryMap = Object.fromEntries(catalogEntries.map((entry) => [entry.id, entry]));
</script>

<section id="catalog-hub" class="panel token-summary" aria-labelledby="catalog-heading">
  <div class="section-header">
    <div>
      <Eyebrow>Catalog hub</Eyebrow>
      <h2 id="catalog-heading">Information architecture, coverage, and adoption bar</h2>
    </div>
  </div>
  <div class="docs-overview-stack">
    <div class="docs-overview-top">
      <article class="demo-card docs-overview-card">
        <div class="card-header">
          <h3>Family directory</h3>
          <p>
            The docs shell still groups examples by the same package and contract
            layers adopters actually consume.
          </p>
        </div>
        <div class="docs-family-grid">
          {#each docsFamilies as family}
            <article class="docs-family-card">
              <div class="docs-family-card__header">
                <div>
                  <Eyebrow>{family.eyebrow}</Eyebrow>
                  <h4>{family.title}</h4>
                </div>
                <span class="command-shortcut-hint">{family.packageName}</span>
              </div>
              <p class="detail-card-meta">{family.summary}</p>
              <div class="docs-family-meta">
                <span class="token-path">{family.contractRoot}</span>
                <strong>{family.adoptionBar}</strong>
              </div>
              <div class="docs-link-row">
                {#each family.sectionIds as sectionId}
                  {#if catalogEntryMap[sectionId]}
                    <Button
                      className="docs-link-chip"
                      variant="ghost"
                      size="sm"
                      on:click={() => onSelectSection(sectionId)}
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

      <article class="demo-card docs-overview-card docs-overview-card--narrow">
        <div class="card-header">
          <h3>Adoption-ready minimum</h3>
          <p>
            The docs shell still freezes what must be visible before wider rollout,
            not just what exists somewhere in the repo.
          </p>
        </div>
        <div class="behavior-list">
          {#each docsAdoptionChecklist as item}
            <div class="behavior-item">
              <strong>Required</strong>
              <p>{item}</p>
            </div>
          {/each}
        </div>
      </article>

      <article class="demo-card docs-overview-card docs-overview-card--narrow">
        <div class="card-header">
          <h3>Disclosure primitives</h3>
          <p>
            Catalog chrome now proves grouped and single-block disclosure through
            real foundation primitives.
          </p>
        </div>
        <div class="demo-stack">
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
            let:item
          >
            <p class="detail-card-meta">
              {item.value === "accordion-foundation"
                ? "Foundation-safe grouped disclosure now exists for more web-oriented products and docs surfaces."
                : "Single-block reveal belongs to Collapsible; grouped disclosure belongs to Accordion."}
            </p>
          </Accordion>

          <Collapsible
            title="Collapsible"
            description="Single revealable content block for compact notes, diagnostics, or settings groups."
            defaultOpen={true}
          >
            <p class="detail-card-meta">
              This surface owns one trigger and one revealable region without
              pretending to be grouped navigation.
            </p>
          </Collapsible>
        </div>
      </article>
    </div>

    <article class="demo-card docs-overview-card">
      <div class="card-header">
        <h3>Example directory</h3>
        <p>
          The docs shell can still launch every section directly, but the shared
          demo target now sits behind a cleaner screen model.
        </p>
      </div>
      <div class="docs-section-list">
        {#each catalogEntries as entry}
          <Toggle
            className="docs-section-card"
            isPressed={false}
            layout="stack"
            variant="ghost"
            ariaLabel={`Open ${entry.title}`}
            on:pressedChange={() => onSelectSection(entry.id)}
          >
            <div>
              <Eyebrow>{entry.eyebrow}</Eyebrow>
              <strong>{entry.title}</strong>
            </div>
            <p>{entry.summary}</p>
            <div class="docs-section-meta">
              <span class="token-path">{entry.contractRoot}</span>
              <span class="command-shortcut-hint">{entry.packageName}</span>
            </div>
            <div class="docs-tag-row">
              {#each entry.exampleTypes as exampleType}
                <Pill appearance="subtle">{exampleType}</Pill>
              {/each}
            </div>
          </Toggle>
        {/each}
      </div>
    </article>

    <article class="demo-card docs-overview-card">
      <div class="card-header">
        <h3>Scoped brand proof</h3>
        <p>
          Brand expression still belongs to app-owned wrappers and scoped recipes,
          not the default system chrome.
        </p>
      </div>
      <div class="brand-proof-scope">
        <PageHeader
          title="Make room for brand styling without rebuilding the system"
          eyebrow="Website-style wrapper"
          subtitle="This proof uses app-owned composition plus scoped appearance recipes so cards, header framing, and CTA chrome can shift together."
        >
          <Stack slot="actions" direction="row" justify="end" wrap gap="md">
            <Button variant="secondary">Read pattern notes</Button>
            <Button variant="primary">Launch branded preview</Button>
          </Stack>
        </PageHeader>

        <div class="brand-proof-grid">
          {#each brandProofCards as card}
            <Card variant={card.variant}>
              <div slot="header">
                <Eyebrow>{card.eyebrow}</Eyebrow>
              </div>
              <strong class="detail-card-value">{card.title}</strong>
              <p class="detail-card-meta">{card.summary}</p>
            </Card>
          {/each}
        </div>
      </div>
    </article>
  </div>
</section>
