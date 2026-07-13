import type { CSSProperties, ReactNode } from "react";

import "@poodle/styles/field-set.css";

import { scaleToSpace } from "./internal";
import type { SpaceScale } from "./types";

export interface FieldSetProps {
  legend?: string | null;
  description?: string | null;
  columns?: number;
  gap?: SpaceScale;
  span?: number | "full" | null;
  children?: ReactNode;
}

export function FieldSet({
  legend = null,
  description = null,
  columns = 1,
  gap = "md",
  span = null,
  children,
}: FieldSetProps) {
  const gridStyle: CSSProperties = {
    gridTemplateColumns: `repeat(${columns}, minmax(0, 1fr))`,
    rowGap: `calc(${scaleToSpace(gap)} + 0.5rem)`,
    columnGap: scaleToSpace(gap),
  };
  const rootStyle: CSSProperties | undefined = span
    ? span === "full"
      ? { gridColumn: "1 / -1" }
      : { gridColumn: `span ${span}` }
    : undefined;

  return (
    <fieldset className="poodle-fieldset" style={rootStyle}>
      {legend ? <legend className="poodle-fieldset__legend">{legend}</legend> : null}
      {description ? <p className="poodle-fieldset__description">{description}</p> : null}
      <div className="poodle-fieldset__fields" style={gridStyle}>
        {children}
      </div>
    </fieldset>
  );
}
