// The web axis census (g15.017): for every catalogue route, in both web
// runtimes, the `Sizes` tab appears exactly when the component takes a `size`
// prop and the `Densities` tab exactly when it takes `density`, every visible
// axis tab renders content, and the Svelte and React tab sets agree. Axis
// eligibility is derived from each component's own source props — never from
// habit — so a page advertising a tab the component does not take fails here.
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import {
  act,
  cleanup as cleanupReact,
  fireEvent,
  render as renderReact,
  waitFor as waitForReact,
} from "@testing-library/react";
import {
  cleanup as cleanupSvelte,
  render as renderSvelte,
  waitFor as waitForSvelte,
} from "@testing-library/svelte";
import { createElement, type ComponentType } from "react";
import { describe, expect, it, vi } from "vitest";
import iconNodes from "lucide-static/icon-nodes.json";

import { IconProvider, type IconSet } from "../../packages/react/components/src";
import { specimenMap as reactMap } from "../../packages/react/preview/src/gallery/specimen-map";
import { SceneSpecimen as ReactSceneSpecimen } from "../../packages/react/preview/src/gallery/SceneSpecimen";
import { specimenMap as svelteMap } from "../../packages/svelte/preview/src/specimens/registry";
import PilotSpecimenHarness from "../../packages/svelte/preview/test/PilotSpecimenHarness.svelte";
import AxisSceneFixture from "../../packages/svelte/preview/test/AxisSceneFixture.svelte";
import AxisHelperNoRenderers from "../../packages/svelte/preview/test/AxisHelperNoRenderers.svelte";
import AxisHelperSizesOnly from "../../packages/svelte/preview/test/AxisHelperSizesOnly.svelte";
import AxisHelperDensitiesOnly from "../../packages/svelte/preview/test/AxisHelperDensitiesOnly.svelte";
import AxisHelperHiddenRenderers from "../../packages/svelte/preview/test/AxisHelperHiddenRenderers.svelte";
import catalogue from "../../packages/codegen/fixtures/preview-catalogue.json";
import { SpecimenLayout } from "../../packages/react/preview/src/gallery/SpecimenLayout";

const COMPONENT_SRC = join(import.meta.dirname, "../../packages/svelte/components/src");
const { components: catalogueComponents } = catalogue as {
  components: Array<{ slug: string; displayName: string }>;
};

/** The 175-route denominator: the 174 portable catalogue entries plus the
 *  web-only `MeterSurface` from `component-registry.ts`. */
const ROUTES = [...catalogueComponents.map((entry) => entry.slug), "meter-surface"];

/** The five fixture scenes, all generated from specimens-model.json. These
 *  render through SceneSpecimen, which needs the slug (the fixture data), not
 *  the specimen component. */
const SCENE_SLUGS = ["avatar", "callout", "empty-state", "pill", "spinner"];

/** The two routes whose specimens log by design: ToolbarSpecimen uses icon
 *  names absent from the fixture icon set, and ErrorBoundarySpecimen
 *  demonstrates a caught throw. Both are pre-existing and out of g15.017
 *  scope; they get tolerant tests below instead of running under the suite's
 *  console.error guard. */
const NOISY_SLUGS = ["toolbar", "error-boundary"];

/** A component takes an axis prop when its own source declares it. Mirrors the
 *  audit's static pass (axis eligibility read from each component's props).
 *  Keyed case-insensitively: `XYPad.svelte` matches the `xy-pad` route on
 *  case-sensitive filesystems. */
function axisEligibility(): Record<string, { size: boolean; density: boolean }> {
  const result: Record<string, { size: boolean; density: boolean }> = {};
  for (const file of readdirSync(COMPONENT_SRC)) {
    if (!file.endsWith(".svelte")) continue;
    const source = readFileSync(join(COMPONENT_SRC, file), "utf8");
    const name = file.replace(/\.svelte$/, "").toLowerCase();
    result[name] = {
      size: /\bsize\?\s*:/.test(source),
      density: /\bdensity\?\s*:/.test(source),
    };
  }
  return result;
}

const ELIGIBILITY = axisEligibility();

/** The exact 24 routes corrected by this card, for the explicit decision
 *  evidence the acceptance criteria name. */
const CORRECTED_ROUTES = [
  "confirm-action", "split-view", "toast-host", "alert-dialog", "dialog", "drawer",
  "form-dialog", "block-editor", "log-list", "video-player", "licence-activation",
  "licence-seats", "licence-status", "update-center", "update-status",
  "eyebrow", "text",
  "icon-button", "icon", "ui-presentation-provider",
  "avatar", "tooltip", "picker-shell", "meter-surface",
];

/** The twelve PR #38 audio pages: validation-only, must keep paired populated
 *  axis tabs outside Examples. */
const AUDIO_ROUTES = [
  "drag-number-field", "audio-meter", "audio-switch", "envelope-editor", "fader",
  "gain-reduction-meter", "keyboard", "knob", "mod-matrix-grid", "value-readout",
  "waveform-display", "xy-pad",
];

/** The SpecimenLayout's own tab buttons — scoped to the layout's tab strip so
 *  Tabs nested inside specimen content (e.g. the `tabs` route's own examples)
 *  can't pollute the census. */
function tabButtons(root: ParentNode): HTMLElement[] {
  const layout = root.querySelector(".poodle-specimen-layout");
  const strip = layout?.querySelector(".poodle-tabs") ?? root;
  return [...strip.querySelectorAll<HTMLElement>("button.poodle-tabs__tab")];
}

function tabLabels(root: ParentNode): string[] {
  return tabButtons(root)
    .map((button) => button.querySelector(".poodle-tabs__label")?.textContent?.trim() ?? "")
    .filter(Boolean);
}

async function clickTab(
  root: ParentNode,
  label: string,
  runtime: "Svelte" | "React",
): Promise<boolean> {
  const button = tabButtons(root).find(
    (candidate) => candidate.querySelector(".poodle-tabs__label")?.textContent?.trim() === label,
  );
  if (!button) return false;
  if (runtime === "React") {
    // The tabs machine flushes its transition in a microtask; the async act
    // drains it so the pane re-renders inside act, not after the test exits.
    await act(async () => {
      fireEvent.click(button);
    });
    await waitForReact(() => expect(button.getAttribute("aria-selected")).toBe("true"), { timeout: 10_000, interval: 50 });
  } else {
    fireEvent.click(button);
    await waitForSvelte(() => expect(button.getAttribute("aria-selected")).toBe("true"));
  }
  return true;
}

/** The active axis pane has rendered evidence. React wraps each renderer
 *  output in a `<span>` (an empty span means a null render); Svelte emits
 *  nothing for null renders, so any element there is evidence. Overlays
 *  (Dialog/Drawer/AlertDialog) portal outside the layout. */
function axisPaneHasEvidence(): boolean {
  const variants = [...document.querySelectorAll<HTMLElement>(".poodle-specimen-layout__variants > *")];
  const inPane = variants.some((child) => {
    if (child.tagName === "SPAN" && child.childElementCount === 0 && (child.textContent ?? "").trim() === "") {
      return false;
    }
    return true;
  });
  if (inPane) return true;
  const portaled = [...document.querySelectorAll<HTMLElement>("body .poodle-dialog")];
  return portaled.some((node) => node.childElementCount > 0 || (node.textContent ?? "").trim() !== "");
}

async function awaitPaneEvidence(runtime: "Svelte" | "React"): Promise<void> {
  const waitFor = runtime === "Svelte" ? waitForSvelte : waitForReact;
  // The heavy icon matrices can take longer than waitFor's 1s default when the
  // full suite shares the worker; give the pane time to flush.
  await waitFor(() => expect(axisPaneHasEvidence()).toBe(true), { timeout: 10_000, interval: 50 });
}

function renderSvelteSpecimen(slug: string) {
  return SCENE_SLUGS.includes(slug)
    ? renderSvelte(AxisSceneFixture, { props: { slug } })
    : renderSvelte(PilotSpecimenHarness, { props: { specimen: svelteMap[slug] as never } });
}

function renderReactSpecimen(slug: string) {
  const element = SCENE_SLUGS.includes(slug)
    ? createElement(ReactSceneSpecimen, { slug })
    : createElement(reactMap[slug]! as ComponentType);
  return renderReact(
    createElement(
      IconProvider,
      { icons: iconNodes as unknown as IconSet },
      element,
    ),
  );
}

/** Assert the full tab contract for one route in one runtime and return the
 *  observed tab set plus whether the page uses SpecimenLayout (demo pages
 *  like error-boundary render no layout and therefore no tabs). */
async function assertRoute(
  runtime: "Svelte" | "React",
  slug: string,
): Promise<{ tabs: string[]; hasLayout: boolean } | null> {
  if (runtime === "Svelte") {
    renderSvelteSpecimen(slug);
  } else {
    renderReactSpecimen(slug);
  }

  const layout = document.querySelector(".poodle-specimen-layout");
  const tabs = tabLabels(document);
  if (!layout) {
    // A demo page (e.g. error-boundary) renders no SpecimenLayout at all.
    return tabs.length === 0 ? { tabs, hasLayout: false } : null;
  }

  const eligibility = ELIGIBILITY[slug.replace(/-/g, "")];
  const expected = ["Examples"];
  if (eligibility.size) expected.push("Sizes");
  if (eligibility.density) expected.push("Densities");

  for (const label of expected.slice(1)) {
    const clicked = await clickTab(document, label, runtime);
    if (!clicked) return null;
    await awaitPaneEvidence(runtime);
    await clickTab(document, "Examples", runtime);
  }
  return { tabs, hasLayout: true };
}

describe("SpecimenLayout axis-tab hardening", () => {
  it("shows only Examples when no axis renderer is supplied (Svelte)", () => {
    renderSvelte(AxisHelperNoRenderers);
    expect(tabLabels(document)).toEqual(["Examples"]);
    cleanupSvelte();
  });

  it("shows only Examples when no axis renderer is supplied (React)", () => {
    renderReact(
      <SpecimenLayout>
        <p>examples content</p>
      </SpecimenLayout>,
    );
    expect(tabLabels(document)).toEqual(["Examples"]);
    cleanupReact();
  });

  it("exposes only the matching tab for a supplied sizes renderer (Svelte)", async () => {
    renderSvelte(AxisHelperSizesOnly);
    expect(tabLabels(document)).toEqual(["Examples", "Sizes"]);
    await clickTab(document, "Sizes", "Svelte");
    await awaitPaneEvidence("Svelte");
    cleanupSvelte();
  });

  it("exposes only the matching tab for a supplied sizes renderer (React)", async () => {
    renderReact(
      <SpecimenLayout sizes={(size) => <p data-size={size}>size {size}</p>}>
        <p>examples content</p>
      </SpecimenLayout>,
    );
    expect(tabLabels(document)).toEqual(["Examples", "Sizes"]);
    await clickTab(document, "Sizes", "React");
    await awaitPaneEvidence("React");
    cleanupReact();
  });

  it("exposes only the matching tab for a supplied densities renderer (Svelte)", async () => {
    renderSvelte(AxisHelperDensitiesOnly);
    expect(tabLabels(document)).toEqual(["Examples", "Densities"]);
    await clickTab(document, "Densities", "Svelte");
    await awaitPaneEvidence("Svelte");
    cleanupSvelte();
  });

  it("exposes only the matching tab for a supplied densities renderer (React)", async () => {
    renderReact(
      <SpecimenLayout densities={(density) => <p data-density={density}>density {density}</p>}>
        <p>examples content</p>
      </SpecimenLayout>,
    );
    expect(tabLabels(document)).toEqual(["Examples", "Densities"]);
    await clickTab(document, "Densities", "React");
    await awaitPaneEvidence("React");
    cleanupReact();
  });

  it("lets showSizes/showDensities hide a supplied renderer (Svelte)", () => {
    renderSvelte(AxisHelperHiddenRenderers);
    expect(tabLabels(document)).toEqual(["Examples"]);
    cleanupSvelte();
  });

  it("lets showSizes/showDensities hide a supplied renderer (React)", () => {
    renderReact(
      <SpecimenLayout showSizes={false} showDensities={false} sizes={(size) => <p>{size}</p>} densities={(d) => <p>{d}</p>}>
        <p>examples content</p>
      </SpecimenLayout>,
    );
    expect(tabLabels(document)).toEqual(["Examples"]);
    cleanupReact();
  });
});

describe("authored-scene tab projection", () => {
  it("declares Avatar size-only in the generated scene data", async () => {
    const { specimenScenes } = await import("../../packages/react/preview/src/generated/specimens/specimen-scenes");
    expect(specimenScenes.avatar.tabs).toEqual(["examples", "sizes"]);
    expect(specimenScenes.avatar.densityAxis).toEqual([]);
    expect(specimenScenes.avatar.sizeAxis).toEqual(["xs", "sm", "md", "lg", "xl"]);
  });

  it("renders only the declared tabs for Avatar through SceneSpecimen in both runtimes", async () => {
    renderSvelte(AxisSceneFixture, { props: { slug: "avatar" } });
    expect(tabLabels(document)).toEqual(["Examples", "Sizes"]);
    cleanupSvelte();

    renderReact(createElement(ReactSceneSpecimen, { slug: "avatar" }));
    expect(tabLabels(document)).toEqual(["Examples", "Sizes"]);
    cleanupReact();
  });

  it("keeps the full tab set for scenes that still declare both axes", () => {
    renderSvelte(AxisSceneFixture, { props: { slug: "callout" } });
    expect(tabLabels(document)).toEqual(["Examples", "Sizes", "Densities"]);
    cleanupSvelte();

    renderReact(createElement(ReactSceneSpecimen, { slug: "callout" }));
    expect(tabLabels(document)).toEqual(["Examples", "Sizes", "Densities"]);
    cleanupReact();
  });
});

describe("web axis census (175 routes)", () => {
  it("covers the full catalogue denominator", () => {
    expect(ROUTES).toHaveLength(175);
    expect(new Set(ROUTES).size).toBe(175);
  });

  for (const slug of CORRECTED_ROUTES) {
    it(`${slug} shows exactly the eligible tabs, paired and populated`, async () => {
      // Route-pair renderings (especially the icon matrix) run long under the
      // full-suite worker; reset residue from any interrupted neighbor too.
      document.body.innerHTML = "";
      const eligibility = ELIGIBILITY[slug.replace(/-/g, "")];
      expect(eligibility, `${slug}: component source not found for eligibility`).toBeTruthy();
      const expected = ["Examples"];
      if (eligibility.size) expected.push("Sizes");
      if (eligibility.density) expected.push("Densities");

      expect(svelteMap[slug], `${slug} missing from Svelte registry`).toBeTruthy();
      expect(reactMap[slug], `${slug} missing from React registry`).toBeTruthy();

      const svelteTabs = await assertRoute("Svelte", slug);
      expect(svelteTabs, `${slug} (Svelte)`).toEqual({ tabs: expected, hasLayout: true });
      cleanupSvelte();

      const reactTabs = await assertRoute("React", slug);
      expect(reactTabs, `${slug} (React)`).toEqual({ tabs: expected, hasLayout: true });
      cleanupReact();
    }, 60_000);
  }

  it("keeps the twelve PR #38 audio pages paired with populated axis tabs", async () => {
    for (const slug of AUDIO_ROUTES) {
      const eligibility = ELIGIBILITY[slug.replace(/-/g, "")];
      expect(eligibility?.size && eligibility?.density, `${slug} audio component missing size/density props`).toBe(true);

      const svelteTabs = await assertRoute("Svelte", slug);
      expect(svelteTabs, `${slug} (Svelte)`).toEqual({ tabs: ["Examples", "Sizes", "Densities"], hasLayout: true });
      cleanupSvelte();

      const reactTabs = await assertRoute("React", slug);
      expect(reactTabs, `${slug} (React)`).toEqual({ tabs: ["Examples", "Sizes", "Densities"], hasLayout: true });
      cleanupReact();
    }
  }, 60_000);

  it("reports Sizes/Densities iff the component takes the prop, in both runtimes, across all routes", async () => {
    // 173 routes × two runtimes: the sweep outlives the default 5s test timeout.
    document.body.innerHTML = "";
    const disagreements: string[] = [];
    for (const slug of ROUTES.filter((route) => !NOISY_SLUGS.includes(route))) {
      const eligibility = ELIGIBILITY[slug.replace(/-/g, "")];
      expect(eligibility, `${slug}: component source not found`).toBeTruthy();

      const svelteResult = await assertRoute("Svelte", slug);
      cleanupSvelte();

      const reactResult = await assertRoute("React", slug);
      cleanupReact();

      if (!svelteResult || !reactResult) {
        disagreements.push(`${slug}: route could not be exercised`);
        continue;
      }

      const { tabs: svelteTabs, hasLayout: svelteLayout } = svelteResult;
      const { tabs: reactTabs, hasLayout: reactLayout } = reactResult;

      if (svelteLayout !== reactLayout) {
        disagreements.push(`${slug} layout drift: Svelte=${svelteLayout} React=${reactLayout}`);
      }
      if (JSON.stringify(reactTabs) !== JSON.stringify(svelteTabs)) {
        disagreements.push(`${slug} runtime drift: ${JSON.stringify(svelteTabs)} vs ${JSON.stringify(reactTabs)}`);
      }

      const expected = svelteLayout ? ["Examples"] : [];
      if (svelteLayout) {
        if (eligibility.size) expected.push("Sizes");
        if (eligibility.density) expected.push("Densities");
      }
      if (JSON.stringify(svelteTabs) !== JSON.stringify(expected)) {
        disagreements.push(`${slug} (Svelte): ${JSON.stringify(svelteTabs)} ≠ ${JSON.stringify(expected)}`);
      }
      if (JSON.stringify(reactTabs) !== JSON.stringify(expected)) {
        disagreements.push(`${slug} (React): ${JSON.stringify(reactTabs)} ≠ ${JSON.stringify(expected)}`);
      }
    }
    expect(disagreements, disagreements.join("\n")).toEqual([]);
  }, 120_000);

  it("keeps toolbar and error-boundary on contract despite their designed noise", async () => {
    // ToolbarSpecimen renders icons missing from the fixture set and
    // ErrorBoundarySpecimen demonstrates a caught throw; both log to
    // console.error by design. Detach the suite's error guard for this test
    // only — the tab contract below is still asserted.
    vi.restoreAllMocks();
    document.body.innerHTML = "";

    const toolbarTabs = await assertRoute("Svelte", "toolbar");
    expect(toolbarTabs).toEqual({ tabs: ["Examples", "Sizes", "Densities"], hasLayout: true });
    cleanupSvelte();

    const toolbarReactTabs = await assertRoute("React", "toolbar");
    expect(toolbarReactTabs).toEqual({ tabs: ["Examples", "Sizes", "Densities"], hasLayout: true });
    cleanupReact();

    const boundaryTabs = await assertRoute("Svelte", "error-boundary");
    expect(boundaryTabs).toEqual({ tabs: [], hasLayout: false });
    cleanupSvelte();

    const boundaryReactTabs = await assertRoute("React", "error-boundary");
    expect(boundaryReactTabs).toEqual({ tabs: [], hasLayout: false });
    cleanupReact();
  });
});