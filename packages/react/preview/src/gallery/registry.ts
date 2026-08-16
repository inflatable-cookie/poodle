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

export const allComponents: ComponentEntry[] = [...canonicalComponents, ...webOnlyComponents].map(
  (component) => ({
    ...decorateCanonical(component),
    packageName: "@inflatable-cookie/poodle-react",
    hasSpecimen: true,
  }),
);

export function findComponent(
  componentSlug: string,
): ComponentEntry | undefined {
  return allComponents.find((component) => component.slug === componentSlug);
}
