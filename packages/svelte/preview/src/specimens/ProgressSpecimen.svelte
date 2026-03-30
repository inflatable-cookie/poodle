<script lang="ts">
  import { Progress, Eyebrow } from "@poodle/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Determinate</Eyebrow>
    <Progress value={0} ariaLabel="Empty progress" />
    <Progress value={35} ariaLabel="35% progress" />
    <Progress value={72} ariaLabel="72% progress" />
    <Progress value={100} ariaLabel="Complete progress" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Indeterminate</Eyebrow>
    <Progress indeterminate ariaLabel="Loading" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Custom max</Eyebrow>
    <Progress value={3} max={5} ariaLabel="3 of 5 steps complete" valueText="3 of 5 steps" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <Progress value={60} size="sm" ariaLabel="Small progress" />
    <Progress value={60} size="md" ariaLabel="Medium progress" />
    <Progress value={60} size="lg" ariaLabel="Large progress" />
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
    gap: 0.75rem;
  }
</style>
