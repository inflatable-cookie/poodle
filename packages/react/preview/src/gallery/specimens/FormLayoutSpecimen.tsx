import { FormLayout, Field, TextInput, Select, Checkbox, Button, type SelectOption } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

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

export function FormLayoutSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Two-column layout (span 3 = half)">
        <FormLayout
          description="Fill in the details below to create a new user account."
          actions={
            <>
              <Button variant="ghost">Cancel</Button>
              <Button variant="primary">Create user</Button>
            </>
          }
        >
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
            <TextInput id="fl-notes" placeholder="Any additional notes…" rows={3} />
          </Field>
        </FormLayout>
      </SpecimenGroup>

      <SpecimenGroup label="Mixed 2-col and 3-col rows">
        <FormLayout
          description="Mixing half-width (span 3) and third-width (span 2) fields."
          actions={
            <>
              <Button variant="ghost">Cancel</Button>
              <Button variant="primary">Save</Button>
            </>
          }
        >
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
            <TextInput id="fl-mix-bio" placeholder="Tell us about yourself…" rows={3} />
          </Field>
        </FormLayout>
      </SpecimenGroup>

      <SpecimenGroup label="Single column (columns=1)">
        <FormLayout columns={1} actions={<Button variant="primary">Save profile</Button>}>
          <Field label="Display name" id="fl-display">
            <TextInput id="fl-display" placeholder="Enter a name" />
          </Field>
          <Field label="Bio" id="fl-bio">
            <TextInput id="fl-bio" placeholder="Tell us about yourself…" rows={3} />
          </Field>
          <Checkbox id="fl-agree" label="I agree to the terms" />
        </FormLayout>
      </SpecimenGroup>

      <SpecimenGroup label="With error and field errors">
        <FormLayout
          error="Unable to save. Please fix the errors below."
          fieldErrors={{
            Email: "This email is already in use",
            Role: "A role is required",
          }}
          actions={<Button variant="primary">Retry</Button>}
        >
          <Field
            label="Email"
            id="fl-err-email"
            span={3}
            validationState="invalid"
            error="This email is already in use"
          >
            <TextInput id="fl-err-email" value="taken@example.com" />
          </Field>
          <Field label="Role" id="fl-err-role" span={3} validationState="invalid" error="A role is required">
            <Select id="fl-err-role" options={roleOptions} defaultValue="" ariaLabel="Role" />
          </Field>
        </FormLayout>
      </SpecimenGroup>

      <SpecimenGroup label="With success message">
        <FormLayout success="Settings saved successfully." columns={1} actions={<Button variant="primary">Save</Button>}>
          <Field label="Site name" id="fl-site">
            <TextInput id="fl-site" value="My Project" />
          </Field>
        </FormLayout>
      </SpecimenGroup>
    </div>
  );
}
