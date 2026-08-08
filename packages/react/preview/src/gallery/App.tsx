import { useEffect, useRef, useState, type CSSProperties } from "react";
import { Pill, Tabs, IconProvider, UiPresentationProvider, type IconSet, type TabItem } from "@inflatable-cookie/poodle-react";
import { applyThemeAttributes, cssVars, themes, densityModes, controlSizes } from "@inflatable-cookie/poodle-core/tokens";
import iconNodes from "lucide-static/icon-nodes.json";

import { DisplayControls } from "./DisplayControls";
import { ComponentsSection } from "./ComponentsSection";
import { TokensSection } from "./TokensSection";

type ThemeName = keyof typeof themes;
type DensityName = keyof typeof densityModes;
type ControlSizeName = keyof typeof controlSizes;
type SectionId = "components" | "tokens";

interface Route {
  section: SectionId;
  component?: string;
}

const topTabs: TabItem[] = [
  { value: "components", label: "Components" },
  { value: "tokens", label: "Tokens" },
];

const semanticPaths = Object.keys(cssVars) as Array<keyof typeof cssVars>;

function parseRoute(hash: string): Route {
  const component = hash.match(/^#components\/([a-z0-9-]+)$/);
  if (component) return { section: "components", component: component[1] };
  if (hash.startsWith("#tokens")) return { section: "tokens" };
  return { section: "components" };
}

export function App() {
  const shellRef = useRef<HTMLDivElement | null>(null);

  const [theme, setTheme] = useState<ThemeName>("eclipse");
  const [density, setDensity] = useState<DensityName>("compact");
  const [controlSize, setControlSize] = useState<ControlSizeName>("sm");
  const [contrast, setContrast] = useState(0.5);
  const [search, setSearch] = useState("");
  const [route, setRoute] = useState<Route>({ section: "components" });
  const [mounted, setMounted] = useState(false);
  const [liveTokenValues, setLiveTokenValues] = useState<Partial<Record<string, string>>>({});

  // ── Routing + initial query state ────────────────────────────────────
  useEffect(() => {
    function sync(): void {
      setRoute(parseRoute(window.location.hash));
      const params = new URLSearchParams(window.location.search);
      const paramTheme = params.get("theme");
      const paramDensity = params.get("density");
      const paramControlSize = params.get("controlSize");
      if (paramTheme && paramTheme in themes) setTheme(paramTheme as ThemeName);
      if (paramDensity && paramDensity in densityModes) setDensity(paramDensity as DensityName);
      if (paramControlSize && paramControlSize in controlSizes) setControlSize(paramControlSize as ControlSizeName);
    }
    sync();
    setMounted(true);
    window.addEventListener("hashchange", sync);
    window.addEventListener("popstate", sync);
    return () => {
      window.removeEventListener("hashchange", sync);
      window.removeEventListener("popstate", sync);
    };
  }, []);

  // ── Theme application + live token readout ───────────────────────────
  useEffect(() => {
    if (typeof document !== "undefined") {
      applyThemeAttributes(document.documentElement, { theme, density, controlSize });
    }
    const shell = shellRef.current;
    if (shell) {
      applyThemeAttributes(shell, { theme, density, controlSize });
      const styles = getComputedStyle(shell);
      const next: Partial<Record<string, string>> = {};
      for (const path of semanticPaths) {
        next[path] = styles.getPropertyValue(cssVars[path]).trim();
      }
      setLiveTokenValues(next);
    }
  }, [theme, density, controlSize, contrast]);

  // ── Persist preview mode to the URL ──────────────────────────────────
  useEffect(() => {
    if (!mounted) return;
    const searchParams = new URLSearchParams({ theme, density, controlSize });
    const hash = window.location.hash || "#components";
    const nextUrl = `${window.location.pathname}?${searchParams.toString()}${hash}`;
    const currentUrl = `${window.location.pathname}${window.location.search}${window.location.hash}`;
    if (nextUrl !== currentUrl) {
      window.history.replaceState(null, "", nextUrl);
    }
  }, [mounted, theme, density, controlSize, route]);

  function navigateToSection(section: SectionId): void {
    window.location.hash = section;
  }

  const shellStyle: CSSProperties | undefined =
    contrast === 0.5 ? undefined : ({ "--poodle-contrast": contrast } as CSSProperties);

  return (
    <UiPresentationProvider density={density} sizeScale={controlSize}>
      <div className="poodle-app-shell app-shell" style={shellStyle} ref={shellRef}>
        <header className="poodle-app-top-bar">
          <div className="poodle-app-top-bar__title">
            <strong>Poodle</strong>
            <span className="poodle-app-top-bar__framework">React</span>
          </div>
          <Tabs
            value={route.section}
            items={topTabs}
            variant="pill"
            ariaLabel="Main navigation"
            onValueChange={(value) => navigateToSection(value as SectionId)}
          />
          <div className="poodle-app-top-bar__pills">
            <Pill>{theme}</Pill>
            <Pill>{density}</Pill>
            <Pill>{controlSize}</Pill>
          </div>
        </header>

        <DisplayControls
          theme={theme}
          density={density}
          controlSize={controlSize}
          search={search}
          contrast={contrast}
          onThemeChange={(value) => setTheme(value as ThemeName)}
          onDensityChange={(value) => setDensity(value as DensityName)}
          onControlSizeChange={(value) => setControlSize(value as ControlSizeName)}
          onContrastChange={(value) => setContrast(Math.round(value * 100) / 100)}
          onSearchChange={(value) => {
            setSearch(value);
            if (route.section !== "components") navigateToSection("components");
          }}
        />

        <main className="poodle-app-main">
          <IconProvider icons={iconNodes as unknown as IconSet}>
            {route.section === "tokens" ? (
              <TokensSection liveTokenValues={liveTokenValues} />
            ) : (
              <ComponentsSection activeComponent={route.component} search={search} />
            )}
          </IconProvider>
        </main>
      </div>
    </UiPresentationProvider>
  );
}
