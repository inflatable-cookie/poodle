<script lang="ts">
  import { CodeInput } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let code = "";
  let completed = false;
</script>

<div class="specimen">
  <SpecimenGroup label="Default">
    <CodeInput
      id="code-default"
      value={code}
      label="Verification code"
      hint="Enter the 6-digit code from your authenticator app."
      on:valueChange={(event) => { code = event.detail.value; completed = false; }}
      on:complete={() => { completed = true; }}
    />
    {#if completed}
      <p>Completed value: <strong>{code}</strong></p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="Masked">
    <CodeInput
      id="code-masked"
      label="PIN code"
      hint="Digits are hidden for security."
      mask
    />
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <CodeInput id={"size-" + size} label={"Code at " + size} ariaLabel={"Code at " + size} {size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <CodeInput id={"density-" + density} label={"Code at " + density} ariaLabel={"Code at " + density} {density} />
        </div>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="With error">
    <CodeInput
      id="code-error"
      value="12"
      label="Email code"
      error="That verification code is invalid."
    />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <CodeInput
      id="code-disabled"
      defaultValue="1234"
      length={4}
      label="Disabled code"
      disabled
    />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 24rem;
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
