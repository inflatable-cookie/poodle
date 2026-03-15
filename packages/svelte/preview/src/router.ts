export type SectionId = "primitives" | "composites" | "demo" | "tokens";

export type Route = {
  section: SectionId;
  component?: string;
  subscreen?: string;
};

const validSections: SectionId[] = ["primitives", "composites", "demo", "tokens"];

export function parseRoute(hash: string): Route {
  const raw = hash.replace(/^#/, "").trim();

  if (!raw) {
    return { section: "primitives" };
  }

  const segments = raw.split("/").filter(Boolean);
  const section = segments[0] as SectionId;

  if (!validSections.includes(section)) {
    return { section: "primitives" };
  }

  if (segments.length >= 2) {
    if (section === "demo") {
      return { section, subscreen: segments[1] };
    }

    return { section, component: segments[1] };
  }

  return { section };
}

export function buildHash(route: Route): string {
  if (route.component) {
    return `#${route.section}/${route.component}`;
  }

  if (route.subscreen) {
    return `#${route.section}/${route.subscreen}`;
  }

  return `#${route.section}`;
}

export function navigateTo(route: Route): void {
  if (typeof window === "undefined") {
    return;
  }

  const hash = buildHash(route);
  const currentHash = window.location.hash || "#primitives";

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
