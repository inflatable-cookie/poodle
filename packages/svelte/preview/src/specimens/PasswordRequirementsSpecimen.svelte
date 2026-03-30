<script lang="ts">
  import { Eyebrow, PasswordRequirements } from "@poodle/svelte-primitives";

  let password = "";

  const requirements = {
    minLength: 12,
    requireMixedCase: true,
    requireDigit: true,
    requireSpecial: true,
    minStrengthScore: 3,
    description: "Use a long password with varied character types."
  };
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <label class="specimen__field">
      <span>Password</span>
      <input bind:value={password} type="password" />
    </label>
    <PasswordRequirements {password} {requirements} />
  </div>

  <div class="specimen__group">
    <Eyebrow>Loading</Eyebrow>
    <PasswordRequirements password="" requirements={null} loading />
  </div>

  <div class="specimen__group">
    <Eyebrow>Error</Eyebrow>
    <PasswordRequirements
      password=""
      requirements={null}
      error="Could not load password requirements."
    />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 28rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__field {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }

  .specimen__field input {
    padding: 0.625rem 0.75rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
  }
</style>
