<script lang="ts">
  import { DetailShell, DetailSection, PageHeader } from "@poodle/svelte-composites";
  import { Button, Pill, DetailRow, Eyebrow, Region, Separator, Surface } from "@poodle/svelte-primitives";
</script>

<div class="specimen">
  <Surface tone="panel" border="subtle" padding="md">
    <Eyebrow>Layout structure</Eyebrow>
    <div class="specimen__demo">
      <DetailShell ariaLabel="Layout regions">
        <svelte:fragment slot="header">
          <Region label="Header" color="#5b9bd5" minHeight="3rem" />
        </svelte:fragment>
        <Region label="Section 1" color="#70ad47" minHeight="3rem" />
        <Region label="Section 2" color="#ed7d31" minHeight="3rem" />
        <Region label="Section 3" color="#a855f7" minHeight="3rem" />
      </DetailShell>
    </div>
  </Surface>

  <Surface tone="panel" border="subtle" padding="md">
    <Eyebrow>Multi-section layout with header</Eyebrow>
    <div class="specimen__demo">
      <DetailShell ariaLabel="Project detail view">
        <svelte:fragment slot="header">
          <PageHeader title="Poodle Design System" eyebrow="Project" subtitle="A comprehensive component library.">
            <svelte:fragment slot="actions">
              <Pill appearance="badge" tone="success">Active</Pill>
              <Button variant="secondary">Edit</Button>
            </svelte:fragment>
          </PageHeader>
        </svelte:fragment>
        <DetailSection title="General">
          <DetailRow label="Owner" value="Clay" />
          <DetailRow label="Created" value="March 2025" />
          <DetailRow label="Repository" value="github.com/poodle-ui/poodle" />
        </DetailSection>
        <Separator />
        <DetailSection title="Configuration">
          <svelte:fragment slot="actions">
            <Button variant="ghost">Reset</Button>
          </svelte:fragment>
          <DetailRow label="Theme" value="Dark" />
          <DetailRow label="Density" value="Compact" />
          <DetailRow label="Default size" value="Medium" />
        </DetailSection>
        <Separator />
        <DetailSection title="Integrations">
          <DetailRow label="Figma" value="Connected" />
          <DetailRow label="Storybook" value="Not configured" />
        </DetailSection>
      </DetailShell>
    </div>
  </Surface>

  <Surface tone="panel" border="subtle" padding="md">
    <Eyebrow>Loading state</Eyebrow>
    <div class="specimen__demo">
      <DetailShell title="Loading" state="loading" ariaLabel="Loading view" />
    </div>
  </Surface>

  <Surface tone="panel" border="subtle" padding="md">
    <Eyebrow>Error state</Eyebrow>
    <div class="specimen__demo">
      <DetailShell
        title="Error"
        state="error"
        stateTitle="Failed to load"
        stateMessage="Something went wrong. Please try again."
        ariaLabel="Error view"
      />
    </div>
  </Surface>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__demo {
    margin-top: 0.75rem;
  }
</style>
