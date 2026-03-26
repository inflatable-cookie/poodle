<script lang="ts">
  import { ContextMenu, Eyebrow, type MenuItem } from "@poodle/svelte-primitives";

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
  <div class="specimen__group">
    <Eyebrow>Right-click the area below</Eyebrow>
    <ContextMenu {items} on:action={(e) => (lastAction = e.detail.value)}>
      <div class="target-area">
        <p>Right-click here to open context menu</p>
      </div>
    </ContextMenu>
    {#if lastAction}
      <p>Last action: <strong>{lastAction}</strong></p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__row">
      {#each controlSizes as size}
        <ContextMenu {items} {size}>
          <div class="target-area target-area--small">
            <p>{size.toUpperCase()}</p>
          </div>
        </ContextMenu>
      {/each}
    </div>
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
    gap: 0.5rem;
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
