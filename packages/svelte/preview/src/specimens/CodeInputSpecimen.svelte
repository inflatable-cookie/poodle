<script lang="ts">
  import { CodeInput } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let code = "";
  let completed = false;
</script>

<SpecimenLayout>
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

  <svelte:fragment slot="sizes" let:size>
    <CodeInput id={"size-" + size} label={"Code at " + size} ariaLabel={"Code at " + size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <CodeInput id={"density-" + density} label={"Code at " + density} ariaLabel={"Code at " + density} />
  </svelte:fragment>
</SpecimenLayout>
