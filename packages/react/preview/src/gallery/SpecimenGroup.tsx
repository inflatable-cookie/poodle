import type { ReactNode } from "react";
import { Eyebrow, Surface } from "@poodle/react";

export interface SpecimenGroupProps {
  label: string;
  /** When true, renders without Surface wrapper — for components that are surfaces themselves. */
  bare?: boolean;
  children?: ReactNode;
}

export function SpecimenGroup({ label, bare = false, children }: SpecimenGroupProps) {
  return (
    <div className="poodle-specimen-group">
      <Eyebrow>{label}</Eyebrow>
      {bare ? (
        <div className="poodle-specimen-group__content">{children}</div>
      ) : (
        <Surface tone="panel" border="subtle" padding="md">
          <div className="poodle-specimen-group__content">{children}</div>
        </Surface>
      )}
    </div>
  );
}
