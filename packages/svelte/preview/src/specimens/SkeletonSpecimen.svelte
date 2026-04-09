<script lang="ts">
  import { Skeleton } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
  <SpecimenGroup label="Basic shapes">
    <div class="specimen__row">
      <Skeleton shape="line" width="12rem" />
      <Skeleton shape="circle" width="2.5rem" height="2.5rem" />
      <Skeleton shape="block" width="8rem" height="3rem" />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Preset: avatar-line">
    <Skeleton preset="avatar-line" />
  </SpecimenGroup>

  <SpecimenGroup label="Preset: list-item (×3)">
    <Skeleton preset="list-item" />
    <Skeleton preset="list-item" />
    <Skeleton preset="list-item" />
  </SpecimenGroup>

  <SpecimenGroup label="Preset: table-row (×3)">
    <Skeleton preset="table-row" />
    <Skeleton preset="table-row" />
    <Skeleton preset="table-row" />
  </SpecimenGroup>

  <SpecimenGroup label="Preset: card">
    <div class="specimen__card-grid">
      <Skeleton preset="card" />
      <Skeleton preset="card" />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Preset: detail-section">
    <Skeleton preset="detail-section" lines={4} />
  </SpecimenGroup>

  <SpecimenGroup label="Static (no animation)">
    <Skeleton shape="line" width="10rem" animated={false} />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
  }

  .specimen__card-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
  }
</style>
