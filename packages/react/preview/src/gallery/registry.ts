import {
  decorateCanonical,
  type CatalogueNavEntry,
} from "../../../../svelte/preview/src/catalogue-nav";
import { canonicalComponents } from "../generated/catalogue/catalogue";
// The web-only supplement is declared once, in the canonical (Svelte) preview
// registry, and mirrored here so both galleries catalogue the same surfaces
// without either entering the portable inventory.
import { webOnlyComponents } from "../../../../svelte/preview/src/component-registry";

export type ComponentEntry = CatalogueNavEntry & {
  packageName: string;
  hasSpecimen: boolean;
};

const svelteOnlyComponents = new Set(["AgentPlan", "AgentPlanRecord"]);
const embeddedOnlySpecimens = new Set([
  "AgentMessage",
  "ChangedFiles",
  "ToolCall",
  "ToolCallGroup",
]);

export const allComponents: ComponentEntry[] = [...canonicalComponents, ...webOnlyComponents]
  .filter((component) => !svelteOnlyComponents.has(component.displayName))
  .map((component) => ({
    ...decorateCanonical(component),
    packageName: "@inflatable-cookie/poodle-react",
    hasSpecimen: !embeddedOnlySpecimens.has(component.displayName),
  }));

export function findComponent(
  componentSlug: string,
): ComponentEntry | undefined {
  return allComponents.find((component) => component.slug === componentSlug);
}
