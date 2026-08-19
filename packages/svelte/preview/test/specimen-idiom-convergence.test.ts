import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";

import { specimenMap as svelteRegistry } from "../src/specimens/registry";
import ListCardCounterSpecimen from "../src/specimens/ListCardCounterSpecimen.svelte";
import MetaItemSpecimen from "../src/specimens/MetaItemSpecimen.svelte";

const PREVIEW_ROOT = join(import.meta.dirname, "..");
const SVELTE_SPECIMENS = join(PREVIEW_ROOT, "src/specimens");
const REACT_SPECIMENS = join(PREVIEW_ROOT, "../../react/preview/src/gallery/specimens");

const SCOPED_ROUTE_FILES = [
  ["split-button", "SplitButtonSpecimen"],
  ["tri-state-switch", "TriStateSwitchSpecimen"],
  ["select", "SelectSpecimen"],
  ["text-input", "TextInputSpecimen"],
  ["token-input", "TokenInputSpecimen"],
  ["time-input", "TimeInputSpecimen"],
  ["time-zone-select", "TimeZoneSelectSpecimen"],
  ["eyebrow", "EyebrowSpecimen"],
  ["alert-dialog", "AlertDialogSpecimen"],
  ["dialog", "DialogSpecimen"],
  ["drawer", "DrawerSpecimen"],
  ["menu", "MenuSpecimen"],
  ["markdown-editor", "MarkdownEditorSpecimen"],
  ["settings-shell", "SettingsShellSpecimen"],
  ["drag-number-field", "DragNumberFieldSpecimen"],
  ["audio-meter", "AudioMeterSpecimen"],
  ["audio-switch", "AudioSwitchSpecimen"],
  ["envelope-editor", "EnvelopeEditorSpecimen"],
  ["fader", "FaderSpecimen"],
  ["gain-reduction-meter", "GainReductionMeterSpecimen"],
  ["keyboard", "KeyboardSpecimen"],
  ["knob", "KnobSpecimen"],
  ["mod-matrix-grid", "ModMatrixGridSpecimen"],
  ["value-readout", "ValueReadoutSpecimen"],
  ["waveform-display", "WaveformDisplaySpecimen"],
  ["xy-pad", "XYPadSpecimen"],
  ["meter-surface", "MeterSurfaceSpecimen"],
  ["list-card-counter", "ListCardCounterSpecimen"],
  ["meta-item", "MetaItemSpecimen"],
] as const;

function specimenSources(dir: string): string {
  return readdirSync(dir)
    .filter((name) => /Specimen\.(svelte|tsx)$/.test(name))
    .map((name) => readFileSync(join(dir, name), "utf8"))
    .join("\n");
}

/** Visible example captions declared on SpecimenGroup, excluding loading branches. */
function specimenGroupLabels(source: string): string[] {
  const body = source
    .split("\n")
    .filter((line) => !line.includes("building scene"))
    .join("\n");
  const labels: string[] = [];

  if (body.includes("examples as [label")) {
    labels.push(...[...body.matchAll(/\["([^"]+)",/g)].map((match) => match[1]!));
  }

  for (const match of body.matchAll(/<SpecimenGroup[^>]*\slabel="([^"]+)"/g)) {
    const label = match[1]!;
    if (label !== "{label}") labels.push(label);
  }

  return labels;
}

describe("g15.016 specimen idiom convergence", () => {
  it("keeps dedicated registry mappings for borrowed pages", () => {
    expect(svelteRegistry["list-card-counter"]).toBe(ListCardCounterSpecimen);
    expect(svelteRegistry["meta-item"]).toBe(MetaItemSpecimen);

    const reactMapSource = readFileSync(
      join(PREVIEW_ROOT, "../../react/preview/src/gallery/specimen-map.ts"),
      "utf8",
    );
    expect(reactMapSource).toContain('"list-card-counter": ListCardCounterSpecimen');
    expect(reactMapSource).toContain('"meta-item": MetaItemSpecimen');
    expect(reactMapSource).not.toContain('"list-card-counter": ListCardSpecimen');
    expect(reactMapSource).not.toContain('"meta-item": MetaBarSpecimen');
  });

  it("reports no forked caption idioms in preview specimen sources", () => {
    const svelteSources = specimenSources(SVELTE_SPECIMENS);
    const reactSources = specimenSources(REACT_SPECIMENS);

    expect(svelteSources).not.toMatch(/<section>\s*<h3>/);
    expect(reactSources).not.toMatch(/AudioSpecimenGroup/);

    const eyebrowCaptionUses = [...svelteSources.matchAll(/<Eyebrow>[^<]+<\/Eyebrow>/g)].map((m) => m[0]);
    const allowedEyebrowDemos = [
      "<Eyebrow>Section label</Eyebrow>",
      "<Eyebrow>Primitive</Eyebrow>",
      "<Eyebrow>Composite</Eyebrow>",
      "<Eyebrow>Status</Eyebrow>",
    ];
    for (const match of eyebrowCaptionUses) {
      expect(allowedEyebrowDemos).toContain(match);
    }
  });

  it("scopes all 29 routes to SpecimenGroup caption shells", () => {
    for (const [, baseName] of SCOPED_ROUTE_FILES) {
      const svelteSource = readFileSync(join(SVELTE_SPECIMENS, `${baseName}.svelte`), "utf8");
      const reactSource = readFileSync(join(REACT_SPECIMENS, `${baseName}.tsx`), "utf8");
      expect(svelteSource).toContain("SpecimenGroup");
      expect(reactSource).toContain("SpecimenGroup");
    }
  });

  it("keeps paired caption copy aligned on scoped routes", () => {
    for (const [slug, baseName] of SCOPED_ROUTE_FILES) {
      const svelteSource = readFileSync(join(SVELTE_SPECIMENS, `${baseName}.svelte`), "utf8");
      const reactSource = readFileSync(join(REACT_SPECIMENS, `${baseName}.tsx`), "utf8");
      expect(specimenGroupLabels(svelteSource), slug).toEqual(specimenGroupLabels(reactSource));
    }
  });

  it("moves React audio axis evidence into SpecimenLayout tabs", () => {
    const audioBases = [
      "DragNumberFieldSpecimen",
      "AudioMeterSpecimen",
      "AudioSwitchSpecimen",
      "EnvelopeEditorSpecimen",
      "FaderSpecimen",
      "GainReductionMeterSpecimen",
      "KeyboardSpecimen",
      "KnobSpecimen",
      "ModMatrixGridSpecimen",
      "ValueReadoutSpecimen",
      "WaveformDisplaySpecimen",
      "XYPadSpecimen",
    ];
    for (const baseName of audioBases) {
      const reactSource = readFileSync(join(REACT_SPECIMENS, `${baseName}.tsx`), "utf8");
      expect(reactSource).not.toContain("AudioAxes");
      expect(reactSource).toContain("SpecimenLayout");
      expect(reactSource).toContain("sizes={");
      expect(reactSource).toContain("densities={");
    }
  });
});
