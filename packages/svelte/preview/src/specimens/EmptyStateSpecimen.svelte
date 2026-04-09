<script lang="ts">
  import { EmptyState } from "@poodle/svelte-composites";
  import { Button, Icon } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
  <SpecimenGroup label="Neutral">
    <EmptyState title="No projects yet" message="Create your first project to get started.">
      <svelte:fragment slot="actions">
        <Button>Create project</Button>
      </svelte:fragment>
    </EmptyState>
  </SpecimenGroup>

  <SpecimenGroup label="Search">
    <EmptyState
      variant="search"
      title="No results found"
      message="Try adjusting your search terms or clearing filters."
    >
      <svelte:fragment slot="actions">
        <Button variant="secondary">Clear filters</Button>
      </svelte:fragment>
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
      <svelte:fragment slot="visual">
        <Icon name="mail" />
      </svelte:fragment>
    </EmptyState>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

</style>
