<script lang="ts">
  import { TextInput, Field, Eyebrow } from "@flint/svelte-primitives";
  import type { ValidationState } from "@flint/svelte-primitives";

  let name = "";
  let email = "invalid-email";
  let validationState: ValidationState = "invalid";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <Field id="name-field" label="Name" helpText="Enter your full name.">
      <TextInput
        id="name-field"
        placeholder="Jane Doe"
        on:valueChange={(event) => (name = event.detail.value)}
      />
    </Field>
  </div>

  <div class="specimen__group">
    <Eyebrow>With validation</Eyebrow>
    <Field
      id="email-field"
      label="Email"
      helpText="A valid email address is required."
      {validationState}
      errorMessage="Please enter a valid email address."
    >
      <TextInput
        id="email-field"
        value={email}
        {validationState}
        on:valueChange={(event) => {
          email = event.detail.value;
          validationState = email.includes("@") ? "valid" : "invalid";
        }}
      />
    </Field>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <Field id="disabled-field" label="API key">
      <TextInput
        id="disabled-field"
        value="sk-xxxx-xxxx-xxxx"
        isDisabled
      />
    </Field>
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
</style>
