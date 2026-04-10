<script lang="ts">
  import { SplitButton } from "@poodle/svelte-primitives";
  import type { MenuItem } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let lastAction = "";
  let submitIntent = "save-close";
  let submittedWith = "";

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

  const submitItems: MenuItem[] = [
    { value: "save", label: "Save changes" },
    { value: "save-close", label: "Save & close" },
  ];

  const constrainedItems: MenuItem[] = [
    { value: "restart", label: "Restart worker" },
    { value: "requeue", label: "Requeue job" },
    { value: "silence", label: "Silence alerts" },
    { value: "export", label: "Export logs" },
    { value: "archive", label: "Archive queue" },
  ];

  function handleIntentAction(value: string): void {
    submitIntent = value;
  }

  function handleSubmit(event: SubmitEvent): void {
    event.preventDefault();
    submittedWith = submitIntent;
  }
</script>

<SpecimenLayout>
  <SpecimenGroup label="Primary variant">
    <SplitButton
      variant="primary"
      items={saveItems}
      on:click={() => (lastAction = "Primary save clicked")}
      on:action={(e) => (lastAction = `Action: ${e.detail.value}`)}
    >Save</SplitButton>
  </SpecimenGroup>

  <SpecimenGroup label="Secondary variant">
    <SplitButton
      variant="secondary"
      items={exportItems}
      on:click={() => (lastAction = "Export clicked")}
      on:action={(e) => (lastAction = `Export: ${e.detail.value}`)}
    >Export</SplitButton>
  </SpecimenGroup>

  <SpecimenGroup label="Danger tone">
    <SplitButton
      tone="danger"
      items={[
        { value: "delete-selected", label: "Delete selected" },
        { value: "delete-all", label: "Delete all" },
      ]}
      on:click={() => (lastAction = "Delete clicked")}
      on:action={(e) => (lastAction = `Delete: ${e.detail.value}`)}
    >Delete</SplitButton>
  </SpecimenGroup>

  <SpecimenGroup label="Loading state">
    <SplitButton variant="primary" items={saveItems} loading>Saving…</SplitButton>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <SplitButton variant="secondary" items={saveItems} disabled>Save</SplitButton>
  </SpecimenGroup>

  <SpecimenGroup label="Submit semantics">
    <form class="specimen__form" on:submit={handleSubmit}>
      <input type="hidden" name="intent" value={submitIntent} />
      <SplitButton
        type="submit"
        variant="primary"
        items={submitItems}
        on:action={(e) => handleIntentAction(e.detail.value)}
      >
        {submitIntent === "save" ? "Save changes" : "Save & close"}
      </SplitButton>
    </form>
    {#if submittedWith}
      <p>Submitted with intent: {submittedWith}</p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="Constrained scroll container">
    <div class="specimen__well">
      <div class="specimen__spacer"></div>
      <SplitButton
        variant="secondary"
        items={constrainedItems}
        on:click={() => (lastAction = "Queue action clicked")}
        on:action={(e) => (lastAction = `Queue action: ${e.detail.value}`)}
      >Queue actions</SplitButton>
    </div>
  </SpecimenGroup>

  {#if lastAction}
    <SpecimenGroup label="Last action">
      <p>{lastAction}</p>
    </SpecimenGroup>
  {/if}

  <svelte:fragment slot="densities" let:density>
    <SplitButton variant="primary" {density} items={saveItems}>Save</SplitButton>
  </svelte:fragment>
</SpecimenLayout>

<style>
  .specimen__form {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .specimen__well {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    max-height: 12rem;
    padding: 1rem;
    overflow: auto;
    border: 1px solid color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 92%, transparent);
  }

  .specimen__spacer {
    min-height: 8rem;
  }

  p { margin: 0; }
</style>
