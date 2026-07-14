import "@poodle/styles/form-layout.css";

import type { CSSProperties, ReactNode } from "react";

import { Callout } from "./Callout";
import { FormActions } from "./FormActions";

export interface FormLayoutProps {
  columns?: number;
  error?: string | null;
  success?: string | null;
  fieldErrors?: Record<string, string> | null;
  description?: string | null;
  actions?: ReactNode;
  children?: ReactNode;
}

export function FormLayout({
  columns = 6,
  error = null,
  success = null,
  fieldErrors = null,
  description = null,
  actions,
  children,
}: FormLayoutProps) {
  const hasFieldErrors = Boolean(fieldErrors && Object.keys(fieldErrors).length > 0);

  return (
    <div className="poodle-form-layout">
      {description ? <p className="poodle-form-layout__description">{description}</p> : null}

      {error ? <Callout tone="danger" message={error} /> : null}

      {success ? <Callout tone="success" message={success} /> : null}

      {hasFieldErrors ? (
        <div className="poodle-form-layout__field-errors" role="alert" aria-live="polite">
          <p>Please fix the following errors:</p>
          <ul>
            {Object.entries(fieldErrors ?? {}).map(([field, message]) => (
              <li key={field}>
                <strong>{field}</strong>: {message}
              </li>
            ))}
          </ul>
        </div>
      ) : null}

      <div className="poodle-form-layout__grid" style={{ "--fl-columns": columns } as CSSProperties}>{children}</div>

      {actions ? (
        <div className="poodle-form-layout__actions">
          <FormActions>{actions}</FormActions>
        </div>
      ) : null}
    </div>
  );
}
