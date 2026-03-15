<script lang="ts">
  import { Skeleton, Eyebrow } from "@pug/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Basic shapes</Eyebrow>
    <div class="specimen__row">
      <Skeleton shape="line" width="12rem" />
      <Skeleton shape="circle" width="2.5rem" height="2.5rem" />
      <Skeleton shape="block" width="8rem" height="3rem" />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Preset: avatar-line</Eyebrow>
    <Skeleton preset="avatar-line" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Preset: list-item (×3)</Eyebrow>
    <Skeleton preset="list-item" />
    <Skeleton preset="list-item" />
    <Skeleton preset="list-item" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Preset: table-row (×3)</Eyebrow>
    <Skeleton preset="table-row" />
    <Skeleton preset="table-row" />
    <Skeleton preset="table-row" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Preset: card</Eyebrow>
    <div class="specimen__card-grid">
      <Skeleton preset="card" />
      <Skeleton preset="card" />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Preset: detail-section</Eyebrow>
    <Skeleton preset="detail-section" lines={4} />
  </div>

  <div class="specimen__group">
    <Eyebrow>Static (no animation)</Eyebrow>
    <Skeleton shape="line" width="10rem" isAnimated={false} />
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
    gap: 0.75rem;
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
