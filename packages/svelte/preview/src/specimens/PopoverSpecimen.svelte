<script lang="ts">
  import { Popover, Button, type PopoverTriggerState } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Default (bottom-start)">
    <Popover ariaLabel="Quick settings" triggerIsInteractive>
      {#snippet trigger(state: PopoverTriggerState)}
        <Button
          variant="secondary"
          ariaExpanded={state.expanded}
          controls={state.controls}
          disabled={state.disabled}
        >
          Open popover
        </Button>
      {/snippet}
      <div class="poodle-popover-content">
        <strong>Quick settings</strong>
        <p>Adjust your display preferences or notification settings from this panel.</p>
      </div>
    </Popover>
  </SpecimenGroup>

  <SpecimenGroup label="Top placement">
    <Popover placement="top" ariaLabel="Help tip" triggerIsInteractive>
      {#snippet trigger(state: PopoverTriggerState)}
        <Button
          variant="secondary"
          ariaExpanded={state.expanded}
          controls={state.controls}
          disabled={state.disabled}
        >
          Show help
        </Button>
      {/snippet}
      <div class="poodle-popover-content">
        <p>Popovers can be anchored to any side of their trigger element.</p>
      </div>
    </Popover>
  </SpecimenGroup>

  <SpecimenGroup label="Focus trap (opt-in)">
    <Popover trapFocus ariaLabel="Pinned tools" triggerIsInteractive>
      {#snippet trigger(state: PopoverTriggerState)}
        <Button
          variant="secondary"
          ariaExpanded={state.expanded}
          controls={state.controls}
          disabled={state.disabled}
        >
          Open tools
        </Button>
      {/snippet}
      <div class="poodle-popover-content">
        <strong>Pinned tools</strong>
        <p>Tab stays inside this surface. Escape and outside click still dismiss.</p>
        <div class="poodle-popover-actions">
          <Button variant="secondary" size="sm">Rename</Button>
          <Button variant="primary" size="sm">Apply</Button>
        </div>
      </div>
    </Popover>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-popover-content {
    padding: 0.75rem;
    max-width: 16rem;
  }

  .poodle-popover-content strong {
    display: block;
    margin-bottom: 0.25rem;
  }

  .poodle-popover-content p {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
  }

  .poodle-popover-actions {
    display: flex;
    gap: 0.5rem;
    margin-top: 0.75rem;
  }
</style>
