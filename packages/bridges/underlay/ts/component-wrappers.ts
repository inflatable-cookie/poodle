export type UnderlayWrapperPolicy = {
  underlayExport: string;
  publicOwner: "underlay";
  mayUsePoodleInternals: boolean;
  appImportsPoodleDirectly: false;
  migrationPressurePoints: string[];
  notes: string;
};

// This manifest makes the zero-leak bridge rule concrete. It is not a runtime
// component registry yet; it is a baseline contract artifact for adoption work.
export const underlayWrapperPolicies: UnderlayWrapperPolicy[] = [
  {
    underlayExport: "Button",
    publicOwner: "underlay",
    mayUsePoodleInternals: true,
    appImportsPoodleDirectly: false,
    migrationPressurePoints: [
      "variant and size translation",
      "focus-ring parity",
      "event-name compatibility",
    ],
    notes:
      "Underlay may wrap Poodle's button family internally, but apps should keep importing Underlay Button.",
  },
  {
    underlayExport: "SearchInput",
    publicOwner: "underlay",
    mayUsePoodleInternals: true,
    appImportsPoodleDirectly: false,
    migrationPressurePoints: [
      "query prop naming",
      "submit/cancel behavior",
      "result-shell composition remains Underlay-owned",
    ],
    notes:
      "Underlay may reuse Poodle TextInput type=\"search\" internals while preserving Underlay search APIs and surrounding browse workflows.",
  },
  {
    underlayExport: "Panel",
    publicOwner: "underlay",
    mayUsePoodleInternals: true,
    appImportsPoodleDirectly: false,
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
  "Underlay apps do not import Poodle packages directly.",
  "Underlay apps do not depend on Poodle component names.",
  "Underlay apps do not depend on Poodle token variable names.",
  "Wrapper layers preserve accessibility, focus, and keyboard semantics.",
] as const;
