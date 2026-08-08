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

