<script lang="ts">
  import { TextInput, Field, Eyebrow } from "@poodle/svelte-primitives";
  import type { InputValidationStatus, ValidationState } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let name = "";
  let email = "invalid-email";
  let validationState: ValidationState = "invalid";
  let workspace = "acme-admin";
  let workspaceStatus: InputValidationStatus = "idle";
  let workspaceError: string | null = null;

  async function validateWorkspace(value: string) {
    await new Promise((resolve) => setTimeout(resolve, 250));

    if (["admin", "settings", "support"].includes(value.trim().toLowerCase())) {
      return {
        valid: false,
        message: "That workspace handle is reserved.",
      };
    }

    return { valid: true };
  }
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <Field id="name-field" label="Name" description="Enter your full name.">
      <TextInput
        id="name-field"
        placeholder="Jane Doe"
        on:valueChange={(event) => (name = event.detail.value)}
      />
    </Field>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <TextInput id={"size-" + size} placeholder={size.toUpperCase()} {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each ["compact", "default", "comfortable"] as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <TextInput id={"density-" + density} placeholder="Type here" {density} />
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With validation</Eyebrow>
    <Field
      id="email-field"
      label="Email"
      description="A valid email address is required."
      validationState={validationState}
      error={validationState === "invalid" ? "Please enter a valid email address." : null}
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
    <Eyebrow>Async validation</Eyebrow>
    <Field
      id="workspace-field"
      label="Workspace"
      description="Check whether the workspace handle is available."
      validationState={workspaceStatus === "validating" ? "pending" : workspaceStatus === "invalid" ? "invalid" : workspaceStatus === "valid" ? "valid" : "none"}
      pendingMessage={workspaceStatus === "validating" ? "Checking availability..." : null}
      error={workspaceStatus === "invalid" ? workspaceError : null}
    >
      <TextInput
        id="workspace-field"
        value={workspace}
        autocomplete="off"
        required
        pattern="[a-z0-9-]+"
        validate={validateWorkspace}
        validationDebounce={250}
        validationContext={{ reserved: true }}
        on:valueChange={(event) => (workspace = event.detail.value)}
        on:validationChange={(event) => {
          workspaceStatus = event.detail.status;
          workspaceError = event.detail.message || null;
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
        disabled
      />
    </Field>
  </div>

  <div class="specimen__group">
    <Eyebrow>Multiline (explicit type)</Eyebrow>
    <Field id="multiline-field" label="Description">
      <TextInput
        id="multiline-field"
        type="multiline"
        placeholder="Enter a description..."
      />
    </Field>
  </div>

  <div class="specimen__group">
    <Eyebrow>Multiline (auto-detected from rows)</Eyebrow>
    <Field id="rows-field" label="Notes">
      <TextInput
        id="rows-field"
        rows={6}
        placeholder="Type your notes here..."
      />
    </Field>
  </div>

  <div class="specimen__group">
    <Eyebrow>Multiline with character count</Eyebrow>
    <Field id="bio-field" label="Bio">
      <TextInput
        id="bio-field"
        type="multiline"
        rows={3}
        maxLength={280}
        showCharCount
        placeholder="Tell us about yourself..."
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
