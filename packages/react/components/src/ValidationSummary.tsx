import "@inflatable-cookie/poodle-core/styles/validation-summary.css";
import type { AnnouncementMode, ValidationSummaryEntry } from "./types";

export interface ValidationSummaryProps {
  title?: string | null;
  entries?: ValidationSummaryEntry[];
  announceMode?: AnnouncementMode;
  includePending?: boolean;
}

export function ValidationSummary({ title = null, entries = [], announceMode = "polite", includePending = false }: ValidationSummaryProps) {
  const activeEntries = entries.filter((entry) =>
    entry.validationState === "invalid" || (includePending && entry.validationState === "pending")
  );
  if (activeEntries.length === 0) return null;
  const hasBlockingEntries = activeEntries.some((entry) => entry.validationState === "invalid");
  const role = announceMode === "assertive" ? "alert" : announceMode === "polite" ? "status" : undefined;
  const ariaLive = announceMode === "none" ? undefined : announceMode;

  return (
    <div className="poodle-validation-summary" data-state={hasBlockingEntries ? "blocking" : "pending"} role={role} aria-live={ariaLive}>
      {title ? <strong className="poodle-validation-summary__title">{title}</strong> : null}
      <ul className="poodle-validation-summary__list">
        {activeEntries.map((entry) => (
          <li key={entry.fieldId} className="poodle-validation-summary__entry" data-state={entry.validationState}>
            <a href={`#${entry.fieldId}`}>{entry.label}</a>
            <span>{entry.message}</span>
          </li>
        ))}
      </ul>
    </div>
  );
}
