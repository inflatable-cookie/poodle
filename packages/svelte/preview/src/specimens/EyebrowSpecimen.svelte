<script lang="ts">
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
  <SpecimenGroup label="Section label">
    <p>Eyebrow renders small uppercase text used for categorizing content above headings.</p>
  </SpecimenGroup>

  <SpecimenGroup label="Primitive">
    <h3 class="demo-heading">Button</h3>
    <p>Primary interactive control for triggering actions.</p>
  </SpecimenGroup>

  <SpecimenGroup label="Composite">
    <h3 class="demo-heading">DataTable</h3>
    <p>Feature-rich table with sorting, selection, and pagination.</p>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .demo-heading {
    margin: 0;
    font-size: 1.25rem;
    color: var(--poodle-color-text-primary);
  }
</style>
