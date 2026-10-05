<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/password-requirements.css";
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
      {#snippet statusIcon(met: boolean)}
        <span class="poodle-password-requirements__item-icon" aria-hidden="true">
          {#if met}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" width="1em" height="1em"><path d="M4.5 12.5l5 5L19.5 7" /></svg>
          {:else}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" width="1em" height="1em"><path d="M6 6l12 12M18 6L6 18" /></svg>
          {/if}
        </span>
      {/snippet}
      <li
        class:poodle-password-requirements__item--met={lengthMet}
        aria-label={`At least ${effectiveRequirements.minLength} characters — ${lengthMet ? "met" : "not met"}`}
      >
        {@render statusIcon(lengthMet)}At least {effectiveRequirements.minLength} characters
      </li>
      {#if effectiveRequirements.requireMixedCase}
        <li
          class:poodle-password-requirements__item--met={mixedCaseMet}
          aria-label={`Mix of uppercase and lowercase letters — ${mixedCaseMet ? "met" : "not met"}`}
        >
          {@render statusIcon(mixedCaseMet)}Mix of uppercase and lowercase letters
        </li>
      {/if}
      {#if effectiveRequirements.requireDigit}
        <li
          class:poodle-password-requirements__item--met={digitMet}
          aria-label={`At least one number — ${digitMet ? "met" : "not met"}`}
        >
          {@render statusIcon(digitMet)}At least one number
        </li>
      {/if}
      {#if effectiveRequirements.requireSpecial}
        <li
          class:poodle-password-requirements__item--met={specialMet}
          aria-label={`At least one special character — ${specialMet ? "met" : "not met"}`}
        >
          {@render statusIcon(specialMet)}At least one special character
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
    <p class="poodle-password-requirements__error" role="alert">{error}</p>
  {/if}
</div>

