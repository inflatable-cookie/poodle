<script lang="ts">
  import { HoverCard } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Default (top placement)">
    <HoverCard ariaLabel="User preview">
      <span slot="trigger" class="poodle-trigger-link">@clay</span>
      <div class="poodle-card-content">
        <strong>Clay</strong>
        <p>Design systems engineer working on Poodle. Loves component architecture and accessibility.</p>
      </div>
    </HoverCard>
  </SpecimenGroup>

  <SpecimenGroup label="Bottom placement">
    <HoverCard placement="bottom" ariaLabel="Repository info">
      <span slot="trigger" class="poodle-trigger-link">poodle/svelte-primitives</span>
      <div class="poodle-card-content">
        <strong>svelte-primitives</strong>
        <p>Core primitive components for the Poodle design system. 64 components, 94% coverage.</p>
      </div>
    </HoverCard>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .poodle-trigger-link {
    color: var(--poodle-color-text-accent);
    text-decoration: underline;
    cursor: pointer;
    font-size: 0.875rem;
  }

  .poodle-card-content {
    padding: 0.5rem;
    max-width: 16rem;
  }

  .poodle-card-content strong {
    display: block;
    margin-bottom: 0.25rem;
  }

  .poodle-card-content p {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
