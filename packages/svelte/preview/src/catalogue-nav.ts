import {
  canonicalComponents,
  catalogueCollections,
  catalogueFamilies,
  catalogueKinds,
  catalogueSections,
  type CanonicalComponent,
  type CatalogueFamilyId,
  type CatalogueSectionId,
} from "./generated/catalogue/catalogue";

export type CatalogueNavEntry = CanonicalComponent & {
  familyLabel: string;
  kindLabel: string;
  sectionLabel: string;
  collectionLabels: string[];
};

export type CatalogueFamilyGroup<
  T extends CatalogueNavEntry = CatalogueNavEntry,
> = {
  id: CatalogueFamilyId;
  label: string;
  section: CatalogueSectionId;
  items: T[];
};

export type CatalogueSectionGroup<
  T extends CatalogueNavEntry = CatalogueNavEntry,
> = {
  id: CatalogueSectionId;
  label: string;
  families: CatalogueFamilyGroup<T>[];
};

const familyById = new Map(
  catalogueFamilies.map((family) => [family.id, family]),
);
const kindById = new Map(catalogueKinds.map((kind) => [kind.id, kind]));
const sectionById = new Map(
  catalogueSections.map((section) => [section.id, section]),
);
// Empty collection vocabularies emit `CatalogueCollectionId = never`; keep the
// lookup string-keyed so decorate/search still typecheck before any collection exists.
const collectionById = new Map<string, { id: string; label: string }>(
  (catalogueCollections as ReadonlyArray<{ id: string; label: string }>).map(
    (collection) => [collection.id, collection],
  ),
);

export function decorateCanonical<T extends CanonicalComponent>(
  component: T,
): T & CatalogueNavEntry {
  const family = familyById.get(component.family);
  const kind = kindById.get(component.kind);
  const section = sectionById.get(component.section);
  if (!family || !kind || !section) {
    throw new Error(`incomplete catalogue decoration for ${component.slug}`);
  }
  return {
    ...component,
    familyLabel: family.label,
    kindLabel: kind.label,
    sectionLabel: section.label,
    collectionLabels: (component.collections as readonly string[]).map((id) => {
      const collection = collectionById.get(id);
      if (!collection) {
        throw new Error(`unknown collection ${id} on ${component.slug}`);
      }
      return collection.label;
    }),
  };
}

export function matchesCatalogueSearch(
  entry: CatalogueNavEntry,
  query: string,
): boolean {
  const needle = query.trim().toLowerCase();
  if (!needle) return true;
  return (
    entry.displayName.toLowerCase().includes(needle) ||
    entry.description.toLowerCase().includes(needle) ||
    entry.familyLabel.toLowerCase().includes(needle) ||
    entry.kindLabel.toLowerCase().includes(needle) ||
    entry.collectionLabels.some((label) => label.toLowerCase().includes(needle))
  );
}

export function componentsByFamily<T extends CatalogueNavEntry>(
  components: readonly T[],
): CatalogueFamilyGroup<T>[] {
  const allowed = new Set(components.map((component) => component.slug));
  return catalogueFamilies
    .map((family) => ({
      id: family.id,
      label: family.label,
      section: family.section,
      items: components.filter(
        (component) =>
          component.family === family.id && allowed.has(component.slug),
      ),
    }))
    .filter((group) => group.items.length > 0);
}

export function componentsBySection<T extends CatalogueNavEntry>(
  components: readonly T[],
): CatalogueSectionGroup<T>[] {
  const families = componentsByFamily(components);
  return catalogueSections
    .map((section) => ({
      id: section.id,
      label: section.label,
      families: families.filter((family) => family.section === section.id),
    }))
    .filter((section) => section.families.length > 0);
}

export function isFamilyDisclosed(
  familyId: CatalogueFamilyId,
  activeSlug: string | undefined,
  userDisclosure: ReadonlyMap<string, boolean>,
  components: readonly CatalogueNavEntry[],
): boolean {
  const userChoice = userDisclosure.get(familyId);
  if (userChoice !== undefined) return userChoice;
  if (!activeSlug) return false;
  const active = components.find((component) => component.slug === activeSlug);
  return active?.family === familyId;
}

export {
  canonicalComponents,
  catalogueFamilies,
  catalogueSections,
  catalogueKinds,
};
