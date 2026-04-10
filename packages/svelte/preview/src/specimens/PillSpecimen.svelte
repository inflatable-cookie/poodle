<script lang="ts">
  import { Pill } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Tones">
    <div class="specimen__row">
      <Pill tone="neutral" sizeRole="control">Neutral</Pill>
      <Pill tone="info" sizeRole="control">Info</Pill>
      <Pill tone="success" sizeRole="control">Success</Pill>
      <Pill tone="warning" sizeRole="control">Warning</Pill>
      <Pill tone="danger" sizeRole="control">Danger</Pill>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Code font">
    <div class="specimen__row">
      <Pill font="mono" sizeRole="control">v2.4.1</Pill>
      <Pill font="mono" tone="success" sizeRole="control">stable</Pill>
      <Pill font="mono" tone="warning" sizeRole="control">beta</Pill>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Muted">
    <div class="specimen__row">
      <Pill muted sizeRole="control">Muted neutral</Pill>
      <Pill muted tone="success" sizeRole="control">Muted success</Pill>
      <Pill muted tone="danger" sizeRole="control">Muted danger</Pill>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Badge">
    <div class="specimen__row">
      <Pill appearance="badge" sizeRole="control">3</Pill>
      <Pill appearance="badge" sizeRole="control">12</Pill>
      <Pill appearance="badge" sizeRole="control">99+</Pill>
      <Pill appearance="badge" sizeRole="control">New</Pill>
      <Pill appearance="badge" tone="neutral" sizeRole="control">Draft</Pill>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Custom accent">
    <div class="specimen__row">
      <Pill accent="#3b82f6" sizeRole="control">Info-ish</Pill>
      <Pill accent="#22c55e" sizeRole="control">Positive-ish</Pill>
      <Pill accent="#f59e0b" sizeRole="control">Caution-ish</Pill>
      <Pill accent="#ef4444" sizeRole="control">Danger-ish</Pill>
    </div>
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <Pill>{size.toUpperCase()}</Pill>
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <Pill>Label</Pill>
  </svelte:fragment>
</SpecimenLayout>

<style>
  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: center;
  }
</style>
