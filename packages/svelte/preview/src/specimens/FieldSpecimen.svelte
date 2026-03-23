<script lang="ts">
  import { Field, TextInput, Eyebrow } from "@flint/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default with description</Eyebrow>
    <Field label="Display name" description="This is how your name appears to other users." id="field-name">
      <TextInput id="field-name" placeholder="Enter your name" ariaLabel="Display name" />
    </Field>
  </div>

  <div class="specimen__group">
    <Eyebrow>Required</Eyebrow>
    <Field label="Email address" isRequired id="field-email">
      <TextInput id="field-email" placeholder="you@example.com" ariaLabel="Email address" />
    </Field>
  </div>

  <div class="specimen__group">
    <Eyebrow>With error</Eyebrow>
    <Field label="Username" error="This username is already taken." validationState="invalid" id="field-user">
      <TextInput id="field-user" value="admin" ariaLabel="Username" />
    </Field>
  </div>

  <div class="specimen__group">
    <Eyebrow>Valid</Eyebrow>
    <Field label="Password" validationState="valid" description="Must be at least 8 characters." id="field-pass">
      <TextInput id="field-pass" value="••••••••••" ariaLabel="Password" />
    </Field>
  </div>

  <div class="specimen__group">
    <Eyebrow>Optional</Eyebrow>
    <Field label="Phone number" optionalLabel="optional" id="field-phone">
      <TextInput id="field-phone" placeholder="+1 (555) 000-0000" ariaLabel="Phone number" />
    </Field>
  </div>

  <div class="specimen__group">
    <Eyebrow>With hint (progressive disclosure)</Eyebrow>
    <Field label="URL Slug" hint="A URL-friendly identifier used in page addresses. Lowercase letters, numbers, and hyphens only." id="field-slug">
      <TextInput id="field-slug" placeholder="my-page-slug" ariaLabel="URL Slug" />
    </Field>
  </div>

  <div class="specimen__group">
    <Eyebrow>Hint + description + required</Eyebrow>
    <Field label="API Key" description="Your personal API key for authentication." hint="Keep this key secret. Rotate it periodically for security." isRequired id="field-api">
      <TextInput id="field-api" placeholder="sk_live_..." ariaLabel="API Key" />
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
