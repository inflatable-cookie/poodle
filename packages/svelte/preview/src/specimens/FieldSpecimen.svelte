<script lang="ts">
  import { Field, TextInput } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
  <SpecimenGroup label="Default with description">
    <Field label="Display name" description="This is how your name appears to other users." id="field-name">
      <TextInput id="field-name" placeholder="Enter your name" ariaLabel="Display name" />
    </Field>
  </SpecimenGroup>

  <SpecimenGroup label="Required">
    <Field label="Email address" required id="field-email">
      <TextInput id="field-email" placeholder="you@example.com" ariaLabel="Email address" />
    </Field>
  </SpecimenGroup>

  <SpecimenGroup label="With error">
    <Field label="Username" error="This username is already taken." validationState="invalid" id="field-user">
      <TextInput id="field-user" value="admin" ariaLabel="Username" />
    </Field>
  </SpecimenGroup>

  <SpecimenGroup label="Valid">
    <Field label="Password" validationState="valid" description="Must be at least 8 characters." id="field-pass">
      <TextInput id="field-pass" value="••••••••••" ariaLabel="Password" />
    </Field>
  </SpecimenGroup>

  <SpecimenGroup label="Optional">
    <Field label="Phone number" optionalLabel="optional" id="field-phone">
      <TextInput id="field-phone" placeholder="+1 (555) 000-0000" ariaLabel="Phone number" />
    </Field>
  </SpecimenGroup>

  <SpecimenGroup label="With hint (progressive disclosure)">
    <Field label="URL Slug" hint="A URL-friendly identifier used in page addresses. Lowercase letters, numbers, and hyphens only." id="field-slug">
      <TextInput id="field-slug" placeholder="my-page-slug" ariaLabel="URL Slug" />
    </Field>
  </SpecimenGroup>

  <SpecimenGroup label="Hint + description + required">
    <Field label="API Key" description="Your personal API key for authentication." hint="Keep this key secret. Rotate it periodically for security." required id="field-api">
      <TextInput id="field-api" placeholder="sk_live_..." ariaLabel="API Key" />
    </Field>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 24rem;
  }

</style>
