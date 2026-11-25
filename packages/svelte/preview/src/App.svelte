<script lang="ts">
  import { previewThemeCssProperties, resolveTheme } from "@inflatable-cookie/poodle-core";
  import {
    applyThemeAttributes,
    cssVars,
    styles,
    themes,
    densityModes,
    controlSizes,
    type StyleName,
  } from "@inflatable-cookie/poodle-core/tokens";
  import {
    Pill,
    Tabs,
    IconProvider,
    UiPresentationProvider,
    type TabItem,
    type IconSet,
  } from "@inflatable-cookie/poodle-svelte";
  import iconNodes from "lucide-static/icon-nodes.json";
  import { onMount, setContext, tick } from "svelte";

  import DisplayControls from "./components/DisplayControls.svelte";
  import LiveThemePanel from "./components/LiveThemePanel.svelte";
  import ComponentsSection from "./sections/ComponentsSection.svelte";
  import TokensSection from "./sections/TokensSection.svelte";
  import { parseRoute, type Route, type SectionId } from "./router";
  import { findComponent } from "./component-registry";
  import { specimenMap } from "./specimens/registry";
  import { SPECIMEN_CAPTURE_CONTEXT } from "./specimen-capture-context";
  import { readLiveTheme, startLiveTheme, writeLiveTheme, type LiveThemeSettings } from "./live-theme";
  import { previewShell } from "./generated/preview-shell";
  import specimenCaptureFrame from "../../../preview-capture/specimen-frame.json";

  type ThemeName = keyof typeof themes;
  type DensityName = keyof typeof densityModes;
  type ControlSizeName = keyof typeof controlSizes;
  type SemanticTokenPath = keyof typeof cssVars;

  const initialSearch = typeof window === "undefined" ? "" : window.location.search;
  const startsInSpecimenCapture = new URLSearchParams(initialSearch).get("capture") === "specimen";
  setContext(SPECIMEN_CAPTURE_CONTEXT, startsInSpecimenCapture);

  // Navigation labels come from the scene (card 035 R4): the shell's top
  // tabs are the scene's layout sections, never authored text here.
  const topTabs: TabItem[] = previewShell.layout.sections.map((section) => ({
    value: section.kind,
    label: section.title,
  }));

  const semanticPaths = Object.keys(cssVars) as SemanticTokenPath[];

  // ── State ───────────────────────────────────────────────────────────

  let appShell: HTMLElement | null = $state(null);
  let theme: ThemeName = $state("eclipse");
  let liveTheme: LiveThemeSettings | null = $state(null);
  let style: StyleName = $state("default");
  let density: DensityName = $state("compact");
  let controlSize: ControlSizeName = $state("sm");
  let contrast = $state(0.5);
  let componentSearch = $state("");
  let route: Route = $state(
    typeof window === "undefined" ? { section: "components" } : parseRoute(window.location.hash),
  );
  let captureMode = $state(startsInSpecimenCapture);
  let liveTokenValues: Partial<Record<SemanticTokenPath, string>> = $state({});
  let hasMounted = $state(false);

  let activeSection = $derived(route.section);
  let captureEntry = $derived(captureMode && route.component ? findComponent(route.component) : undefined);
  let captureSpecimen = $derived(captureEntry ? specimenMap[captureEntry.slug] ?? null : null);
  const liveColors = $derived(liveTheme ? previewThemeCssProperties(resolveTheme(liveTheme).roles) : null);
  const liveSwatch = $derived(liveColors ? {
    canvas: liveColors["--poodle-color-background-canvas"],
    surface: liveColors["--poodle-color-background-surface"],
    accent: liveColors["--poodle-color-accent-base"],
    text: liveColors["--poodle-color-text-primary"],
    border: liveColors["--poodle-color-border-default"],
  } : undefined);
  // ── Theme application ───────────────────────────────────────────────

  function readSemanticTokenValues(element: HTMLElement): Partial<Record<SemanticTokenPath, string>> {
    const styles = getComputedStyle(element);
    return semanticPaths.reduce<Partial<Record<SemanticTokenPath, string>>>((acc, path) => {
      acc[path] = styles.getPropertyValue(cssVars[path]).trim();
      return acc;
    }, {});
  }

  $effect(() => {
    if (!appShell || !hasMounted) return;
    const root = document.documentElement;
    const axes = { theme: liveTheme ? undefined : theme, style, density, controlSize };
    applyThemeAttributes(root, axes);
    applyThemeAttributes(appShell, axes);

    const colors = liveColors;
    if (liveTheme && colors) {
      // Resolve aliases on html and keep the shell as the nearest theme root for portals.
      if (root.dataset.theme !== "live") root.dataset.theme = "live";
      if (appShell.dataset.theme !== "live") appShell.dataset.theme = "live";
      root.style.colorScheme = liveTheme.scheme;
      for (const [property, value] of Object.entries(colors)) root.style.setProperty(property, value);
    }
    liveTokenValues = readSemanticTokenValues(appShell);

    if (colors) return () => {
      root.style.removeProperty("color-scheme");
      for (const property of Object.keys(colors)) root.style.removeProperty(property);
    };
  });

  async function openLiveTheme(): Promise<void> {
    liveTheme ??= startLiveTheme(theme);
    await tick();
    appShell?.querySelector<HTMLElement>("#live-theme-settings")?.focus();
  }

  // ── Routing ─────────────────────────────────────────────────────────

  function syncCurrentLocation(): void {
    if (typeof window === "undefined") return;

    const hash = window.location.hash;
    const params = new URLSearchParams(window.location.search);

    route = parseRoute(hash);
    captureMode = params.get("capture") === "specimen";
    liveTheme = readLiveTheme(params);

    const paramTheme = params.get("theme");
    const paramStyle = params.get("style");
    const paramDensity = params.get("density");
    const paramControlSize = params.get("controlSize");

    if (paramTheme && paramTheme in themes) theme = paramTheme as ThemeName;
    style = isStyleName(paramStyle) ? paramStyle : "default";
    if (paramDensity && paramDensity in densityModes) density = paramDensity as DensityName;
    if (paramControlSize && paramControlSize in controlSizes) controlSize = paramControlSize as ControlSizeName;
  }

  function isStyleName(value: string | null): value is StyleName {
    return value !== null && Object.hasOwn(styles, value);
  }

  function navigateToSection(section: SectionId): void {
    if (typeof window !== "undefined") {
      window.location.hash = `${section}`;
    }
  }

  $effect(() => {
    if (hasMounted && typeof window !== "undefined") {
      const searchParams = new URLSearchParams({
        theme,
        style,
        density,
        controlSize,
      });
      if (captureMode) searchParams.set("capture", "specimen");
      if (liveTheme) writeLiveTheme(searchParams, liveTheme);
      const hash = window.location.hash || "#components";
      const nextUrl = `${window.location.pathname}?${searchParams.toString()}${hash}`;
      const currentUrl = `${window.location.pathname}${window.location.search}${window.location.hash}`;
      if (nextUrl !== currentUrl) {
        window.history.replaceState(null, "", nextUrl);
      }
    }
  });

  onMount(() => {
    syncCurrentLocation();
    hasMounted = true;

    const originalHtmlOverflow = document.documentElement.style.overflow;
    const originalBodyOverflow = document.body.style.overflow;
    if (captureMode) {
      document.documentElement.style.overflow = "hidden";
      document.body.style.overflow = "hidden";
      void (async () => {
        await tick();
        await document.fonts.ready;
        await tick();
        await new Promise<void>((resolve) => requestAnimationFrame(() => resolve()));
        await new Promise<void>((resolve) => requestAnimationFrame(() => resolve()));
        if (captureEntry && captureSpecimen && appShell) {
          appShell.dataset.captureReady = captureEntry.slug;
          appShell.dataset.captureDeviceScale = String(window.devicePixelRatio);
        }
      })();
    }

    return () => {
      document.documentElement.style.overflow = originalHtmlOverflow;
      document.body.style.overflow = originalBodyOverflow;
    };
  });
</script>

<svelte:head>
  <title>Poodle Docs Preview</title>
</svelte:head>

<svelte:window
  on:hashchange={syncCurrentLocation}
  on:popstate={syncCurrentLocation}
/>

<UiPresentationProvider density={density} sizeScale={controlSize}>
  {#if captureMode}
    {#if captureEntry && captureSpecimen}
      {@const Specimen = captureSpecimen as any}
      <main
        class="poodle-specimen-capture-frame"
        data-specimen-capture={captureEntry.slug}
        data-capture-reference-device-scale={specimenCaptureFrame.deviceScale}
        style:--specimen-capture-width={`${specimenCaptureFrame.logicalWidth}px`}
        style:--specimen-capture-height={`${specimenCaptureFrame.logicalHeight}px`}
        style:--specimen-capture-padding={`${specimenCaptureFrame.padding}px`}
        style:--poodle-contrast={liveTheme || contrast === 0.5 ? undefined : contrast}
        bind:this={appShell}
      >
        <IconProvider icons={iconNodes as unknown as IconSet}>
          <Specimen slug={captureEntry.slug} />
        </IconProvider>
      </main>
    {/if}
  {:else}
    <div class="poodle-app-shell" style:--poodle-contrast={liveTheme || contrast === 0.5 ? undefined : contrast} bind:this={appShell}>
      <header class="poodle-app-top-bar">
        <div class="poodle-app-top-bar__title">
          <strong>Poodle</strong>
          <span class="poodle-app-top-bar__framework">Svelte</span>
        </div>
        <Tabs
          value={activeSection}
          items={topTabs}
          variant="pill"
          ariaLabel="Main navigation"
          onValueChange={(value) => navigateToSection(value as SectionId)}
        />
        <div class="poodle-app-top-bar__pills">
          <Pill>{liveTheme ? "live" : theme}</Pill>
          <Pill>{density}</Pill>
          <Pill>{controlSize}</Pill>
        </div>
      </header>

      <DisplayControls
        {theme}
        {style}
        {density}
        {controlSize}
        search={componentSearch}
        onThemeChange={(value) => {
          theme = value as ThemeName;
          liveTheme = null;
        }}
        {liveSwatch}
        onLiveTheme={openLiveTheme}
        onStyleChange={(value) => { if (isStyleName(value)) style = value; }}
        onDensityChange={(value) => (density = value as DensityName)}
        onControlSizeChange={(value) => (controlSize = value as ControlSizeName)}
        {contrast}
        onContrastChange={(value) => (contrast = Math.round(value * 100) / 100)}
        onSearchChange={(value) => {
          componentSearch = value;
          if (activeSection !== "components") {
            navigateToSection("components");
          }
        }}
      />

      {#if liveTheme}
        <LiveThemePanel settings={liveTheme} onChange={(settings) => (liveTheme = settings)} />
      {/if}

      <main class="poodle-app-main">
        {#key `${theme}:${activeSection}`}
          <IconProvider icons={iconNodes as unknown as IconSet}>
            {#if activeSection === "components"}
              <ComponentsSection activeComponent={route.component} search={componentSearch} />
            {:else if activeSection === "tokens"}
              <TokensSection {liveTokenValues} />
            {/if}
          </IconProvider>
        {/key}
      </main>
    </div>
  {/if}
</UiPresentationProvider>

<style>
  .poodle-specimen-capture-frame {
    box-sizing: border-box;
    width: var(--specimen-capture-width);
    height: var(--specimen-capture-height);
    padding: var(--specimen-capture-padding);
    overflow: hidden;
    background: var(--poodle-color-background-canvas);
    color: var(--poodle-color-text-primary);
  }

  .poodle-app-shell {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }

  .poodle-app-top-bar {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.5rem 1rem;
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
    background: var(--poodle-color-background-elevated);
    flex-shrink: 0;
  }

  .poodle-app-top-bar__title strong {
    font-size: 1rem;
    font-weight: 700;
    color: var(--poodle-color-text-primary);
    white-space: nowrap;
  }

  .poodle-app-top-bar__title {
    display: flex;
    align-items: baseline;
    gap: 0.4rem;
  }

  .poodle-app-top-bar__framework {
    font-size: 0.6875rem;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--poodle-color-text-secondary);
  }

  .poodle-app-top-bar__pills {
    display: flex;
    gap: 0.375rem;
    margin-left: auto;
  }

  .poodle-app-main {
    flex: 1 1 0;
    min-height: 0;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
  }
</style>
