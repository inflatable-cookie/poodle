<script lang="ts">
  import { Eyebrow, TotpInput } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let code = "";
  let completed = false;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <TotpInput
      id="totp-default"
      value={code}
      label="Verification code"
      hint="Enter the 6-digit code from your authenticator app."
      on:valueChange={(event) => { code = event.detail.value; completed = false; }}
      on:complete={() => { completed = true; }}
    />
    {#if completed}
      <p>Completed value: <strong>{code}</strong></p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <TotpInput id={"size-" + size} label={"Code at " + size} ariaLabel={"Code at " + size} {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each ["compact", "default", "comfortable"] as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <TotpInput id={"density-" + density} label={"Code at " + density} ariaLabel={"Code at " + density} {density} />
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With error</Eyebrow>
    <TotpInput
      id="totp-error"
      value="12"
      label="Email code"
      error="That verification code is invalid."
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <TotpInput
      id="totp-disabled"
      defaultValue="1234"
      length={4}
      label="Disabled code"
      disabled
    />
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

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }
</style>
