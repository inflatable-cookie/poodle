import type { ReactNode } from "react";
import { Eyebrow, Surface, Text } from "@inflatable-cookie/poodle-react";

export interface SpecimenGroupProps {
  label: string;
  description?: string;
  /** When true, renders without Surface wrapper — for components that are surfaces themselves. */
  bare?: boolean;
  children?: ReactNode;
}

export function SpecimenGroup({ label, description, bare = false, children }: SpecimenGroupProps) {
  return (
    <div className="poodle-specimen-group">
      <Eyebrow>{label}</Eyebrow>
      {description ? (
        <Text tone="muted" size="sm" spacing="compact">{description}</Text>
      ) : null}
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
