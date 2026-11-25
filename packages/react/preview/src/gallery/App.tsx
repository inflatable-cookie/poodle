import { useEffect, useMemo, useRef, useState, type CSSProperties } from "react";
import { Pill, Tabs, IconProvider, UiPresentationProvider, type IconSet, type TabItem } from "@inflatable-cookie/poodle-react";
import { previewThemeCssProperties, resolveTheme } from "@inflatable-cookie/poodle-core";
import { applyThemeAttributes, cssVars, styles, themes, densityModes, controlSizes, type StyleName } from "@inflatable-cookie/poodle-core/tokens";
import iconNodes from "lucide-static/icon-nodes.json";
import { readLiveTheme, startLiveTheme, writeLiveTheme, type LiveThemeSettings } from "@poodle/svelte-preview/live-theme";

import { DisplayControls } from "./DisplayControls";
import { ComponentsSection } from "./ComponentsSection";
import { LiveThemePanel } from "./LiveThemePanel";
import { TokensSection } from "./TokensSection";
import { previewShell } from "../generated/preview-shell";

type ThemeName = keyof typeof themes;
type DensityName = keyof typeof densityModes;
type ControlSizeName = keyof typeof controlSizes;
type SectionId = "components" | "tokens";

interface Route {
  section: SectionId;
  component?: string;
}

// Navigation labels come from the scene (card 035 R4): the shell's top
// tabs are the scene's layout sections, never authored text here.
const topTabs: TabItem[] = previewShell.layout.sections.map((section) => ({
  value: section.kind,
  label: section.title,
}));

const semanticPaths = Object.keys(cssVars) as Array<keyof typeof cssVars>;

function parseRoute(hash: string): Route {
  const component = hash.match(/^#components\/([a-z0-9-]+)$/);
  if (component) return { section: "components", component: component[1] };
  if (hash.startsWith("#tokens")) return { section: "tokens" };
  return { section: "components" };
}

function isStyleName(value: string | null): value is StyleName {
  return value !== null && Object.hasOwn(styles, value);
}

export function App() {
  const shellRef = useRef<HTMLDivElement | null>(null);

  const [theme, setTheme] = useState<ThemeName>("eclipse");
  const [style, setStyle] = useState<StyleName>("default");
  const [density, setDensity] = useState<DensityName>("compact");
  const [controlSize, setControlSize] = useState<ControlSizeName>("sm");
  const [contrast, setContrast] = useState(0.5);
  const [search, setSearch] = useState("");
  const [route, setRoute] = useState<Route>({ section: "components" });
  const [mounted, setMounted] = useState(false);
  const [liveTokenValues, setLiveTokenValues] = useState<Partial<Record<string, string>>>({});
  // Null while a built-in theme is selected; the settings drive the colours otherwise.
  const [liveTheme, setLiveTheme] = useState<LiveThemeSettings | null>(null);
  const [panelFocusRequest, setPanelFocusRequest] = useState(0);
  const liveColors = useMemo(
    () => (liveTheme ? previewThemeCssProperties(resolveTheme(liveTheme).roles) : null),
    [liveTheme],
  );
  const liveSwatch = useMemo(
    () =>
      liveColors
        ? {
            canvas: liveColors["--poodle-color-background-canvas"],
            surface: liveColors["--poodle-color-background-surface"],
            accent: liveColors["--poodle-color-accent-base"],
            text: liveColors["--poodle-color-text-primary"],
            border: liveColors["--poodle-color-border-default"],
          }
        : undefined,
    [liveColors],
  );

  // ── Routing + initial query state ────────────────────────────────────
  useEffect(() => {
    function sync(): void {
      setRoute(parseRoute(window.location.hash));
      const params = new URLSearchParams(window.location.search);
      const paramTheme = params.get("theme");
      const paramStyle = params.get("style");
      const paramDensity = params.get("density");
      const paramControlSize = params.get("controlSize");
      setLiveTheme(readLiveTheme(params));
      if (paramTheme && paramTheme in themes) setTheme(paramTheme as ThemeName);
      setStyle(isStyleName(paramStyle) ? paramStyle : "default");
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
    const root = document.documentElement;
    const shell = shellRef.current;
    const axes = { theme: liveColors ? undefined : theme, style, density, controlSize };
    applyThemeAttributes(root, axes);
    if (shell) applyThemeAttributes(shell, axes);

    // The live colours sit on the root so portals inherit them; the shell is
    // marked as the nearest theme root for the same reason.
    if (liveTheme && liveColors) {
      root.dataset.theme = "live";
      if (shell) shell.dataset.theme = "live";
      root.style.colorScheme = liveTheme.scheme;
      for (const [property, value] of Object.entries(liveColors)) root.style.setProperty(property, value);
    }

    if (shell) {
      const computedStyles = getComputedStyle(shell);
      const next: Partial<Record<string, string>> = {};
      for (const path of semanticPaths) {
        next[path] = computedStyles.getPropertyValue(cssVars[path]).trim();
      }
      setLiveTokenValues(next);
    }

    if (!liveColors) return;
    return () => {
      root.style.removeProperty("color-scheme");
      for (const property of Object.keys(liveColors)) root.style.removeProperty(property);
    };
  }, [theme, style, density, controlSize, contrast, liveTheme, liveColors]);

  // ── Persist preview mode to the URL ──────────────────────────────────
  useEffect(() => {
    if (!mounted) return;
    const searchParams = new URLSearchParams({ theme, style, density, controlSize });
    if (liveTheme) writeLiveTheme(searchParams, liveTheme);
    const hash = window.location.hash || "#components";
    const nextUrl = `${window.location.pathname}?${searchParams.toString()}${hash}`;
    const currentUrl = `${window.location.pathname}${window.location.search}${window.location.hash}`;
    if (nextUrl !== currentUrl) {
      window.history.replaceState(null, "", nextUrl);
    }
  }, [mounted, theme, style, density, controlSize, liveTheme, route]);

  function navigateToSection(section: SectionId): void {
    window.location.hash = section;
  }

  // Picking "Live theme" starts from the current built-in theme; an open live theme is kept.
  function openLiveTheme(): void {
    setLiveTheme((current) => current ?? startLiveTheme(theme));
    setPanelFocusRequest((request) => request + 1);
  }

  const shellStyle: CSSProperties | undefined =
    liveTheme || contrast === 0.5 ? undefined : ({ "--poodle-contrast": contrast } as CSSProperties);

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
            <Pill>{liveTheme ? "live" : theme}</Pill>
            <Pill>{density}</Pill>
            <Pill>{controlSize}</Pill>
          </div>
        </header>

        <DisplayControls
          theme={theme}
          style={style}
          density={density}
          controlSize={controlSize}
          search={search}
          contrast={contrast}
          liveSwatch={liveSwatch}
          onLiveTheme={openLiveTheme}
          onThemeChange={(value) => {
            setTheme(value as ThemeName);
            setLiveTheme(null);
          }}
          onStyleChange={(value) => {
            if (isStyleName(value)) setStyle(value);
          }}
          onDensityChange={(value) => setDensity(value as DensityName)}
          onControlSizeChange={(value) => setControlSize(value as ControlSizeName)}
          onContrastChange={(value) => setContrast(Math.round(value * 100) / 100)}
          onSearchChange={(value) => {
            setSearch(value);
            if (route.section !== "components") navigateToSection("components");
          }}
        />

        {liveTheme && (
          <LiveThemePanel settings={liveTheme} onChange={setLiveTheme} focusRequest={panelFocusRequest} />
        )}

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
