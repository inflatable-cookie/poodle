<script lang="ts">
  import type { PasswordRequirementsPolicy } from "./types";

  interface Props {
    password?: string;
    requirements?: PasswordRequirementsPolicy | null;
    loading?: boolean;
    error?: string | null;
    title?: string;
    hint?: string | null;
    loadingLabel?: string;
  }

  let {
    password = "",
    requirements = null,
    loading = false,
    error = null,
    title = "Password requirements",
    hint = "Avoid common words, patterns, and personal information.",
    loadingLabel = "Loading requirements...",
  }: Props = $props();

  const effectiveRequirements = $derived(requirements);
  const lengthMet = $derived(effectiveRequirements ? password.length >= effectiveRequirements.minLength : false);
  const mixedCaseMet = $derived(
    !effectiveRequirements?.requireMixedCase || (/[a-z]/.test(password) && /[A-Z]/.test(password))
  );
  const digitMet = $derived(!effectiveRequirements?.requireDigit || /\d/.test(password));
  const specialMet = $derived(!effectiveRequirements?.requireSpecial || /[^a-zA-Z0-9]/.test(password));
</script>

<div class="poodle-password-requirements" aria-live="polite">
  {#if loading}
    <p class="poodle-password-requirements__loading">{loadingLabel}</p>
  {:else if effectiveRequirements}
    <p class="poodle-password-requirements__title">{title}:</p>
    <ul class="poodle-password-requirements__list">
      <li class:poodle-password-requirements__item--met={lengthMet}>
        At least {effectiveRequirements.minLength} characters
      </li>
      {#if effectiveRequirements.requireMixedCase}
        <li class:poodle-password-requirements__item--met={mixedCaseMet}>
          Mix of uppercase and lowercase letters
        </li>
      {/if}
      {#if effectiveRequirements.requireDigit}
        <li class:poodle-password-requirements__item--met={digitMet}>
          At least one number
        </li>
      {/if}
      {#if effectiveRequirements.requireSpecial}
        <li class:poodle-password-requirements__item--met={specialMet}>
          At least one special character
        </li>
      {/if}
    </ul>
    {#if effectiveRequirements.description}
      <p class="poodle-password-requirements__description">{effectiveRequirements.description}</p>
    {/if}
    {#if hint}
      <p class="poodle-password-requirements__hint">{hint}</p>
    {/if}
  {:else if error}
    <p class="poodle-password-requirements__error">{error}</p>
  {/if}
</div>

<style>
  .poodle-password-requirements {
    padding: 1rem;
    border-radius: var(--poodle-radius-panel, 0.75rem);
    background: var(--poodle-color-background-subtle, color-mix(in srgb, var(--poodle-color-background-surface) 92%, var(--poodle-color-surface-muted) 8%));
    border: 0.0625rem solid var(--poodle-color-border-subtle, color-mix(in srgb, var(--poodle-color-border-default) 70%, transparent));
  }

  .poodle-password-requirements__title {
    margin: 0 0 0.5rem;
    font-weight: 600;
    color: var(--poodle-color-text-primary);
  }

  .poodle-password-requirements__loading,
  .poodle-password-requirements__description,
  .poodle-password-requirements__hint,
  .poodle-password-requirements__error {
    margin: 0;
    font-size: 0.875rem;
    line-height: 1.5;
    color: var(--poodle-color-text-secondary);
  }

  .poodle-password-requirements__error {
    color: var(--poodle-color-status-danger);
  }

  .poodle-password-requirements__description {
    margin-top: 0.75rem;
  }

  .poodle-password-requirements__hint {
    margin-top: 0.5rem;
    font-style: italic;
  }

  .poodle-password-requirements__list {
    margin: 0;
    padding: 0 0 0 1.25rem;
    list-style: disc;
  }

  .poodle-password-requirements__list li {
    color: var(--poodle-color-text-secondary);
    font-size: 0.875rem;
    line-height: 1.6;
    transition: color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-password-requirements__list li.poodle-password-requirements__item--met {
    color: var(--poodle-color-status-success);
  }
</style>
