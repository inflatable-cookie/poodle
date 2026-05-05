<script lang="ts">
  import { ContextMenu, type MenuItem } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const items: MenuItem[] = [
    { value: "cut", label: "Cut", shortcutLabel: "⌘X" },
    { value: "copy", label: "Copy", shortcutLabel: "⌘C" },
    { value: "paste", label: "Paste", shortcutLabel: "⌘V" },
    { value: "sep1", label: "", kind: "separator" },
    { value: "select-all", label: "Select all", shortcutLabel: "⌘A" },
    { value: "sep2", label: "", kind: "separator" },
    { value: "delete", label: "Delete", disabled: true },
  ];

  let lastAction = "";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Right-click the area below">
    <ContextMenu {items} on:action={(e) => (lastAction = e.detail.value)}>
      <div class="poodle-target-area">
        <p>Right-click here to open context menu</p>
      </div>
    </ContextMenu>
    {#if lastAction}
      <p>Last action: <strong>{lastAction}</strong></p>
    {/if}
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <ContextMenu {items} {size}>
      <div class="poodle-target-area poodle-target-area--small">
        <p>{size.toUpperCase()}</p>
      </div>
    </ContextMenu>
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <ContextMenu {items} {density}>
      <div class="poodle-target-area poodle-target-area--small">
        <p>{density}</p>
      </div>
    </ContextMenu>
  </svelte:fragment>
</SpecimenLayout>

<style>
  .poodle-target-area {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 8rem;
    border: 2px dashed var(--poodle-color-border-default);
    border-radius: 4px;
  }

  .poodle-target-area--small {
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
