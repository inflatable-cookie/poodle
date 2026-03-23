export type UnderlayWrapperPolicy = {
  underlayExport: string;
  publicOwner: "underlay";
  mayUseFlintInternals: boolean;
  appImportsFlintDirectly: false;
  migrationPressurePoints: string[];
  notes: string;
};

// This manifest makes the zero-leak bridge rule concrete. It is not a runtime
// component registry yet; it is a baseline contract artifact for adoption work.
export const underlayWrapperPolicies: UnderlayWrapperPolicy[] = [
  {
    underlayExport: "Button",
    publicOwner: "underlay",
    mayUseFlintInternals: true,
    appImportsFlintDirectly: false,
    migrationPressurePoints: [
      "variant and size translation",
      "focus-ring parity",
      "event-name compatibility",
    ],
    notes:
      "Underlay may wrap Flint's button family internally, but apps should keep importing Underlay Button.",
  },
  {
    underlayExport: "SearchField",
    publicOwner: "underlay",
    mayUseFlintInternals: true,
    appImportsFlintDirectly: false,
    migrationPressurePoints: [
      "query prop naming",
      "submit/cancel behavior",
      "result-shell composition remains Underlay-owned",
    ],
    notes:
      "Underlay may reuse Flint SearchField internals while preserving Underlay search APIs and surrounding browse workflows.",
  },
  {
    underlayExport: "Panel",
    publicOwner: "underlay",
    mayUseFlintInternals: true,
    appImportsFlintDirectly: false,
    migrationPressurePoints: [
      "header slot translation",
      "panel identity naming",
      "workstation-only rollout sequencing",
    ],
    notes:
      "If Underlay adopts workstation shell surfaces later, public shell exports remain Underlay-owned.",
  },
];

export const underlayZeroLeakRules = [
  "Underlay apps do not import Flint packages directly.",
  "Underlay apps do not depend on Flint component names.",
  "Underlay apps do not depend on Flint token variable names.",
  "Wrapper layers preserve accessibility, focus, and keyboard semantics.",
] as const;
