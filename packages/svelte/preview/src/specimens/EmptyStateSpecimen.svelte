<script lang="ts">
  import { EmptyState } from "@inflatable-cookie/poodle-svelte";
  import { Button, Icon } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Neutral">
    <EmptyState title="No projects yet" message="Create your first project to get started.">
      {#snippet actions()}
        <Button>Create project</Button>
      {/snippet}
    </EmptyState>
  </SpecimenGroup>

  <SpecimenGroup label="Search">
    <EmptyState
      variant="search"
      title="No results found"
      message="Try adjusting your search terms or clearing filters."
    >
      {#snippet actions()}
        <Button variant="secondary">Clear filters</Button>
      {/snippet}
    </EmptyState>
  </SpecimenGroup>

  <SpecimenGroup label="First run">
    <EmptyState
      variant="firstRun"
      title="Welcome to your workspace"
      message="This is where your team's components will appear once you start building."
    />
  </SpecimenGroup>

  <SpecimenGroup label="Compact custom visual">
    <EmptyState
      size="compact"
      title="No captured emails found"
      message="Emails will appear here when sent in development mode."
    >
      {#snippet visual()}
        <Icon name="mail" />
      {/snippet}
    </EmptyState>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

</style>
