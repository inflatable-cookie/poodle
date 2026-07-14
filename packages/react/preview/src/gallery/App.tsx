import { useEffect, useRef, useState, type CSSProperties } from "react";
import { Pill, IconProvider, UiPresentationProvider, type IconSet } from "@poodle/react";
import { applyThemeAttributes, themes, densityModes, controlSizes } from "@poodle/svelte-tokens";
import iconNodes from "lucide-static/icon-nodes.json";

import { DisplayControls } from "./DisplayControls";
import { ComponentsSection } from "./ComponentsSection";

type ThemeName = keyof typeof themes;
type DensityName = keyof typeof densityModes;
type ControlSizeName = keyof typeof controlSizes;

interface Route {
  section: "components";
  component?: string;
}

function parseRoute(hash: string): Route {
  const match = hash.match(/^#components\/([a-z0-9-]+)$/);
  if (match) return { section: "components", component: match[1] };
  return { section: "components" };
}

export function App() {
  const shellRef = useRef<HTMLDivElement | null>(null);

  const [theme, setTheme] = useState<ThemeName>("dark");
  const [density, setDensity] = useState<DensityName>("compact");
  const [controlSize, setControlSize] = useState<ControlSizeName>("sm");
  const [contrast, setContrast] = useState(0.5);
  const [search, setSearch] = useState("");
  const [route, setRoute] = useState<Route>({ section: "components" });
  const [mounted, setMounted] = useState(false);

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

  // ── Theme application ────────────────────────────────────────────────
  useEffect(() => {
    if (typeof document !== "undefined") {
      applyThemeAttributes(document.documentElement, { theme, density, controlSize });
    }
    if (shellRef.current) {
      applyThemeAttributes(shellRef.current, { theme, density, controlSize });
    }
  }, [theme, density, controlSize]);

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
          onSearchChange={setSearch}
        />

        <main className="poodle-app-main">
          <IconProvider icons={iconNodes as unknown as IconSet}>
            <ComponentsSection activeComponent={route.component} search={search} />
          </IconProvider>
        </main>
      </div>
    </UiPresentationProvider>
  );
}
