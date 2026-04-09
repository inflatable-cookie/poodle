<script lang="ts">
  import { Progress } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
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

  <SpecimenGroup label="Sizes">
    <Progress value={60} size="sm" ariaLabel="Small progress" />
    <Progress value={60} size="md" ariaLabel="Medium progress" />
    <Progress value={60} size="lg" ariaLabel="Large progress" />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

</style>
