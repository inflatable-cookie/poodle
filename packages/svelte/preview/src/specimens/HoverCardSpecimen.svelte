<script lang="ts">
  import { HoverCard } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
  <SpecimenGroup label="Default (top placement)">
    <HoverCard ariaLabel="User preview">
      <span slot="trigger" class="trigger-link">@clay</span>
      <div class="card-content">
        <strong>Clay</strong>
        <p>Design systems engineer working on Poodle. Loves component architecture and accessibility.</p>
      </div>
    </HoverCard>
  </SpecimenGroup>

  <SpecimenGroup label="Bottom placement">
    <HoverCard placement="bottom" ariaLabel="Repository info">
      <span slot="trigger" class="trigger-link">poodle/svelte-primitives</span>
      <div class="card-content">
        <strong>svelte-primitives</strong>
        <p>Core primitive components for the Poodle design system. 64 components, 94% coverage.</p>
      </div>
    </HoverCard>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .trigger-link {
    color: var(--poodle-color-text-accent);
    text-decoration: underline;
    cursor: pointer;
    font-size: 0.875rem;
  }

  .card-content {
    padding: 0.5rem;
    max-width: 16rem;
  }

  .card-content strong {
    display: block;
    margin-bottom: 0.25rem;
  }

  .card-content p {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
