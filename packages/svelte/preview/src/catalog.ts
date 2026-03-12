export type DocsSectionEntry = {
  id: string;
  title: string;
  eyebrow: string;
  layer: "foundation" | "composites" | "workstation" | "tokens";
  packageName: string;
  contractRoot: string;
  summary: string;
  exampleTypes: string[];
};

export type DocsFamilyEntry = {
  id: string;
  title: string;
  eyebrow: string;
  packageName: string;
  contractRoot: string;
  summary: string;
  adoptionBar: string;
  sectionIds: string[];
};

export const docsSections: DocsSectionEntry[] = [
  {
    id: "form-suite",
    title: "Form baseline and validation posture",
    eyebrow: "Foundation",
    layer: "foundation",
    packageName: "@pug/svelte-primitives",
    contractRoot: "docs/contracts/foundation/",
    summary: "Field wrappers, inputs, search, and action rows with validation and announcement posture.",
    exampleTypes: ["default", "invalid", "pending", "disabled"],
  },
  {
    id: "table-suite",
    title: "Selection, sorting, and bulk actions",
    eyebrow: "Composites",
    layer: "composites",
    packageName: "@pug/svelte-composites",
    contractRoot: "docs/contracts/composites/",
    summary: "Structured rows with visible-scope selection, sort semantics, row actions, and pagination context.",
    exampleTypes: ["sorting", "selection", "bulk action", "pagination"],
  },
  {
    id: "browse-suite",
    title: "Lists, grids, filters, and search depth",
    eyebrow: "Composites",
    layer: "composites",
    packageName: "@pug/svelte-composites",
    contractRoot: "docs/contracts/composites/",
    summary: "Browse shells that separate ready, loading, empty, and no-results posture across list and grid views.",
    exampleTypes: ["filtering", "ready", "empty", "no-results"],
  },
  {
    id: "detail-suite",
    title: "Detail display, cards, headers, and navigation",
    eyebrow: "Composites",
    layer: "composites",
    packageName: "@pug/svelte-composites",
    contractRoot: "docs/contracts/composites/",
    summary: "Readonly detail surfaces with breadcrumb context, page headers, cards, and section hierarchy.",
    exampleTypes: ["summary", "detail", "navigation", "empty/error"],
  },
  {
    id: "picker-suite",
    title: "Picker, relation, and selection workflows",
    eyebrow: "Composites",
    layer: "composites",
    packageName: "@pug/svelte-composites",
    contractRoot: "docs/contracts/composites/",
    summary: "Searchable attach-or-choose flows with selection summaries, commit/cancel rhythm, and keyboard traversal.",
    exampleTypes: ["single", "multiple", "search", "confirm/cancel"],
  },
  {
    id: "media-suite",
    title: "Preview framing, embeds, and fallback posture",
    eyebrow: "Composites",
    layer: "composites",
    packageName: "@pug/svelte-composites",
    contractRoot: "docs/contracts/composites/",
    summary: "Media surfaces that keep framing, metadata, and fallback meaning explicit without owning playback engines.",
    exampleTypes: ["image", "audio", "video", "document", "embed fallback"],
  },
  {
    id: "notification-suite",
    title: "Banners, toasts, skeletons, and remediation",
    eyebrow: "Hardening",
    layer: "composites",
    packageName: "@pug/svelte-primitives + @pug/svelte-composites",
    contractRoot: "docs/contracts/foundation/ + docs/contracts/composites/",
    summary: "Persistent and transient state surfaces with loading, remediation, and announcement posture.",
    exampleTypes: ["banner", "toast", "skeleton", "remediation"],
  },
  {
    id: "command-suite",
    title: "Palette search, grouped actions, and inline rediscovery",
    eyebrow: "Workstation",
    layer: "workstation",
    packageName: "@pug/svelte-workstation",
    contractRoot: "docs/contracts/workstation/",
    summary: "Modal and inline command discovery with grouped results, focus containment, and keyboard movement.",
    exampleTypes: ["modal", "inline", "loading", "error", "no-results"],
  },
  {
    id: "workspace-suite",
    title: "Headers, dock orchestration, and workspace shell posture",
    eyebrow: "Workstation",
    layer: "workstation",
    packageName: "@pug/svelte-workstation",
    contractRoot: "docs/contracts/workstation/",
    summary: "App and project headers, docks, tabs, split views, and host-owned layout snapshots.",
    exampleTypes: ["shell states", "docking", "resize", "persistence"],
  },
  {
    id: "token-summary-section",
    title: "Runtime-critical values",
    eyebrow: "Tokens",
    layer: "tokens",
    packageName: "@pug/svelte-tokens",
    contractRoot: "packages/tokens/",
    summary: "Live values pulled from the active emitted theme, density, and control-size overlays.",
    exampleTypes: ["semantic tokens", "theme overlays", "density", "control size"],
  },
  {
    id: "token-inspector",
    title: "Search the emitted token tree",
    eyebrow: "Tokens",
    layer: "tokens",
    packageName: "@pug/svelte-tokens",
    contractRoot: "packages/tokens/artifacts/",
    summary: "Searchable inspection of the emitted semantic token paths and their active computed values.",
    exampleTypes: ["search", "artifact provenance", "live readout"],
  },
];

export const docsFamilies: DocsFamilyEntry[] = [
  {
    id: "tokens",
    title: "Tokens and artifacts",
    eyebrow: "Artifacts",
    packageName: "@pug/svelte-tokens",
    contractRoot: "packages/tokens/",
    summary: "Adopters need to see emitted values, theme overlays, density modes, and control-size overlays in one place.",
    adoptionBar: "Show both live values and the artifact path they came from.",
    sectionIds: ["token-summary-section", "token-inspector"],
  },
  {
    id: "primitives",
    title: "Foundation primitives",
    eyebrow: "Foundations",
    packageName: "@pug/svelte-primitives",
    contractRoot: "docs/contracts/foundation/",
    summary: "Primitives need behavior-first examples with error, pending, disabled, and focus posture visible.",
    adoptionBar: "Every primitive family needs at least one stateful example, not only a default specimen.",
    sectionIds: ["form-suite", "notification-suite"],
  },
  {
    id: "composites",
    title: "Product composites",
    eyebrow: "Composites",
    packageName: "@pug/svelte-composites",
    contractRoot: "docs/contracts/composites/",
    summary: "Higher-order surfaces need real browse, detail, picker, media, and remediation examples that prove composition rules.",
    adoptionBar: "Examples must show ready, loading, empty, and recovery posture where the contract owns them.",
    sectionIds: ["table-suite", "browse-suite", "detail-suite", "picker-suite", "media-suite", "notification-suite"],
  },
  {
    id: "workstation",
    title: "Workstation shells",
    eyebrow: "Workstation",
    packageName: "@pug/svelte-workstation",
    contractRoot: "docs/contracts/workstation/",
    summary: "Workstation adopters need command discovery, shell hierarchy, dock orchestration, and persistence posture visible together.",
    adoptionBar: "Examples must prove keyboard, focus, shell state, and layout semantics instead of only showing chrome.",
    sectionIds: ["command-suite", "workspace-suite"],
  },
];

export const docsAdoptionChecklist = [
  "Show where a component lives in contracts, package code, and the live example surface.",
  "Expose at least one stateful example for every adopted family, not only a default happy path.",
  "Keep token provenance visible so theme and density changes remain inspectable, not inferred.",
  "Keep accessibility and keyboard posture visible in the same example surface used for visual review.",
];
