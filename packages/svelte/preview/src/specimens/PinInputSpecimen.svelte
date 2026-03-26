<script lang="ts">
  import { PinInput, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let code = "";
  let completed = false;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>6-digit code</Eyebrow>
    <PinInput
      length={6}
      ariaLabel="Verification code"
      on:valueChange={(e) => { code = e.detail.value; completed = false; }}
      on:complete={() => (completed = true)}
    />
    {#if completed}
      <p>Code entered: <strong>{code}</strong></p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <PinInput length={4} ariaLabel={"PIN at " + size} {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>4-digit masked</Eyebrow>
    <PinInput length={4} mask ariaLabel="PIN" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <PinInput length={6} defaultValue="123" disabled ariaLabel="Disabled code" />
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

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
</style>
