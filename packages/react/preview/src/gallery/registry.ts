import {
  allComponents as canonicalComponents,
  type ComponentEntry as CanonicalComponentEntry,
  type ComponentTag as CanonicalComponentTag,
} from "../../../../svelte/preview/src/component-registry";

export type ComponentTag = CanonicalComponentTag;
export type ComponentEntry = CanonicalComponentEntry;

const svelteOnlyComponents = new Set(["AgentPlan", "AgentPlanRecord"]);
const embeddedOnlySpecimens = new Set([
  "AgentMessage",
  "ChangedFiles",
  "ToolCall",
  "ToolCallGroup",
]);

// Common catalogue metadata is canonical in the Svelte preview. React keeps
// only real runtime differences here; specimen-map.ts verifies every claimed
// standalone specimen at module load.
export const allComponents: ComponentEntry[] = canonicalComponents
  .filter((component) => !svelteOnlyComponents.has(component.displayName))
  .map((component) => ({
    ...component,
    packageName: "@inflatable-cookie/poodle-react",
    description:
      component.displayName === "ErrorBoundary"
        ? "React error boundary with retryable empty-state fallback."
        : component.description,
    hasSpecimen: embeddedOnlySpecimens.has(component.displayName)
      ? false
      : component.hasSpecimen,
  }));

export const tagLabels: Record<ComponentTag, string> = {
  control: "Controls",
  input: "Inputs",
  layout: "Layout",
  display: "Display",
  overlay: "Overlays",
  navigation: "Navigation",
  data: "Data",
  media: "Media",
  feedback: "Feedback",
  form: "Form",
  workstation: "Workstation",
};

export const tagOrder: ComponentTag[] = [
  "control",
  "input",
  "layout",
  "display",
  "overlay",
  "navigation",
  "data",
  "media",
  "feedback",
  "form",
  "workstation",
];

export function findComponent(componentSlug: string): ComponentEntry | undefined {
  return allComponents.find((component) => component.slug === componentSlug);
}

export function componentsByTag(): { tag: ComponentTag; label: string; items: ComponentEntry[] }[] {
  return tagOrder
    .map((tag) => ({
      tag,
      label: tagLabels[tag],
      items: allComponents.filter((component) => component.tag === tag),
    }))
    .filter((group) => group.items.length > 0);
}
