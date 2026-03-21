<script lang="ts">
  import { FormLayout } from "@pug/svelte-composites";
  import {
    Eyebrow,
    Field,
    TextInput,
    TextArea,
    Select,
    Checkbox,
    Button,
    type SelectOption,
  } from "@pug/svelte-primitives";

  const roleOptions: SelectOption[] = [
    { value: "", label: "Select a role…" },
    { value: "admin", label: "Admin" },
    { value: "editor", label: "Editor" },
    { value: "viewer", label: "Viewer" },
  ];

  const regionOptions: SelectOption[] = [
    { value: "", label: "Select region…" },
    { value: "us", label: "United States" },
    { value: "eu", label: "Europe" },
    { value: "ap", label: "Asia Pacific" },
  ];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Two-column grid</Eyebrow>
    <FormLayout description="Fill in the details below to create a new user account.">
      <Field label="First name" id="fl-first">
        <TextInput id="fl-first" placeholder="Jane" />
      </Field>
      <Field label="Last name" id="fl-last">
        <TextInput id="fl-last" placeholder="Doe" />
      </Field>
      <Field label="Email" id="fl-email" span={2}>
        <TextInput id="fl-email" placeholder="jane@example.com" />
      </Field>
      <Field label="Role" id="fl-role">
        <Select id="fl-role" options={roleOptions} defaultValue="" ariaLabel="Role" />
      </Field>
      <Field label="Region" id="fl-region">
        <Select id="fl-region" options={regionOptions} defaultValue="" ariaLabel="Region" />
      </Field>
      <Field label="Notes" id="fl-notes" span={2}>
        <TextArea id="fl-notes" placeholder="Any additional notes…" />
      </Field>
      <svelte:fragment slot="actions">
        <Button variant="ghost">Cancel</Button>
        <Button variant="primary">Create user</Button>
      </svelte:fragment>
    </FormLayout>
  </div>

  <div class="specimen__group">
    <Eyebrow>Single column</Eyebrow>
    <FormLayout columns={1}>
      <Field label="Display name" id="fl-display">
        <TextInput id="fl-display" placeholder="Enter a name" />
      </Field>
      <Field label="Bio" id="fl-bio">
        <TextArea id="fl-bio" placeholder="Tell us about yourself…" />
      </Field>
      <Checkbox id="fl-agree" label="I agree to the terms" />
      <svelte:fragment slot="actions">
        <Button variant="primary">Save profile</Button>
      </svelte:fragment>
    </FormLayout>
  </div>

  <div class="specimen__group">
    <Eyebrow>With error and field errors</Eyebrow>
    <FormLayout
      error="Unable to save. Please fix the errors below."
      fieldErrors={{
        "Email": "This email is already in use",
        "Role": "A role is required"
      }}
    >
      <Field label="Email" id="fl-err-email" validationState="invalid" error="This email is already in use">
        <TextInput id="fl-err-email" value="taken@example.com" />
      </Field>
      <Field label="Role" id="fl-err-role" validationState="invalid" error="A role is required">
        <Select id="fl-err-role" options={roleOptions} defaultValue="" ariaLabel="Role" />
      </Field>
      <svelte:fragment slot="actions">
        <Button variant="primary">Retry</Button>
      </svelte:fragment>
    </FormLayout>
  </div>

  <div class="specimen__group">
    <Eyebrow>With success message</Eyebrow>
    <FormLayout success="Settings saved successfully." columns={1}>
      <Field label="Site name" id="fl-site">
        <TextInput id="fl-site" value="My Project" />
      </Field>
      <svelte:fragment slot="actions">
        <Button variant="primary">Save</Button>
      </svelte:fragment>
    </FormLayout>
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
</style>
