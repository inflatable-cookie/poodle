<script lang="ts">
  import { PasswordRequirements } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

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
  <SpecimenGroup label="Default">
    <label class="specimen__field">
      <span>Password</span>
      <input bind:value={password} type="password" />
    </label>
    <PasswordRequirements {password} {requirements} />
  </SpecimenGroup>

  <SpecimenGroup label="Loading">
    <PasswordRequirements password="" requirements={null} loading />
  </SpecimenGroup>

  <SpecimenGroup label="Error">
    <PasswordRequirements
      password=""
      requirements={null}
      error="Could not load password requirements."
    />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 28rem;
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
