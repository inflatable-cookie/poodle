<script lang="ts">
  import { DetailShell, DetailSection, DetailRow, PageHeader } from "@pug/svelte-composites";
  import { Eyebrow, Button } from "@pug/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Ready state</Eyebrow>
    <div class="shell-container">
      <DetailShell title="Project detail" ariaLabel="Project detail view">
        <svelte:fragment slot="header">
          <PageHeader title="Pug Design System" eyebrow="Project" subtitle="A comprehensive component library." />
        </svelte:fragment>
        <DetailSection title="Overview">
          <DetailRow label="Status" value="Active" />
          <DetailRow label="Owner" value="Clay" />
          <DetailRow label="Created" value="March 2025" />
        </DetailSection>
      </DetailShell>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Loading state</Eyebrow>
    <div class="shell-container">
      <DetailShell title="Loading" state="loading" ariaLabel="Loading view" />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Error state</Eyebrow>
    <div class="shell-container">
      <DetailShell
        title="Error"
        state="error"
        stateTitle="Failed to load"
        stateMessage="Something went wrong. Please try again."
        ariaLabel="Error view"
      />
    </div>
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .shell-container {
    height: 14rem;
    border: 1px solid var(--pug-color-border-default);
    border-radius: 4px;
    overflow: hidden;
  }
</style>
