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

  const countryOptions: SelectOption[] = [
    { value: "", label: "Select country…" },
    { value: "us", label: "United States" },
    { value: "gb", label: "United Kingdom" },
    { value: "de", label: "Germany" },
  ];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Two-column layout (span 3 = half)</Eyebrow>
    <FormLayout description="Fill in the details below to create a new user account.">
      <Field label="First name" id="fl-first" span={3}>
        <TextInput id="fl-first" placeholder="Jane" />
      </Field>
      <Field label="Last name" id="fl-last" span={3}>
        <TextInput id="fl-last" placeholder="Doe" />
      </Field>
      <Field label="Email" id="fl-email" span={6}>
        <TextInput id="fl-email" placeholder="jane@example.com" />
      </Field>
      <Field label="Role" id="fl-role" span={3}>
        <Select id="fl-role" options={roleOptions} defaultValue="" ariaLabel="Role" />
      </Field>
      <Field label="Region" id="fl-region" span={3}>
        <Select id="fl-region" options={regionOptions} defaultValue="" ariaLabel="Region" />
      </Field>
      <Field label="Notes" id="fl-notes" span={6}>
        <TextArea id="fl-notes" placeholder="Any additional notes…" />
      </Field>
      <svelte:fragment slot="actions">
        <Button variant="ghost">Cancel</Button>
        <Button variant="primary">Create user</Button>
      </svelte:fragment>
    </FormLayout>
  </div>

  <div class="specimen__group">
    <Eyebrow>Mixed 2-col and 3-col rows</Eyebrow>
    <FormLayout description="Mixing half-width (span 3) and third-width (span 2) fields.">
      <Field label="First name" id="fl-mix-first" span={2}>
        <TextInput id="fl-mix-first" placeholder="Jane" />
      </Field>
      <Field label="Middle name" id="fl-mix-middle" span={2}>
        <TextInput id="fl-mix-middle" placeholder="M." />
      </Field>
      <Field label="Last name" id="fl-mix-last" span={2}>
        <TextInput id="fl-mix-last" placeholder="Doe" />
      </Field>
      <Field label="Email" id="fl-mix-email" span={3}>
        <TextInput id="fl-mix-email" placeholder="jane@example.com" />
      </Field>
      <Field label="Phone" id="fl-mix-phone" span={3}>
        <TextInput id="fl-mix-phone" placeholder="+1 555 0100" />
      </Field>
      <Field label="Role" id="fl-mix-role" span={2}>
        <Select id="fl-mix-role" options={roleOptions} defaultValue="" ariaLabel="Role" />
      </Field>
      <Field label="Region" id="fl-mix-region" span={2}>
        <Select id="fl-mix-region" options={regionOptions} defaultValue="" ariaLabel="Region" />
      </Field>
      <Field label="Country" id="fl-mix-country" span={2}>
        <Select id="fl-mix-country" options={countryOptions} defaultValue="" ariaLabel="Country" />
      </Field>
      <Field label="Bio" id="fl-mix-bio" span={6}>
        <TextArea id="fl-mix-bio" placeholder="Tell us about yourself…" />
      </Field>
      <svelte:fragment slot="actions">
        <Button variant="ghost">Cancel</Button>
        <Button variant="primary">Save</Button>
      </svelte:fragment>
    </FormLayout>
  </div>

  <div class="specimen__group">
    <Eyebrow>Single column (columns=1)</Eyebrow>
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
      <Field label="Email" id="fl-err-email" span={3} validationState="invalid" error="This email is already in use">
        <TextInput id="fl-err-email" value="taken@example.com" />
      </Field>
      <Field label="Role" id="fl-err-role" span={3} validationState="invalid" error="A role is required">
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
