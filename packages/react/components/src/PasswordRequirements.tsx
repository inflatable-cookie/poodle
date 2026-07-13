import "@poodle/styles/password-requirements.css";

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

  const item = (met: boolean, text: string) => (
    <li className={met ? "poodle-password-requirements__item--met" : undefined}>{text}</li>
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
        <p className="poodle-password-requirements__error">{error}</p>
      ) : null}
    </div>
  );
}
