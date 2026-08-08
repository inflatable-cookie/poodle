<script lang="ts">
  import { PaginationSummary } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Default">
    <PaginationSummary currentPage={1} totalPages={8} totalItems={156} pageSize={20} />
  </SpecimenGroup>

  <SpecimenGroup label="Single page">
    <PaginationSummary currentPage={1} totalPages={1} totalItems={12} pageSize={20} />
  </SpecimenGroup>

  <SpecimenGroup label="Large dataset">
    <PaginationSummary currentPage={5} totalPages={50} totalItems={1000} pageSize={20} />
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

</style>
