import {
  decorateCanonical,
  type CatalogueNavEntry,
} from "./catalogue-nav";
import {
  canonicalComponents,
  findCanonicalComponent,
} from "./generated/catalogue/catalogue";

export type ComponentEntry = CatalogueNavEntry & {
  packageName: string;
  hasSpecimen: boolean;
};

export const allComponents: ComponentEntry[] = canonicalComponents.map((component) => ({
  ...decorateCanonical(component),
  packageName: "@inflatable-cookie/poodle-svelte",
  hasSpecimen: true,
}));

export function findComponent(slug: string): ComponentEntry | undefined {
  return allComponents.find((component) => component.slug === slug);
}

export { findCanonicalComponent };
