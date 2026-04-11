<script lang="ts">
  import { Popover, Button } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
  <SpecimenGroup label="Default (bottom-start)">
    <Popover ariaLabel="Quick settings">
      <Button variant="secondary" slot="trigger">Open popover</Button>
      <div class="popover-content">
        <strong>Quick settings</strong>
        <p>Adjust your display preferences or notification settings from this panel.</p>
      </div>
    </Popover>
  </SpecimenGroup>

  <SpecimenGroup label="Top placement">
    <Popover placement="top" ariaLabel="Help tip">
      <Button variant="secondary" slot="trigger">Show help</Button>
      <div class="popover-content">
        <p>Popovers can be anchored to any side of their trigger element.</p>
      </div>
    </Popover>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .popover-content {
    padding: 0.75rem;
    max-width: 16rem;
  }

  .popover-content strong {
    display: block;
    margin-bottom: 0.25rem;
  }

  .popover-content p {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
