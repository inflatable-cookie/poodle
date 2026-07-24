import {
  buildPreviewUrl,
  docsNavigationSections,
  normalizePreviewState,
  previewStateDefaults,
  type DocsSectionId,
  type ReviewRoutePreset,
} from "./parity";

export type AccessibilityAuditStatus =
  | "explicit"
  | "hybrid"
  | "manual"
  | "blocked"
  | "not-applicable";

export type AccessibilityAuditAreaStatus = {
  semantics: AccessibilityAuditStatus;
  focus: AccessibilityAuditStatus;
  keyboard: AccessibilityAuditStatus;
  announcements: AccessibilityAuditStatus;
  gpui: AccessibilityAuditStatus;
};

export type AccessibilityAuditTarget = {
  sectionId: DocsSectionId;
  title: string;
  layer: string;
  packageName: string;
  contractRoot: string;
  summary: string;
  auditAreas: AccessibilityAuditAreaStatus;
  automatedChecks: string[];
  manualChecks: string[];
  gpuiDeltaNotes: string[];
  blockerNotes: string[];
  reviewRoutes: ReviewRoutePreset[];
};

type AccessibilityAuditConfig = Omit<
  AccessibilityAuditTarget,
  "sectionId" | "title" | "layer" | "packageName" | "contractRoot" | "summary"
>;

function route(
  sectionId: DocsSectionId,
  label: string,
  note: string,
  overrides: Partial<Omit<ReviewRoutePreset["state"], "sectionId">> = {},
): ReviewRoutePreset {
  return {
    id: `${sectionId}-${label.toLowerCase().replace(/[^a-z0-9]+/g, "-")}`,
    label,
    note,
    state: normalizePreviewState({ ...previewStateDefaults, sectionId, ...overrides }),
  };
}

const gpuiManualProofRequired = [
  "GPUI proof posture is explicit for this section now, but mounted native review and assistive-technology traces are still required before it can be promoted beyond manual evidence.",
];

const accessibilityAuditConfig: Record<DocsSectionId, AccessibilityAuditConfig> = {
  "catalog-hub": {
    auditAreas: {
      semantics: "explicit",
      focus: "hybrid",
      keyboard: "hybrid",
      announcements: "not-applicable",
      gpui: "not-applicable",
    },
    automatedChecks: [
      "catalog-hub review routes stay deep-linkable",
      "section navigation remains present in the generated docs surface",
    ],
    manualChecks: [
      "section browser stays usable with keyboard-only navigation",
      "landing copy still exposes package ownership and adoption guidance clearly",
    ],
    gpuiDeltaNotes: [
      "This section is a browser review index rather than a candidate shared GPUI component surface.",
    ],
    blockerNotes: [],
    reviewRoutes: [
      route("catalog-hub", "baseline", "Primary catalog landing review."),
      route("catalog-hub", "iceberg", "Iceberg-theme catalog and navigation review.", {
        theme: "iceberg",
        density: "comfortable",
      }),
    ],
  },
  "form-suite": {
    auditAreas: {
      semantics: "explicit",
      focus: "explicit",
      keyboard: "hybrid",
      announcements: "hybrid",
      gpui: "manual",
    },
    automatedChecks: [
      "invalid, pending, and disabled form review routes are stable",
      "control-size and theme state remain deep-linkable for repeat audits",
    ],
    manualChecks: [
      "screen-reader announcement quality for help, error, and pending copy",
      "caret, selection, and IME behavior remain intact across text entry surfaces",
    ],
    gpuiDeltaNotes: [
      "Browser form semantics come from native inputs, while GPUI will need explicit accessible tree nodes plus announcement wiring.",
      "Validation messaging parity must be proven separately for native focus movement and error exposure.",
    ],
    blockerNotes: [
      ...gpuiManualProofRequired,
      "Field widgets, validation summaries, and remediation flows still need mounted GPUI focus and announcement traces.",
    ],
    reviewRoutes: [
      route("form-suite", "baseline", "Default form suite audit."),
      route("form-suite", "iceberg", "Iceberg-theme contrast and field grouping audit.", {
        theme: "iceberg",
        density: "comfortable",
      }),
      route("form-suite", "small", "Minimum-size input and label audit.", {
        controlSize: "sm",
      }),
    ],
  },
  "table-suite": {
    auditAreas: {
      semantics: "explicit",
      focus: "hybrid",
      keyboard: "hybrid",
      announcements: "manual",
      gpui: "manual",
    },
    automatedChecks: [
      "table review routes stay stable for default and light-theme audits",
      "selection and sorting examples remain attached to the table section",
    ],
    manualChecks: [
      "header and row action keyboard flow stays understandable",
      "sort or selection state changes are announced clearly enough for assistive technology",
    ],
    gpuiDeltaNotes: [
      "HTML table semantics do not translate directly to GPUI; native structured data surfaces will require explicit accessible relationships.",
      "Row focus and selection narration cannot be claimed cross-runtime yet.",
    ],
    blockerNotes: [
      ...gpuiManualProofRequired,
      "Row narration, sort-state exposure, and selection feedback still need mounted GPUI table evidence.",
    ],
    reviewRoutes: [
      route("table-suite", "baseline", "Primary table interaction audit."),
      route("table-suite", "iceberg", "Iceberg-theme table review.", {
        theme: "iceberg",
        density: "comfortable",
      }),
    ],
  },
  "browse-suite": {
    auditAreas: {
      semantics: "hybrid",
      focus: "hybrid",
      keyboard: "hybrid",
      announcements: "manual",
      gpui: "manual",
    },
    automatedChecks: [
      "browse routes remain stable for ready and empty-state review",
      "catalog registration still maps browse ownership to the correct contracts",
    ],
    manualChecks: [
      "empty, loading, and no-results transitions stay understandable without relying on layout alone",
      "filter and search affordances maintain predictable focus order",
    ],
    gpuiDeltaNotes: [
      "DOM list semantics and browser search fields currently do part of the accessibility work automatically.",
      "Native browse-shell parity will need explicit narration for result-count and progressive-loading state.",
    ],
    blockerNotes: [
      ...gpuiManualProofRequired,
      "Result-count changes, loading posture, and filter-focus recovery remain native browse-shell review work.",
    ],
    reviewRoutes: [
      route("browse-suite", "baseline", "Default browse audit."),
      route("browse-suite", "iceberg", "Iceberg-theme browse-shell review.", {
        theme: "iceberg",
        density: "comfortable",
      }),
    ],
  },
  "detail-suite": {
    auditAreas: {
      semantics: "explicit",
      focus: "hybrid",
      keyboard: "hybrid",
      announcements: "manual",
      gpui: "manual",
    },
    automatedChecks: [
      "detail routes remain stable for metadata and action review",
      "theme and density changes keep the same detail audit entry points",
    ],
    manualChecks: [
      "metadata rows remain understandable when scanned by assistive technology",
      "header actions and companion panes keep a predictable focus order",
    ],
    gpuiDeltaNotes: [
      "Structured metadata is explicit in Svelte through DOM text order, but GPUI will need deliberate accessible grouping.",
      "Cross-pane navigation expectations remain unproven natively.",
    ],
    blockerNotes: [
      ...gpuiManualProofRequired,
      "Metadata narration and cross-pane focus order still need mounted GPUI detail-shell verification.",
    ],
    reviewRoutes: [
      route("detail-suite", "baseline", "Primary detail layout audit."),
      route("detail-suite", "iceberg", "Iceberg-theme detail contrast audit.", {
        theme: "iceberg",
        density: "comfortable",
      }),
    ],
  },
  "picker-suite": {
    auditAreas: {
      semantics: "hybrid",
      focus: "hybrid",
      keyboard: "hybrid",
      announcements: "hybrid",
      gpui: "manual",
    },
    automatedChecks: [
      "picker routes remain stable for single, multiple, and modal review",
      "preview-state switches remain deep-linkable for repeat audits",
    ],
    manualChecks: [
      "selection change feedback remains understandable for keyboard-only and screen-reader users",
      "popover or modal picker variants keep focus contained and recover correctly on close",
    ],
    gpuiDeltaNotes: [
      "Browser overlays rely on DOM focus scope patterns that do not automatically map to GPUI.",
      "Selection announcement and active-descendant style behavior still need native proof.",
    ],
    blockerNotes: [
      ...gpuiManualProofRequired,
      "Popover or modal focus containment, active-item narration, and close recovery still require mounted GPUI picker evidence.",
    ],
    reviewRoutes: [
      route("picker-suite", "baseline", "Primary picker audit."),
      route("picker-suite", "iceberg", "Iceberg-theme picker review.", {
        theme: "iceberg",
        density: "comfortable",
      }),
      route("picker-suite", "small", "Small control-size picker audit.", {
        controlSize: "sm",
      }),
    ],
  },
  "media-suite": {
    auditAreas: {
      semantics: "hybrid",
      focus: "hybrid",
      keyboard: "manual",
      announcements: "manual",
      gpui: "manual",
    },
    automatedChecks: [
      "media preview routes remain stable for ready and empty-state review",
      "embed and media preview examples stay attached to the media section",
    ],
    manualChecks: [
      "thumbnail strips and large preview surfaces remain keyboard-discoverable",
      "fallback or unavailable preview states remain understandable without relying on art direction alone",
    ],
    gpuiDeltaNotes: [
      "Media previews depend on browser-native image and document affordances that are not equivalent to GPUI surfaces.",
      "Playback, focus, and preview fallback narration remain native-runtime debt.",
    ],
    blockerNotes: [
      ...gpuiManualProofRequired,
      "Media playback, document preview, and unavailable-preview fallback narration remain runtime-specific GPUI debt.",
    ],
    reviewRoutes: [
      route("media-suite", "baseline", "Primary media review."),
      route("media-suite", "iceberg", "Iceberg-theme media audit.", {
        theme: "iceberg",
        density: "comfortable",
      }),
    ],
  },
  "notification-suite": {
    auditAreas: {
      semantics: "explicit",
      focus: "hybrid",
      keyboard: "explicit",
      announcements: "hybrid",
      gpui: "manual",
    },
    automatedChecks: [
      "notification routes remain stable for banner and toast review",
      "tone-switching review state stays deep-linkable",
    ],
    manualChecks: [
      "banner, toast, and remediation actions expose the right urgency without over-announcing",
      "dismiss and follow-up actions stay reachable without focus traps",
    ],
    gpuiDeltaNotes: [
      "DOM live-region behavior and toast timing do not imply equivalent native announcement behavior.",
      "Notification queue narration will need runtime-specific proof in GPUI.",
    ],
    blockerNotes: [
      ...gpuiManualProofRequired,
      "Native toast timing, queue narration, and escalation between polite and assertive announcements still need mounted GPUI review.",
    ],
    reviewRoutes: [
      route("notification-suite", "baseline", "Primary notification audit."),
      route("notification-suite", "iceberg", "Iceberg-theme notification tone audit.", {
        theme: "iceberg",
        density: "comfortable",
      }),
    ],
  },
  "command-suite": {
    auditAreas: {
      semantics: "hybrid",
      focus: "hybrid",
      keyboard: "explicit",
      announcements: "hybrid",
      gpui: "manual",
    },
    automatedChecks: [
      "command review routes remain stable for modal and inline discovery",
      "review URLs continue to target the command section directly",
    ],
    manualChecks: [
      "modal launcher, list navigation, and close recovery remain fully keyboard-usable",
      "result count, active item, and execution feedback remain understandable to assistive technology",
    ],
    gpuiDeltaNotes: [
      "Browser dialog, list, and focus-lock patterns do not establish native command palette accessibility automatically.",
      "Keyboard discovery and announcement behavior need standalone GPUI proof.",
    ],
    blockerNotes: [
      ...gpuiManualProofRequired,
      "Modal focus recovery, active-result narration, and execution feedback remain command-palette-specific GPUI review work.",
    ],
    reviewRoutes: [
      route("command-suite", "baseline", "Primary command discovery audit."),
      route("command-suite", "iceberg", "Iceberg-theme command review.", {
        theme: "iceberg",
        density: "comfortable",
      }),
    ],
  },
  "workspace-suite": {
    auditAreas: {
      semantics: "hybrid",
      focus: "hybrid",
      keyboard: "hybrid",
      announcements: "manual",
      gpui: "manual",
    },
    automatedChecks: [
      "workspace routes remain stable for dock and persistence review",
      "serialized layout review stays attached to the workspace section",
    ],
    manualChecks: [
      "panel traversal, dock controls, and persistence readout remain understandable without pointer-only assumptions",
      "large multi-panel examples do not hide the currently active region or action target",
    ],
    gpuiDeltaNotes: [
      "Complex shell focus scopes and docking semantics are likely to diverge most between DOM and GPUI.",
      "Native accessible region naming and movement remain unproven for this section.",
    ],
    blockerNotes: [
      ...gpuiManualProofRequired,
      "Dock-region naming, multi-panel traversal, and layout-change narration still require mounted GPUI shell evidence.",
    ],
    reviewRoutes: [
      route("workspace-suite", "baseline", "Primary workspace shell audit."),
      route("workspace-suite", "iceberg", "Iceberg-theme workspace review.", {
        theme: "iceberg",
        density: "comfortable",
      }),
    ],
  },
  "token-summary-section": {
    auditAreas: {
      semantics: "explicit",
      focus: "explicit",
      keyboard: "explicit",
      announcements: "not-applicable",
      gpui: "not-applicable",
    },
    automatedChecks: [
      "token summary review routes remain stable across theme and density changes",
      "token family registration remains catalog-linked",
    ],
    manualChecks: [
      "token summary cards stay readable at all theme and density combinations",
      "chip and metadata ordering still exposes hierarchy clearly for quick inspection",
    ],
    gpuiDeltaNotes: [
      "This section is a docs inspection surface rather than a shared runtime component contract.",
    ],
    blockerNotes: [],
    reviewRoutes: [
      route("token-summary-section", "baseline", "Primary token summary audit."),
      route("token-summary-section", "iceberg", "Iceberg-theme token summary review.", {
        theme: "iceberg",
        density: "comfortable",
      }),
    ],
  },
  "token-inspector": {
    auditAreas: {
      semantics: "explicit",
      focus: "hybrid",
      keyboard: "hybrid",
      announcements: "manual",
      gpui: "not-applicable",
    },
    automatedChecks: [
      "token inspector route remains stable for search-led audits",
      "inspector continues to expose token provenance and computed values",
    ],
    manualChecks: [
      "search, focus movement, and result scanning remain usable without pointer dependence",
      "value and provenance copy remain understandable when the search surface filters aggressively",
    ],
    gpuiDeltaNotes: [
      "The inspector is a preview-only documentation utility and is not a target shared GPUI surface.",
    ],
    blockerNotes: [],
    reviewRoutes: [
      route("token-inspector", "baseline", "Primary token inspector audit."),
      route("token-inspector", "iceberg", "Iceberg-theme inspector review.", {
        theme: "iceberg",
        density: "comfortable",
      }),
    ],
  },
};

export const accessibilityAuditBoundary = {
  automated: [
    "Every docs section must have a machine-readable accessibility audit target.",
    "Each target must list audit-area status, automated checks, manual checks, and stable review routes.",
    "GPUI manual or blocked targets must name the remaining blockers instead of implying native proof from Svelte behavior.",
  ],
  manual: [
    "Screen-reader announcement quality, keyboard rhythm, and focus recovery still require human review.",
    "Cross-runtime deltas between browser semantics and GPUI native accessibility must be documented explicitly.",
    "Preview coverage and crate tests can seed accessibility review, but they cannot claim native assistive-technology conformance without runtime evidence.",
  ],
} as const;

export const accessibilityAuditTargets: AccessibilityAuditTarget[] = docsNavigationSections.map((entry) => {
  const config = accessibilityAuditConfig[entry.id];

  return {
    sectionId: entry.id,
    title: entry.title,
    layer: entry.layer,
    packageName: entry.packageName,
    contractRoot: entry.contractRoot,
    summary: entry.summary,
    auditAreas: config.auditAreas,
    automatedChecks: config.automatedChecks,
    manualChecks: config.manualChecks,
    gpuiDeltaNotes: config.gpuiDeltaNotes,
    blockerNotes: config.blockerNotes,
    reviewRoutes: config.reviewRoutes,
  };
});

export function buildAccessibilityAuditUrl(routePreset: ReviewRoutePreset): string {
  return buildPreviewUrl(routePreset.state);
}
