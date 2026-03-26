<script lang="ts">
  import { Eyebrow, PaginationSummary } from "@poodle/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <PaginationSummary currentPage={1} totalPages={8} totalItems={156} pageSize={20} />
  </div>

  <div class="specimen__group">
    <Eyebrow>Single page</Eyebrow>
    <PaginationSummary currentPage={1} totalPages={1} totalItems={12} pageSize={20} />
  </div>

  <div class="specimen__group">
    <Eyebrow>Large dataset</Eyebrow>
    <PaginationSummary currentPage={5} totalPages={50} totalItems={1000} pageSize={20} />
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
</style>
