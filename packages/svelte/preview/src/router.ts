export type SectionId = "components" | "tokens";

export type Route = {
  section: SectionId;
  component?: string;
};

const validSections: SectionId[] = ["components", "tokens"];

/** Legacy section aliases that redirect to the unified components section. */
const sectionAliases: Record<string, SectionId> = {
  primitives: "components",
  composites: "components",
};

export function parseRoute(hash: string): Route {
  const raw = hash.replace(/^#/, "").trim();

  if (!raw) {
    return { section: "components" };
  }

  const segments = raw.split("/").filter(Boolean);
  let section = segments[0] as SectionId;

  if (sectionAliases[section]) {
    section = sectionAliases[section];
  }

  if (!validSections.includes(section)) {
    return { section: "components" };
  }

  if (segments.length >= 2) {
    return { section, component: segments[1] };
  }

  return { section };
}

export function buildHash(route: Route): string {
  if (route.component) {
    return `#${route.section}/${route.component}`;
  }

  return `#${route.section}`;
}

export function navigateTo(route: Route): void {
  if (typeof window === "undefined") {
    return;
  }

  const hash = buildHash(route);
  const currentHash = window.location.hash || "#components";

  if (hash !== currentHash) {
    window.location.hash = hash;
  }
}

export function buildPreviewUrl(
  route: Route,
  params: { theme: string; density: string; controlSize: string },
  basePath = "/",
): string {
  const searchParams = new URLSearchParams({
    theme: params.theme,
    density: params.density,
    controlSize: params.controlSize,
  });

  return `${basePath}?${searchParams.toString()}${buildHash(route)}`;
}
