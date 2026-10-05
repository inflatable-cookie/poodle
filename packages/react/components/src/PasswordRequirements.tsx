import "@inflatable-cookie/poodle-core/styles/password-requirements.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlSize, PasswordRequirementsPolicy, SemanticControlSizeRole } from "./types";

export interface PasswordRequirementsProps {
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

export function PasswordRequirements({
  password = "",
  requirements = null,
  loading = false,
  error = null,
  title = "Password requirements",
  hint = "Avoid common words, patterns, and personal information.",
  loadingLabel = "Loading requirements...",
  size = null,
  sizeRole = "control",
}: PasswordRequirementsProps) {
  const uiPresentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const lengthMet = requirements ? password.length >= requirements.minLength : false;
  const mixedCaseMet = !requirements?.requireMixedCase || (/[a-z]/.test(password) && /[A-Z]/.test(password));
  const digitMet = !requirements?.requireDigit || /\d/.test(password);
  const specialMet = !requirements?.requireSpecial || /[^a-zA-Z0-9]/.test(password);

  const statusIcon = (met: boolean) => (
    <span className="poodle-password-requirements__item-icon" aria-hidden="true">
      <svg
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2.5"
        strokeLinecap="round"
        strokeLinejoin="round"
        width="1em"
        height="1em"
      >
        {met ? <path d="M4.5 12.5l5 5L19.5 7" /> : <path d="M6 6l12 12M18 6L6 18" />}
      </svg>
    </span>
  );

  const item = (met: boolean, text: string) => (
    <li
      aria-label={`${text} — ${met ? "met" : "not met"}`}
      className={met ? "poodle-password-requirements__item--met" : undefined}
    >
      {statusIcon(met)}
      {text}
    </li>
  );

  return (
    <div className="poodle-password-requirements" aria-live="polite" data-size={resolvedSize}>
      {loading ? (
        <p className="poodle-password-requirements__loading">{loadingLabel}</p>
      ) : requirements ? (
        <>
          <p className="poodle-password-requirements__title">{title}:</p>
          <ul className="poodle-password-requirements__list">
            {item(lengthMet, `At least ${requirements.minLength} characters`)}
            {requirements.requireMixedCase ? item(mixedCaseMet, "Mix of uppercase and lowercase letters") : null}
            {requirements.requireDigit ? item(digitMet, "At least one number") : null}
            {requirements.requireSpecial ? item(specialMet, "At least one special character") : null}
          </ul>
          {requirements.description ? (
            <p className="poodle-password-requirements__description">{requirements.description}</p>
          ) : null}
          {hint ? <p className="poodle-password-requirements__hint">{hint}</p> : null}
        </>
      ) : error ? (
        <p className="poodle-password-requirements__error" role="alert">
          {error}
        </p>
      ) : null}
    </div>
  );
}
