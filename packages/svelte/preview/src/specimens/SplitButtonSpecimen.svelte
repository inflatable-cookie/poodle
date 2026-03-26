<script lang="ts">
  import { SplitButton, Eyebrow } from "@poodle/svelte-primitives";
  import type { MenuItem } from "@poodle/svelte-primitives";

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
    <SplitButton variant="primary" items={saveItems} loading>Saving…</SplitButton>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <SplitButton variant="secondary" items={saveItems} disabled>Save</SplitButton>
  </div>

  <div class="specimen__group">
    <Eyebrow>Submit semantics</Eyebrow>
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
  </div>

  <div class="specimen__group">
    <Eyebrow>Constrained scroll container</Eyebrow>
    <div class="specimen__well">
      <div class="specimen__spacer"></div>
      <SplitButton
        variant="secondary"
        items={constrainedItems}
        on:click={() => (lastAction = "Queue action clicked")}
        on:action={(e) => (lastAction = `Queue action: ${e.detail.value}`)}
      >Queue actions</SplitButton>
    </div>
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
