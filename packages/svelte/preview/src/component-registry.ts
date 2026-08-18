import {
  decorateCanonical,
  type CatalogueNavEntry,
} from "./catalogue-nav";
import {
  canonicalComponents,
  findCanonicalComponent,
  type CanonicalComponent,
  type CatalogueCollectionId,
} from "./generated/catalogue/catalogue";

export type ComponentEntry = CatalogueNavEntry & {
  packageName: string;
  hasSpecimen: boolean;
};

/**
 * Web-only catalogue entries.
 *
 * The canonical catalogue is the portable component inventory and also feeds
 * the GPUI and Jetstream preview catalogues, so a web-only rendering
 * coordinator must not enter it (spec 068 / g14.024 fixed decision:
 * `MeterSurface` has no native counterpart — native backends already batch
 * meter nodes in their renderer scene). These entries exist so the Svelte and
 * React catalogues can document and exercise such surfaces without claiming a
 * portable component.
 */
export const webOnlyComponents: CanonicalComponent[] = [
  {
    slug: "meter-surface",
    displayName: "MeterSurface",
    description: "Web-only batched canvas renderer for high-count AudioMeter consoles.",
    section: "systems",
    family: "audio-music",
    kind: "display",
    collections: [] as const satisfies readonly CatalogueCollectionId[],
  },
];

export const allComponents: ComponentEntry[] = [...canonicalComponents, ...webOnlyComponents].map((component) => ({
  ...decorateCanonical(component),
  packageName: "@inflatable-cookie/poodle-svelte",
  hasSpecimen: true,
}));

export function findComponent(slug: string): ComponentEntry | undefined {
  return allComponents.find((component) => component.slug === slug);
}

export { findCanonicalComponent };
