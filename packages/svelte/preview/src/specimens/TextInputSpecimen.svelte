<script lang="ts">
  import { TextInput, Field } from "@poodle/svelte-primitives";
  import type { InputValidationStatus, ValidationState } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let name = "";
  let email = "invalid-email";
  let validationState: ValidationState = "invalid";
  let workspace = "acme-admin";
  let workspaceStatus: InputValidationStatus = "idle";
  let workspaceError: string | null = null;
  let slug = "";
  let slugStatus: InputValidationStatus = "idle";
  let searchQuery = "";

  async function validateSlug(value: string) {
    await new Promise((resolve) => setTimeout(resolve, 250));

    if (value === "northstar") {
      return {
        valid: false,
        message: "That slug is already in use.",
      };
    }

    return { valid: true };
  }

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

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <Field id="name-field" label="Name" description="Enter your full name.">
      <TextInput
        id="name-field"
        placeholder="Jane Doe"
        on:valueChange={(event) => (name = event.detail.value)}
      />
    </Field>
  </SpecimenGroup>

  <SpecimenGroup label="With validation">
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
  </SpecimenGroup>

  <SpecimenGroup label="Async validation">
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
  </SpecimenGroup>

  <SpecimenGroup label="Slug">
    <Field
      id="slug-field"
      label="Slug"
      description="Generates from the title until the user edits it."
      validationState={slugStatus === "validating" ? "pending" : slugStatus === "invalid" ? "invalid" : slugStatus === "valid" ? "valid" : "none"}
      pendingMessage={slugStatus === "validating" ? "Checking slug..." : null}
      error={slugStatus === "invalid" ? "That slug is not available." : null}
    >
      <TextInput
        id="slug-field"
        type="slug"
        value={slug}
        source="Northstar Launch Plan"
        prefix="/projects/"
        maxLength={64}
        validate={validateSlug}
        on:valueChange={(event) => (slug = event.detail.value)}
        on:validationChange={(event) => {
          slugStatus = event.detail.status;
        }}
      />
    </Field>
  </SpecimenGroup>

  <SpecimenGroup label="Search">
    <TextInput
      id="search-field"
      type="search"
      placeholder="Search..."
      value={searchQuery}
      on:valueChange={(event) => (searchQuery = event.detail.value)}
      on:clear={() => (searchQuery = "")}
    />
    {#if searchQuery}
      <p class="specimen__hint">Query: <strong>{searchQuery}</strong></p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="Prefix and suffix">
    <TextInput
      id="price-field"
      prefix="$"
      suffix="USD"
      placeholder="0.00"
      inputMode="decimal"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Suffix only">
    <TextInput
      id="weight-field"
      suffix="kg"
      placeholder="0"
      inputMode="numeric"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <Field id="disabled-field" label="API key">
      <TextInput
        id="disabled-field"
        value="sk-xxxx-xxxx-xxxx"
        disabled
      />
    </Field>
  </SpecimenGroup>

  <SpecimenGroup label="Multiline (explicit type)">
    <Field id="multiline-field" label="Description">
      <TextInput
        id="multiline-field"
        type="multiline"
        placeholder="Enter a description..."
      />
    </Field>
  </SpecimenGroup>

  <SpecimenGroup label="Multiline (auto-detected from rows)">
    <Field id="rows-field" label="Notes">
      <TextInput
        id="rows-field"
        rows={6}
        placeholder="Type your notes here..."
      />
    </Field>
  </SpecimenGroup>

  <SpecimenGroup label="Multiline with character count">
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
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <TextInput id={"size-" + size} {size} placeholder={size.toUpperCase()} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <TextInput id={"density-" + density} {density} placeholder="Type here" />
  </svelte:fragment>
</SpecimenLayout>

<style>
  .specimen__hint {
    margin: 0;
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
