<script lang="ts">
  import { DetailShell, DetailSection, PageHeader } from "@poodle/svelte";
  import { Button, Pill, DetailItem, Region, Separator } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Layout structure">
    <div class="poodle-specimen__demo">
      <DetailShell ariaLabel="Layout regions">
        {#snippet header()}
          <Region label="Header" color="#5b9bd5" minHeight="3rem" />
        {/snippet}
        <Region label="Section 1" color="#70ad47" minHeight="3rem" />
        <Region label="Section 2" color="#ed7d31" minHeight="3rem" />
        <Region label="Section 3" color="#a855f7" minHeight="3rem" />
      </DetailShell>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Multi-section layout with header">
    <div class="poodle-specimen__demo">
      <DetailShell ariaLabel="Project detail view">
        {#snippet header()}
          <PageHeader title="Poodle Design System" eyebrow="Project" subtitle="A comprehensive component library.">
            {#snippet actions()}
              <Pill appearance="badge" tone="success">Active</Pill>
              <Button variant="secondary">Edit</Button>
            {/snippet}
          </PageHeader>
        {/snippet}
        <DetailSection title="General">
          <DetailItem label="Owner" value="Clay" />
          <DetailItem label="Created" value="March 2025" />
          <DetailItem label="Repository" value="github.com/poodle-ui/poodle" />
        </DetailSection>
        <Separator />
        <DetailSection title="Configuration">
          {#snippet actions()}
            <Button variant="ghost">Reset</Button>
          {/snippet}
          <DetailItem label="Theme" value="Dark" />
          <DetailItem label="Density" value="Compact" />
          <DetailItem label="Default size" value="Medium" />
        </DetailSection>
        <Separator />
        <DetailSection title="Integrations">
          <DetailItem label="Figma" value="Connected" />
          <DetailItem label="Storybook" value="Not configured" />
        </DetailSection>
      </DetailShell>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Loading state">
    <div class="poodle-specimen__demo">
      <DetailShell title="Loading" state="loading" ariaLabel="Loading view" />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Error state">
    <div class="poodle-specimen__demo">
      <DetailShell
        title="Error"
        state="error"
        stateTitle="Failed to load"
        stateMessage="Something went wrong. Please try again."
        ariaLabel="Error view"
      />
    </div>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__demo {
    margin-top: 0.75rem;
  }
</style>
