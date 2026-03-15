<script lang="ts">
  import { ContextMenu, Eyebrow, type MenuItem } from "@pug/svelte-primitives";

  const items: MenuItem[] = [
    { value: "cut", label: "Cut", shortcutLabel: "⌘X" },
    { value: "copy", label: "Copy", shortcutLabel: "⌘C" },
    { value: "paste", label: "Paste", shortcutLabel: "⌘V" },
    { value: "sep1", label: "", kind: "separator" },
    { value: "select-all", label: "Select all", shortcutLabel: "⌘A" },
    { value: "sep2", label: "", kind: "separator" },
    { value: "delete", label: "Delete", isDisabled: true },
  ];

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

  .target-area {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 8rem;
    border: 2px dashed var(--pug-color-border-default);
    border-radius: 4px;
  }

  p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--pug-color-text-secondary);
  }
</style>
