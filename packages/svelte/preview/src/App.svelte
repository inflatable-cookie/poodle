<script lang="ts">
  import {
    applyThemeAttributes,
    cssVars,
    themes,
    densityModes,
    controlSizes,
  } from "@inflatable-cookie/poodle-svelte-tokens";
  import {
    Pill,
    Tabs,
    IconProvider,
    UiPresentationProvider,
    type TabItem,
    type IconSet,
  } from "@inflatable-cookie/poodle-svelte";
  import iconNodes from "lucide-static/icon-nodes.json";
  import { onMount } from "svelte";

  import DisplayControls from "./components/DisplayControls.svelte";
  import ComponentsSection from "./sections/ComponentsSection.svelte";
  import TokensSection from "./sections/TokensSection.svelte";
  import { parseRoute, type Route, type SectionId } from "./router";

  type ThemeName = keyof typeof themes;
  type DensityName = keyof typeof densityModes;
  type ControlSizeName = keyof typeof controlSizes;
  type SemanticTokenPath = keyof typeof cssVars;

  const topTabs: TabItem[] = [
    { value: "components", label: "Components" },
    { value: "tokens", label: "Tokens" },
  ];

  const semanticPaths = Object.keys(cssVars) as SemanticTokenPath[];

  // ── State ───────────────────────────────────────────────────────────

  let appShell: HTMLElement | null = $state(null);
  let theme: ThemeName = $state("eclipse");
  let density: DensityName = $state("compact");
  let controlSize: ControlSizeName = $state("sm");
  let contrast = $state(0.5);
  let componentSearch = $state("");
  let route: Route = $state({ section: "components" });
  let liveTokenValues: Partial<Record<SemanticTokenPath, string>> = $state({});
  let appliedPreviewModeKey = "";
  let hasMounted = false;

  let activeSection = $derived(route.section);
  // ── Theme application ───────────────────────────────────────────────

  function readSemanticTokenValues(element: HTMLElement): Partial<Record<SemanticTokenPath, string>> {
    const styles = getComputedStyle(element);
    return semanticPaths.reduce<Partial<Record<SemanticTokenPath, string>>>((acc, path) => {
      acc[path] = styles.getPropertyValue(cssVars[path]).trim();
      return acc;
    }, {});
  }

  function refreshPreviewSurface(): void {
    if (!appShell) return;
    if (typeof document !== "undefined") {
      applyThemeAttributes(document.documentElement, { theme, density, controlSize });
    }
    applyThemeAttributes(appShell, { theme, density, controlSize });
    liveTokenValues = readSemanticTokenValues(appShell);
    appliedPreviewModeKey = previewModeKey;
  }

  let previewModeKey = $derived(`${theme}:${density}:${controlSize}`);
  $effect(() => {
    if (appShell && previewModeKey && previewModeKey !== appliedPreviewModeKey) {
      refreshPreviewSurface();
    }
  });

  // ── Routing ─────────────────────────────────────────────────────────

  function syncCurrentLocation(): void {
    if (typeof window === "undefined") return;

    const hash = window.location.hash;
    const params = new URLSearchParams(window.location.search);

    route = parseRoute(hash);

    const paramTheme = params.get("theme");
    const paramDensity = params.get("density");
    const paramControlSize = params.get("controlSize");

    if (paramTheme && paramTheme in themes) theme = paramTheme as ThemeName;
    if (paramDensity && paramDensity in densityModes) density = paramDensity as DensityName;
    if (paramControlSize && paramControlSize in controlSizes) controlSize = paramControlSize as ControlSizeName;
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
        density,
        controlSize,
      });
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
    refreshPreviewSurface();
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
  <div class="poodle-app-shell" style:--poodle-contrast={contrast === 0.5 ? undefined : contrast} bind:this={appShell}>
    <header class="poodle-app-top-bar">
      <div class="poodle-app-top-bar__title">
        <strong>Poodle</strong>
      </div>
      <Tabs
        value={activeSection}
        items={topTabs}
        variant="pill"
        ariaLabel="Main navigation"
        onValueChange={(value) => navigateToSection(value as SectionId)}
      />
      <div class="poodle-app-top-bar__pills">
        <Pill>{theme}</Pill>
        <Pill>{density}</Pill>
        <Pill>{controlSize}</Pill>
      </div>
    </header>

    <DisplayControls
      {theme}
      {density}
      {controlSize}
      search={componentSearch}
      onThemeChange={(value) => (theme = value as ThemeName)}
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
</UiPresentationProvider>

<style>
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
