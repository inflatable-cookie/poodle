<script lang="ts">
  import { FieldSet, Field, TextInput, Select } from "@poodle/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__card">
    <FieldSet legend="Contact Information">
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
  </div>

  <div class="specimen__card">
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
  </div>

  <div class="specimen__card">
    <FieldSet columns={2} gap="sm">
      <Field id="fs-first" label="First Name">
        <TextInput id="fs-first" placeholder="Jane" ariaLabel="First Name" />
      </Field>
      <Field id="fs-last" label="Last Name">
        <TextInput id="fs-last" placeholder="Smith" ariaLabel="Last Name" />
      </Field>
    </FieldSet>
  </div>

  <div class="specimen__card">
    <div class="form-sections">
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
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 36rem;
    /* Break out of the parent Surface padding */
    margin: calc(-1 * var(--poodle-space-panel-y, 0.75rem)) calc(-1 * var(--poodle-space-panel-x, 1rem));
    padding: var(--poodle-space-panel-y, 0.75rem) var(--poodle-space-panel-x, 1rem);
  }

  .specimen__card {
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 93%, var(--poodle-color-background-elevated));
  }

  .form-sections {
    display: flex;
    flex-direction: column;
    gap: var(--poodle-space-stack-lg);
  }
</style>
