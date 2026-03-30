<script lang="ts">
  import type { PasswordRequirementsPolicy } from "./types";

  export let password = "";
  export let requirements: PasswordRequirementsPolicy | null = null;
  export let loading = false;
  export let error: string | null = null;
  export let title = "Password requirements";
  export let hint: string | null = "Avoid common words, patterns, and personal information.";
  export let loadingLabel = "Loading requirements...";

  $: effectiveRequirements = requirements;
  $: lengthMet = effectiveRequirements ? password.length >= effectiveRequirements.minLength : false;
  $: mixedCaseMet = !effectiveRequirements?.requireMixedCase || (/[a-z]/.test(password) && /[A-Z]/.test(password));
  $: digitMet = !effectiveRequirements?.requireDigit || /\d/.test(password);
  $: specialMet = !effectiveRequirements?.requireSpecial || /[^a-zA-Z0-9]/.test(password);
</script>

<div class="password-requirements" aria-live="polite">
  {#if loading}
    <p class="password-requirements__loading">{loadingLabel}</p>
  {:else if effectiveRequirements}
    <p class="password-requirements__title">{title}:</p>
    <ul class="password-requirements__list">
      <li class:password-requirements__item--met={lengthMet}>
        At least {effectiveRequirements.minLength} characters
      </li>
      {#if effectiveRequirements.requireMixedCase}
        <li class:password-requirements__item--met={mixedCaseMet}>
          Mix of uppercase and lowercase letters
        </li>
      {/if}
      {#if effectiveRequirements.requireDigit}
        <li class:password-requirements__item--met={digitMet}>
          At least one number
        </li>
      {/if}
      {#if effectiveRequirements.requireSpecial}
        <li class:password-requirements__item--met={specialMet}>
          At least one special character
        </li>
      {/if}
    </ul>
    {#if effectiveRequirements.description}
      <p class="password-requirements__description">{effectiveRequirements.description}</p>
    {/if}
    {#if hint}
      <p class="password-requirements__hint">{hint}</p>
    {/if}
  {:else if error}
    <p class="password-requirements__error">{error}</p>
  {/if}
</div>

<style>
  .password-requirements {
    margin-bottom: 1rem;
    padding: 1rem;
    border-radius: var(--poodle-radius-panel, 0.75rem);
    background: var(--poodle-color-background-subtle, color-mix(in srgb, var(--poodle-color-background-surface) 92%, var(--poodle-color-surface-muted) 8%));
    border: 0.0625rem solid var(--poodle-color-border-subtle, color-mix(in srgb, var(--poodle-color-border-default) 70%, transparent));
  }

  .password-requirements__title {
    margin: 0 0 0.5rem;
    font-weight: 600;
    color: var(--poodle-color-text-primary);
  }

  .password-requirements__loading,
  .password-requirements__description,
  .password-requirements__hint,
  .password-requirements__error {
    margin: 0;
    font-size: 0.875rem;
    line-height: 1.5;
    color: var(--poodle-color-text-secondary);
  }

  .password-requirements__error {
    color: var(--poodle-color-status-danger);
  }

  .password-requirements__description {
    margin-top: 0.75rem;
  }

  .password-requirements__hint {
    margin-top: 0.5rem;
    font-style: italic;
  }

  .password-requirements__list {
    margin: 0;
    padding: 0 0 0 1.25rem;
    list-style: disc;
  }

  .password-requirements__list li {
    color: var(--poodle-color-text-secondary);
    font-size: 0.875rem;
    line-height: 1.6;
    transition: color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .password-requirements__list li.password-requirements__item--met {
    color: var(--poodle-color-status-success);
  }
</style>
