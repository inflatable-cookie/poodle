import type { CSSProperties } from "react";
import { FieldSet, Field, TextInput, Select } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";

const formSectionsStyle: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "var(--poodle-space-stack-lg)",
};

export function FieldSetSpecimen() {
  return (
    <div className="poodle-specimen" style={{ maxWidth: "36rem" }}>
      <SpecimenGroup label="Contact Information">
        <FieldSet
          legend="Contact Information"
          description="We use this to reach you about your account."
        >
          <Field id="fs-name" label="Full Name" required>
            <TextInput id="fs-name" placeholder="Jane Smith" ariaLabel="Full Name" />
          </Field>
          <Field id="fs-email" label="Email" required description="We'll never share your email.">
            <TextInput id="fs-email" type="email" placeholder="jane@example.com" ariaLabel="Email" />
          </Field>
          <Field id="fs-phone" label="Phone" optionalLabel="Optional">
            <TextInput id="fs-phone" placeholder="+1 (555) 000-0000" ariaLabel="Phone" />
          </Field>
        </FieldSet>
      </SpecimenGroup>

      <SpecimenGroup label="Address (two-column)">
        <FieldSet legend="Address" columns={2}>
          <Field id="fs-street" label="Street" span="full">
            <TextInput id="fs-street" placeholder="123 Main St" ariaLabel="Street" />
          </Field>
          <Field id="fs-city" label="City">
            <TextInput id="fs-city" placeholder="Springfield" ariaLabel="City" />
          </Field>
          <Field id="fs-state" label="State">
            <Select
              id="fs-state"
              options={[
                { value: "ca", label: "California" },
                { value: "ny", label: "New York" },
                { value: "tx", label: "Texas" },
              ]}
              placeholder="Select state"
            />
          </Field>
          <Field id="fs-zip" label="ZIP Code">
            <TextInput id="fs-zip" placeholder="90210" ariaLabel="ZIP Code" />
          </Field>
          <Field id="fs-country" label="Country">
            <Select
              id="fs-country"
              options={[
                { value: "us", label: "United States" },
                { value: "ca", label: "Canada" },
                { value: "uk", label: "United Kingdom" },
              ]}
              placeholder="Select country"
            />
          </Field>
        </FieldSet>
      </SpecimenGroup>

      <SpecimenGroup label="No legend, small gap">
        <FieldSet columns={2} gap="sm">
          <Field id="fs-first" label="First Name">
            <TextInput id="fs-first" placeholder="Jane" ariaLabel="First Name" />
          </Field>
          <Field id="fs-last" label="Last Name">
            <TextInput id="fs-last" placeholder="Smith" ariaLabel="Last Name" />
          </Field>
        </FieldSet>
      </SpecimenGroup>

      <SpecimenGroup label="Multiple sections">
        <div style={formSectionsStyle}>
          <FieldSet legend="Personal" columns={2}>
            <Field id="fs2-first" label="First Name" required>
              <TextInput id="fs2-first" ariaLabel="First Name" />
            </Field>
            <Field id="fs2-last" label="Last Name" required>
              <TextInput id="fs2-last" ariaLabel="Last Name" />
            </Field>
          </FieldSet>
          <FieldSet legend="Preferences">
            <Field id="fs2-lang" label="Language">
              <Select
                id="fs2-lang"
                options={[
                  { value: "en", label: "English" },
                  { value: "es", label: "Spanish" },
                  { value: "fr", label: "French" },
                ]}
                placeholder="Select language"
              />
            </Field>
            <Field id="fs2-tz" label="Time Zone" description="Used for scheduling and notifications.">
              <Select
                id="fs2-tz"
                options={[
                  { value: "utc", label: "UTC" },
                  { value: "est", label: "Eastern" },
                  { value: "pst", label: "Pacific" },
                ]}
                placeholder="Select time zone"
              />
            </Field>
          </FieldSet>
        </div>
      </SpecimenGroup>
    </div>
  );
}
