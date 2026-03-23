<script lang="ts">
  import { EmptyState } from "@poodle/svelte-composites";
  import { Button, Eyebrow } from "@poodle/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Neutral</Eyebrow>
    <EmptyState title="No projects yet" message="Create your first project to get started.">
      <svelte:fragment slot="actions">
        <Button>Create project</Button>
      </svelte:fragment>
    </EmptyState>
  </div>

  <div class="specimen__group">
    <Eyebrow>Search</Eyebrow>
    <EmptyState
      variant="search"
      title="No results found"
      message="Try adjusting your search terms or clearing filters."
    >
      <svelte:fragment slot="actions">
        <Button variant="secondary">Clear filters</Button>
      </svelte:fragment>
    </EmptyState>
  </div>

  <div class="specimen__group">
    <Eyebrow>First run</Eyebrow>
    <EmptyState
      variant="firstRun"
      title="Welcome to your workspace"
      message="This is where your team's components will appear once you start building."
    />
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
