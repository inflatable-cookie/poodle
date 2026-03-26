<script lang="ts">
  import { TextArea, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let note = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <TextArea
      id="note"
      placeholder="Write a note…"
      ariaLabel="Note"
      on:valueChange={(e) => (note = e.detail.value)}
    />
    {#if note}
      <p>{note.length} characters</p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <TextArea id={"size-" + size} placeholder={size.toUpperCase()} rows={2} {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With initial value</Eyebrow>
    <TextArea
      id="bio"
      defaultValue="A brief description about yourself."
      rows={3}
      ariaLabel="Biography"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Read-only</Eyebrow>
    <TextArea
      id="readonly"
      defaultValue="This content cannot be modified by the user."
      rows={2}
      readOnly
      ariaLabel="Read-only textarea"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <TextArea id="disabled" placeholder="Disabled" disabled ariaLabel="Disabled textarea" />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 24rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
