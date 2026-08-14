import {
  decorateCanonical,
  type CatalogueNavEntry,
} from "../../../../svelte/preview/src/catalogue-nav";
import { canonicalComponents } from "../generated/catalogue/catalogue";

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

export const allComponents: ComponentEntry[] = canonicalComponents
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
