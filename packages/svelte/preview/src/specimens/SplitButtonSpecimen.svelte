<script lang="ts">
  import { SplitButton, Eyebrow } from "@poodle/svelte-primitives";
  import type { MenuItem } from "@poodle/svelte-primitives";

  let lastAction = "";

  const saveItems: MenuItem[] = [
    { value: "save-draft", label: "Save as draft" },
    { value: "save-template", label: "Save as template" },
    { value: "separator-1", label: "", kind: "separator" },
    { value: "discard", label: "Discard changes" },
  ];

  const exportItems: MenuItem[] = [
    { value: "csv", label: "Export as CSV" },
    { value: "json", label: "Export as JSON" },
    { value: "pdf", label: "Export as PDF" },
  ];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Primary variant</Eyebrow>
    <SplitButton
      variant="primary"
      items={saveItems}
      on:click={() => (lastAction = "Primary save clicked")}
      on:action={(e) => (lastAction = `Action: ${e.detail.value}`)}
    >Save</SplitButton>
  </div>

  <div class="specimen__group">
    <Eyebrow>Secondary variant</Eyebrow>
    <SplitButton
      variant="secondary"
      items={exportItems}
      on:click={() => (lastAction = "Export clicked")}
      on:action={(e) => (lastAction = `Export: ${e.detail.value}`)}
    >Export</SplitButton>
  </div>

  <div class="specimen__group">
    <Eyebrow>Danger tone</Eyebrow>
    <SplitButton
      tone="danger"
      items={[
        { value: "delete-selected", label: "Delete selected" },
        { value: "delete-all", label: "Delete all" },
      ]}
      on:click={() => (lastAction = "Delete clicked")}
      on:action={(e) => (lastAction = `Delete: ${e.detail.value}`)}
    >Delete</SplitButton>
  </div>

  <div class="specimen__group">
    <Eyebrow>Loading state</Eyebrow>
    <SplitButton variant="primary" items={saveItems} isLoading>Saving…</SplitButton>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <SplitButton variant="secondary" items={saveItems} isDisabled>Save</SplitButton>
  </div>

  {#if lastAction}
    <div class="specimen__group">
      <Eyebrow>Last action</Eyebrow>
      <p>{lastAction}</p>
    </div>
  {/if}
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

  p { margin: 0; }
</style>
