<script lang="ts">
  import { ContextMenu, type MenuItem, type ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const items: MenuItem[] = [
    { value: "cut", label: "Cut", shortcutLabel: "⌘X" },
    { value: "copy", label: "Copy", shortcutLabel: "⌘C" },
    { value: "paste", label: "Paste", shortcutLabel: "⌘V" },
    { value: "sep1", label: "", kind: "separator" },
    { value: "select-all", label: "Select all", shortcutLabel: "⌘A" },
    { value: "sep2", label: "", kind: "separator" },
    { value: "delete", label: "Delete", disabled: true },
  ];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let lastAction = "";
</script>

<div class="specimen">
  <SpecimenGroup label="Right-click the area below">
    <ContextMenu {items} on:action={(e) => (lastAction = e.detail.value)}>
      <div class="target-area">
        <p>Right-click here to open context menu</p>
      </div>
    </ContextMenu>
    {#if lastAction}
      <p>Last action: <strong>{lastAction}</strong></p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__row">
      {#each controlSizes as size}
        <ContextMenu {items} {size}>
          <div class="target-area target-area--small">
            <p>{size.toUpperCase()}</p>
          </div>
        </ContextMenu>
      {/each}
    </div>
  </SpecimenGroup>
  <SpecimenGroup label="Densities">
    <div class="specimen__row">
      {#each densities as density}
        <ContextMenu {items} {density}>
          <div class="target-area target-area--small">
            <p>{density}</p>
          </div>
        </ContextMenu>
      {/each}
    </div>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
  }

  .target-area {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 8rem;
    border: 2px dashed var(--poodle-color-border-default);
    border-radius: 4px;
  }

  .target-area--small {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 4rem;
    padding: 0 1rem;
    border: 2px dashed var(--poodle-color-border-default);
    border-radius: 4px;
  }

  p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
