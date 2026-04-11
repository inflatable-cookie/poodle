<script lang="ts">
  import { SplitButton, Eyebrow, Surface } from "@poodle/svelte-primitives";
  import type { MenuItem } from "@poodle/svelte-primitives";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

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

<SpecimenLayout>
  <Surface tone="panel" border="subtle" padding="md">
    <div class="specimen">
      <div class="specimen__row">
        <Eyebrow>Primary</Eyebrow>
        <SplitButton variant="primary" items={saveItems} on:click={() => (lastAction = "Save")} on:action={(e) => (lastAction = e.detail.value)}>Save</SplitButton>
      </div>

      <div class="specimen__row">
        <Eyebrow>Secondary</Eyebrow>
        <SplitButton variant="secondary" items={exportItems} on:click={() => (lastAction = "Export")} on:action={(e) => (lastAction = e.detail.value)}>Export</SplitButton>
      </div>

      <div class="specimen__row">
        <Eyebrow>Danger</Eyebrow>
        <SplitButton tone="danger" items={[{ value: "delete-selected", label: "Delete selected" }, { value: "delete-all", label: "Delete all" }]} on:click={() => (lastAction = "Delete")} on:action={(e) => (lastAction = e.detail.value)}>Delete</SplitButton>
      </div>

      <div class="specimen__row">
        <Eyebrow>Loading</Eyebrow>
        <SplitButton variant="primary" items={saveItems} loading>Saving…</SplitButton>
      </div>

      <div class="specimen__row">
        <Eyebrow>Disabled</Eyebrow>
        <SplitButton variant="secondary" items={saveItems} disabled>Save</SplitButton>
      </div>

      {#if lastAction}
        <p class="specimen__hint">Last action: <strong>{lastAction}</strong></p>
      {/if}
    </div>
  </Surface>

  <svelte:fragment slot="sizes" let:size>
    <SplitButton variant="primary" items={saveItems} {size}>Save</SplitButton>
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <SplitButton variant="primary" items={saveItems} {density}>Save</SplitButton>
  </svelte:fragment>
</SpecimenLayout>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
  }

  .specimen__hint {
    margin: 0;
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
