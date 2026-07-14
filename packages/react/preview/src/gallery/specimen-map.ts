import type { ComponentType } from "react";

import { ButtonSpecimen } from "./specimens/ButtonSpecimen";
import { CheckboxSpecimen } from "./specimens/CheckboxSpecimen";
import { PillSpecimen } from "./specimens/PillSpecimen";
import { SwitchSpecimen } from "./specimens/SwitchSpecimen";

/**
 * Slug → specimen component, mirroring the Svelte preview's specimens/registry.ts.
 * Slugs match the component-registry so the two galleries route identically.
 * Grows as per-component specimens are ported.
 */
export const specimenMap: Record<string, ComponentType> = {
  button: ButtonSpecimen,
  checkbox: CheckboxSpecimen,
  pill: PillSpecimen,
  switch: SwitchSpecimen,
};
