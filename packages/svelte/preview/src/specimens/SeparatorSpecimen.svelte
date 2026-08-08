<script lang="ts">
  import { Separator } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Horizontal (default)">
    <p>Content above</p>
    <Separator />
    <p>Content below</p>
  </SpecimenGroup>

  <SpecimenGroup label="Vertical">
    <div class="poodle-specimen__row">
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
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__row {
    display: flex;
    gap: 0.75rem;
    align-items: center;
    height: 2rem;
  }
</style>
