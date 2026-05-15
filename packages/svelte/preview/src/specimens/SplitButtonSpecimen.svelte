<script lang="ts">
  import { SplitButton, Eyebrow, Surface } from "@poodle/svelte";
  import type { MenuItem } from "@poodle/svelte";
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
    <div class="poodle-specimen">
      <div class="poodle-specimen__row">
        <Eyebrow>Primary</Eyebrow>
        <SplitButton variant="primary" items={saveItems} onClick={() => (lastAction = "Save")} onAction={(value) => (lastAction = value)}>Save</SplitButton>
      </div>

      <div class="poodle-specimen__row">
        <Eyebrow>Secondary</Eyebrow>
        <SplitButton variant="secondary" items={exportItems} onClick={() => (lastAction = "Export")} onAction={(value) => (lastAction = value)}>Export</SplitButton>
      </div>

      <div class="poodle-specimen__row">
        <Eyebrow>Danger</Eyebrow>
        <SplitButton tone="danger" items={[{ value: "delete-selected", label: "Delete selected" }, { value: "delete-all", label: "Delete all" }]} onClick={() => (lastAction = "Delete")} onAction={(value) => (lastAction = value)}>Delete</SplitButton>
      </div>

      <div class="poodle-specimen__row">
        <Eyebrow>Loading</Eyebrow>
        <SplitButton variant="primary" items={saveItems} loading>Saving…</SplitButton>
      </div>

      <div class="poodle-specimen__row">
        <Eyebrow>Disabled</Eyebrow>
        <SplitButton variant="secondary" items={saveItems} disabled>Save</SplitButton>
      </div>

      {#if lastAction}
        <p class="poodle-specimen__hint">Last action: <strong>{lastAction}</strong></p>
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
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .poodle-specimen__row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
  }

  .poodle-specimen__hint {
    margin: 0;
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
