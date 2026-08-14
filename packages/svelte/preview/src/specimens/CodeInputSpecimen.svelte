<script lang="ts">
  import { CodeInput } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let code = $state("");
  let completed = $state(false);
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <CodeInput
      id="code-default"
      value={code}
      label="Verification code"
      hint="Enter the 6-digit code from your authenticator app."
      groups={[3, 3]}
      onValueChange={(value) => { code = value; completed = false; }}
      onComplete={() => { completed = true; }}
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
      length={4}
      mask
    />
  </SpecimenGroup>

  <SpecimenGroup label="Multiple groups">
    <CodeInput
      id="code-grouped"
      label="Grouped recovery key"
      defaultValue="AB12CD34"
      length={12}
      groups={[4, 4, 4]}
      numbersOnly={false}
      autocomplete="off"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Alphanumeric">
    <CodeInput
      id="code-alphanumeric"
      label="Recovery code"
      hint="Supports letters and numbers when the consumer opts in."
      defaultValue="AB12"
      length={6}
      numbersOnly={false}
      autocomplete="off"
    />
  </SpecimenGroup>

  <SpecimenGroup label="With error">
    <CodeInput
      id="code-error"
      value="12"
      label="Email code"
      error="That verification code is invalid."
      groups={[3, 3]}
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

  {#snippet sizes(size)}
    <CodeInput id={"size-" + size} {size} groups={[3, 3]} label={"Code at " + size} ariaLabel={"Code at " + size} />
  {/snippet}

  {#snippet densities(density)}
    <CodeInput id={"density-" + density} {density} groups={[3, 3]} label={"Code at " + density} ariaLabel={"Code at " + density} />
  {/snippet}
</SpecimenLayout>
