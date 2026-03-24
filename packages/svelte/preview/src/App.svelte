<script lang="ts">
  import {
    applyThemeAttributes,
    cssVars,
    themes,
    densityModes,
    controlSizes,
  } from "@poodle/svelte-tokens";
  import {
    Pill,
    Tabs,
    IconProvider,
    type TabItem,
  } from "@poodle/svelte-primitives";
  import iconNodes from "lucide-static/icon-nodes.json";
  import { onMount } from "svelte";

  import DisplayControls from "./components/DisplayControls.svelte";
  import PrimitivesSection from "./sections/PrimitivesSection.svelte";
  import CompositesSection from "./sections/CompositesSection.svelte";
  import TokensSection from "./sections/TokensSection.svelte";
  import TreatmentsSection from "./sections/TreatmentsSection.svelte";
  import { parseRoute, type Route, type SectionId } from "./router";

  type ThemeName = keyof typeof themes;
  type DensityName = keyof typeof densityModes;
  type ControlSizeName = keyof typeof controlSizes;
  type AppearanceTreatmentName = "system" | "brand-raised";
  type SemanticTokenPath = keyof typeof cssVars;

  const topTabs: TabItem[] = [
    { value: "primitives", label: "Primitives" },
    { value: "composites", label: "Composites" },
    { value: "tokens", label: "Tokens" },
    { value: "treatments", label: "Treatments" },
  ];

  const semanticPaths = Object.keys(cssVars) as SemanticTokenPath[];

  // ── State ───────────────────────────────────────────────────────────

  let appShell: HTMLElement | null = null;
  let theme: ThemeName = "dark";
  let density: DensityName = "compact";
  let controlSize: ControlSizeName = "md";
  let appearanceTreatment: AppearanceTreatmentName = "system";
  let disabled = false;
  let invalid = true;
  let busy = false;
  let route: Route = { section: "primitives" };
  let liveTokenValues: Partial<Record<SemanticTokenPath, string>> = {};
  let previewModeKey = "";
  let appliedPreviewModeKey = "";
  let hasMounted = false;

  $: activeSection = route.section;

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
    applyThemeAttributes(appShell, { theme, density, controlSize });
    liveTokenValues = readSemanticTokenValues(appShell);
    appliedPreviewModeKey = previewModeKey;
  }

  $: previewModeKey = `${theme}:${density}:${controlSize}`;

  $: if (appShell && previewModeKey && previewModeKey !== appliedPreviewModeKey) {
    refreshPreviewSurface();
  }

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

  $: if (hasMounted && typeof window !== "undefined") {
    const searchParams = new URLSearchParams({
      theme,
      density,
      controlSize,
    });
    const hash = window.location.hash || "#primitives";
    const nextUrl = `${window.location.pathname}?${searchParams.toString()}${hash}`;
    const currentUrl = `${window.location.pathname}${window.location.search}${window.location.hash}`;
    if (nextUrl !== currentUrl) {
      window.history.replaceState(null, "", nextUrl);
    }
  }

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

<div class="app-shell" data-appearance-treatment={appearanceTreatment} bind:this={appShell}>
  <header class="app-top-bar">
    <div class="app-top-bar__title">
      <strong>Poodle</strong>
    </div>
    <Tabs
      value={activeSection}
      items={topTabs}
      variant="pill"
      ariaLabel="Main navigation"
      on:valueChange={(event) => navigateToSection(event.detail.value as SectionId)}
    />
    <div class="app-top-bar__pills">
      <Pill>{theme}</Pill>
      <Pill>{density}</Pill>
      <Pill>{controlSize}</Pill>
    </div>
  </header>

  <DisplayControls
    {theme}
    {density}
    {controlSize}
    {appearanceTreatment}
    {disabled}
    {invalid}
    {busy}
    onThemeChange={(value) => (theme = value as ThemeName)}
    onDensityChange={(value) => (density = value as DensityName)}
    onControlSizeChange={(value) => (controlSize = value as ControlSizeName)}
    onAppearanceTreatmentChange={(value) => (appearanceTreatment = value as AppearanceTreatmentName)}
    onDisabledChange={(checked) => (disabled = checked)}
    onInvalidChange={(checked) => (invalid = checked)}
    onBusyChange={(checked) => (busy = checked)}
  />

  <main class="app-main">
    <IconProvider icons={iconNodes}>
      {#if activeSection === "primitives"}
        <PrimitivesSection activeComponent={route.component} />
      {:else if activeSection === "composites"}
        <CompositesSection activeComponent={route.component} />
      {:else if activeSection === "tokens"}
        <TokensSection {liveTokenValues} />
      {:else if activeSection === "treatments"}
        <TreatmentsSection />
      {/if}
    </IconProvider>
  </main>
</div>

<style>
  :global(.app-shell) {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }

  .app-top-bar {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.5rem 1rem;
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
    background: var(--poodle-color-background-elevated);
    flex-shrink: 0;
  }

  .app-top-bar__title strong {
    font-size: 1rem;
    font-weight: 700;
    color: var(--poodle-color-text-primary);
    white-space: nowrap;
  }

  .app-top-bar__pills {
    display: flex;
    gap: 0.375rem;
    margin-left: auto;
  }

  .app-main {
    flex: 1 1 0;
    min-height: 0;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
  }
</style>
