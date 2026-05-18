<script lang="ts">
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlSize, SemanticControlSizeRole, PasswordRequirementsPolicy } from "./types";

  interface Props {
    password?: string;
    requirements?: PasswordRequirementsPolicy | null;
    loading?: boolean;
    error?: string | null;
    title?: string;
    hint?: string | null;
    loadingLabel?: string;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
  }

  let {
    password = "",
    requirements = null,
    loading = false,
    error = null,
    title = "Password requirements",
    hint = "Avoid common words, patterns, and personal information.",
    loadingLabel = "Loading requirements...",
    size = null,
    sizeRole = "control",
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const effectiveRequirements = $derived(requirements);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const lengthMet = $derived(effectiveRequirements ? password.length >= effectiveRequirements.minLength : false);
  const mixedCaseMet = $derived(
    !effectiveRequirements?.requireMixedCase || (/[a-z]/.test(password) && /[A-Z]/.test(password))
  );
  const digitMet = $derived(!effectiveRequirements?.requireDigit || /\d/.test(password));
  const specialMet = $derived(!effectiveRequirements?.requireSpecial || /[^a-zA-Z0-9]/.test(password));
</script>

<div class="poodle-password-requirements" aria-live="polite" data-size={resolvedSize}>
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
    --poodle-password-requirements-padding: 1rem;
    --poodle-password-requirements-title-size: 1rem;
    --poodle-password-requirements-body-size: 0.875rem;
    --poodle-password-requirements-list-line-height: 1.6;
    --poodle-password-requirements-list-indent: 1.25rem;
    --poodle-password-requirements-description-gap: 0.75rem;
    --poodle-password-requirements-hint-gap: 0.5rem;
    padding: 1rem;
    border-radius: var(--poodle-radius-panel, 0.75rem);
    background: var(--poodle-color-background-subtle, color-mix(in srgb, var(--poodle-color-background-surface) 92%, var(--poodle-color-surface-muted) 8%));
    border: 0.0625rem solid var(--poodle-color-border-subtle, color-mix(in srgb, var(--poodle-color-border-default) 70%, transparent));
    padding: var(--poodle-password-requirements-padding);
  }

  .poodle-password-requirements__title {
    margin: 0 0 0.5rem;
    font-weight: 600;
    color: var(--poodle-color-text-primary);
    font-size: var(--poodle-password-requirements-title-size);
    line-height: 1.4;
  }

  .poodle-password-requirements__loading,
  .poodle-password-requirements__description,
  .poodle-password-requirements__hint,
  .poodle-password-requirements__error {
    margin: 0;
    font-size: var(--poodle-password-requirements-body-size);
    line-height: 1.5;
    color: var(--poodle-color-text-secondary);
  }

  .poodle-password-requirements__error {
    color: var(--poodle-color-status-danger);
  }

  .poodle-password-requirements__description {
    margin-top: var(--poodle-password-requirements-description-gap);
  }

  .poodle-password-requirements__hint {
    margin-top: var(--poodle-password-requirements-hint-gap);
    font-style: italic;
  }

  .poodle-password-requirements__list {
    margin: 0;
    padding: 0 0 0 var(--poodle-password-requirements-list-indent);
    list-style: disc;
  }

  .poodle-password-requirements__list li {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-password-requirements-body-size);
    line-height: var(--poodle-password-requirements-list-line-height);
    transition: color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-password-requirements__list li.poodle-password-requirements__item--met {
    color: var(--poodle-color-status-success);
  }

  .poodle-password-requirements[data-size="xs"] {
    --poodle-password-requirements-padding: 0.75rem;
    --poodle-password-requirements-title-size: 0.875rem;
    --poodle-password-requirements-body-size: 0.75rem;
    --poodle-password-requirements-list-line-height: 1.5;
    --poodle-password-requirements-list-indent: 1rem;
    --poodle-password-requirements-description-gap: 0.625rem;
    --poodle-password-requirements-hint-gap: 0.375rem;
  }

  .poodle-password-requirements[data-size="sm"] {
    --poodle-password-requirements-padding: 0.875rem;
    --poodle-password-requirements-title-size: 0.9375rem;
    --poodle-password-requirements-body-size: 0.8125rem;
    --poodle-password-requirements-list-line-height: 1.55;
    --poodle-password-requirements-list-indent: 1.125rem;
    --poodle-password-requirements-description-gap: 0.6875rem;
    --poodle-password-requirements-hint-gap: 0.4375rem;
  }

  .poodle-password-requirements[data-size="md"] {
    --poodle-password-requirements-padding: 1rem;
    --poodle-password-requirements-title-size: 1rem;
    --poodle-password-requirements-body-size: 0.875rem;
    --poodle-password-requirements-list-line-height: 1.6;
    --poodle-password-requirements-list-indent: 1.25rem;
    --poodle-password-requirements-description-gap: 0.75rem;
    --poodle-password-requirements-hint-gap: 0.5rem;
  }

  .poodle-password-requirements[data-size="lg"] {
    --poodle-password-requirements-padding: 1.125rem;
    --poodle-password-requirements-title-size: 1.0625rem;
    --poodle-password-requirements-body-size: 0.9375rem;
    --poodle-password-requirements-list-line-height: 1.65;
    --poodle-password-requirements-list-indent: 1.375rem;
    --poodle-password-requirements-description-gap: 0.8125rem;
    --poodle-password-requirements-hint-gap: 0.5625rem;
  }

  .poodle-password-requirements[data-size="xl"] {
    --poodle-password-requirements-padding: 1.25rem;
    --poodle-password-requirements-title-size: 1.125rem;
    --poodle-password-requirements-body-size: 1rem;
    --poodle-password-requirements-list-line-height: 1.7;
    --poodle-password-requirements-list-indent: 1.5rem;
    --poodle-password-requirements-description-gap: 0.875rem;
    --poodle-password-requirements-hint-gap: 0.625rem;
  }
</style>
