<script lang="ts">
  import { Separator } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
  <SpecimenGroup label="Horizontal (default)">
    <p>Content above</p>
    <Separator />
    <p>Content below</p>
  </SpecimenGroup>

  <SpecimenGroup label="Vertical">
    <div class="specimen__row">
      <span>Left</span>
      <Separator orientation="vertical" />
      <span>Center</span>
      <Separator orientation="vertical" />
      <span>Right</span>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Decorative">
    <Separator decorative />
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
    gap: 0.75rem;
    align-items: center;
    height: 2rem;
  }
</style>
