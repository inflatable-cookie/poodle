<script lang="ts">
  import { TextInput, Field, Eyebrow, Surface } from "@poodle/svelte";
  import type { InputValidationStatus, ValidationState } from "@poodle/svelte";
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
    if (value === "northstar") return { valid: false, message: "That slug is already in use." };
    return { valid: true };
  }

  async function validateWorkspace(value: string) {
    await new Promise((resolve) => setTimeout(resolve, 250));
    if (["admin", "settings", "support"].includes(value.trim().toLowerCase())) {
      return { valid: false, message: "That workspace handle is reserved." };
    }
    return { valid: true };
  }
</script>

<SpecimenLayout>
  <Surface tone="panel" border="subtle" padding="md">
    <div class="specimen">
      <div class="specimen__item">
        <Eyebrow>Default</Eyebrow>
        <Field id="name-field" label="Name" description="Enter your full name.">
          <TextInput id="name-field" placeholder="Jane Doe" on:valueChange={(event) => (name = event.detail.value)} />
        </Field>
      </div>

      <div class="specimen__item">
        <Eyebrow>With validation</Eyebrow>
        <Field id="email-field" label="Email" description="A valid email address is required." validationState={validationState} error={validationState === "invalid" ? "Please enter a valid email address." : null}>
          <TextInput id="email-field" value={email} {validationState} on:valueChange={(event) => { email = event.detail.value; validationState = email.includes("@") ? "valid" : "invalid"; }} />
        </Field>
      </div>

      <div class="specimen__item">
        <Eyebrow>Slug</Eyebrow>
        <Field id="slug-field" label="Slug" description="Generates from the title until the user edits it." validationState={slugStatus === "validating" ? "pending" : slugStatus === "invalid" ? "invalid" : slugStatus === "valid" ? "valid" : "none"} error={slugStatus === "invalid" ? "That slug is not available." : null}>
          <TextInput id="slug-field" type="slug" value={slug} source="Northstar Launch Plan" prefix="/projects/" maxLength={64} validate={validateSlug} on:valueChange={(event) => (slug = event.detail.value)} on:validationChange={(event) => { slugStatus = event.detail.status; }} />
        </Field>
      </div>

      <div class="specimen__item">
        <Eyebrow>Search</Eyebrow>
        <TextInput id="search-field" type="search" placeholder="Search..." value={searchQuery} on:valueChange={(event) => (searchQuery = event.detail.value)} on:clear={() => (searchQuery = "")} />
      </div>

      <div class="specimen__item">
        <Eyebrow>Prefix and suffix</Eyebrow>
        <TextInput id="price-field" prefix="$" suffix="USD" placeholder="0.00" inputMode="decimal" />
      </div>

      <div class="specimen__item">
        <Eyebrow>Disabled</Eyebrow>
        <Field id="disabled-field" label="API key">
          <TextInput id="disabled-field" value="sk-xxxx-xxxx-xxxx" disabled />
        </Field>
      </div>

      <div class="specimen__item">
        <Eyebrow>Multiline</Eyebrow>
        <Field id="multiline-field" label="Description">
          <TextInput id="multiline-field" type="multiline" rows={3} maxLength={280} showCharCount placeholder="Enter a description..." />
        </Field>
      </div>
    </div>
  </Surface>

  <svelte:fragment slot="sizes" let:size>
    <TextInput id={"size-" + size} {size} placeholder={size.toUpperCase()} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <TextInput id={"density-" + density} {density} placeholder="Type here" />
  </svelte:fragment>
</SpecimenLayout>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .specimen__item {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }
</style>
