<script lang="ts">
  import { Progress } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout showSizes={true} showDensities={false}>
  {#snippet children()}
    <div class="poodle-specimen">
      <SpecimenGroup label="Determinate">
        <Progress value={0} ariaLabel="Empty progress" />
        <Progress value={35} ariaLabel="35% progress" />
        <Progress value={72} ariaLabel="72% progress" />
        <Progress value={100} ariaLabel="Complete progress" />
      </SpecimenGroup>

      <SpecimenGroup label="Indeterminate">
        <Progress indeterminate ariaLabel="Loading" />
      </SpecimenGroup>

      <SpecimenGroup label="Custom max">
        <Progress value={3} max={5} ariaLabel="3 of 5 steps complete" valueText="3 of 5 steps" />
      </SpecimenGroup>
    </div>
  {/snippet}

  {#snippet sizes(size)}
    <div class="poodle-progress-specimen__variant">
      <Progress value={60} ariaLabel={`Progress at ${size}`} {size} />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-progress-specimen__variant {
    width: min(100%, 20rem);
  }
</style>
