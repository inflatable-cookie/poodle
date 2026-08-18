<script lang="ts">
  import { SplitButton } from "@inflatable-cookie/poodle-svelte";
  import type { MenuItem } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let lastAction = $state("");

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
    <SpecimenGroup label="Primary">
    <SplitButton variant="primary" items={saveItems} onClick={() => (lastAction = "Save")} onAction={(value) => (lastAction = value)}>Save</SplitButton>
  </SpecimenGroup>

        <SpecimenGroup label="Secondary">
    <SplitButton variant="secondary" items={exportItems} onClick={() => (lastAction = "Export")} onAction={(value) => (lastAction = value)}>Export</SplitButton>
  </SpecimenGroup>

        <SpecimenGroup label="Danger">
    <SplitButton tone="danger" items={[{ value: "delete-selected", label: "Delete selected" }, { value: "delete-all", label: "Delete all" }]} onClick={() => (lastAction = "Delete")} onAction={(value) => (lastAction = value)}>Delete</SplitButton>
  </SpecimenGroup>

        <SpecimenGroup label="Success">
    <SplitButton tone="success" items={[{ value: "publish-now", label: "Publish now" }, { value: "schedule-publish", label: "Schedule" }]} onClick={() => (lastAction = "Publish")} onAction={(value) => (lastAction = value)}>Publish</SplitButton>
  </SpecimenGroup>

        <SpecimenGroup label="Warning">
    <SplitButton tone="warning" items={[{ value: "archive-selected", label: "Archive selected" }, { value: "archive-all", label: "Archive all" }]} onClick={() => (lastAction = "Archive")} onAction={(value) => (lastAction = value)}>Archive</SplitButton>
  </SpecimenGroup>

        <SpecimenGroup label="Loading">
    <SplitButton variant="primary" items={saveItems} loading>Saving…</SplitButton>
  </SpecimenGroup>

        <SpecimenGroup label="Disabled">
    <SplitButton variant="secondary" items={saveItems} disabled>Save</SplitButton>
  </SpecimenGroup>

      {#if lastAction}
        <p class="poodle-specimen__hint">Last action: <strong>{lastAction}</strong></p>
      {/if}

  {#snippet sizes(size)}
    <SplitButton variant="primary" items={saveItems} {size}>Save</SplitButton>
  {/snippet}

  {#snippet densities(density)}
    <SplitButton variant="primary" items={saveItems} {density}>Save</SplitButton>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen__hint {
    margin: 0;
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
