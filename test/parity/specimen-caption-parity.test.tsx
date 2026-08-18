import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { cleanup as cleanupReact, render as renderReact, waitFor } from "@testing-library/react";
import { cleanup as cleanupSvelte, render as renderSvelte } from "@testing-library/svelte";
import iconNodes from "lucide-static/icon-nodes.json";
import { createElement, type ComponentType } from "react";
import { describe, expect, it } from "vitest";

import { IconProvider, type IconSet } from "../../packages/react/components/src";
import { specimenMap as reactMap } from "../../packages/react/preview/src/gallery/specimen-map";
import { specimenMap as svelteMap } from "../../packages/svelte/preview/src/specimens/registry";
import PilotSpecimenHarness from "../../packages/svelte/preview/test/PilotSpecimenHarness.svelte";

const SVELTE_SPECIMENS = join(import.meta.dirname, "../../packages/svelte/preview/src/specimens");

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

const SCOPED_SLUGS = [
  "split-button",
  "tri-state-switch",
  "select",
  "text-input",
  "token-input",
  "time-input",
  "time-zone-select",
  "eyebrow",
  "alert-dialog",
  "dialog",
  "drawer",
  "menu",
  "markdown-editor",
  "settings-shell",
  "drag-number-field",
  "audio-meter",
  "audio-switch",
  "envelope-editor",
  "fader",
  "gain-reduction-meter",
  "keyboard",
  "knob",
  "mod-matrix-grid",
  "value-readout",
  "waveform-display",
  "xy-pad",
  "meter-surface",
  "list-card-counter",
  "meta-item",
] as const;

/** Captions visible on the Examples tab, in DOM order. */
function captions(root: ParentNode = document): string[] {
  return [...root.querySelectorAll(".poodle-specimen-group")].map((group) =>
    (group.querySelector("[class*=eyebrow]")?.textContent ?? "").trim(),
  );
}

function renderReactSpecimen(Specimen: ComponentType) {
  return renderReact(
    createElement(
      IconProvider,
      { icons: iconNodes as unknown as IconSet },
      createElement(Specimen),
    ),
  );
}

describe("specimen caption parity", () => {
  it("gates the scoped route surface", () => {
    expect(SCOPED_SLUGS.length).toBe(29);
  });

  for (const slug of SCOPED_SLUGS) {
    it(`${slug} renders matching non-blank Examples captions in both runtimes`, async () => {
      const SvelteSpecimen = svelteMap[slug];
      const ReactSpecimen = reactMap[slug];
      expect(SvelteSpecimen, `${slug} missing from Svelte registry`).toBeTruthy();
      expect(ReactSpecimen, `${slug} missing from React registry`).toBeTruthy();

      renderSvelte(PilotSpecimenHarness, {
        props: { specimen: SvelteSpecimen as never },
      });
      if (slug === "meter-surface") {
        await waitFor(() => {
          expect(captions().some((caption) => caption === "Live meter strip")).toBe(true);
        });
      }
      const svelteCaptions = captions();
      cleanupSvelte();

      renderReactSpecimen(ReactSpecimen!);
      if (slug === "meter-surface") {
        await waitFor(() => {
          expect(captions().some((caption) => caption === "Live meter strip")).toBe(true);
        });
      }
      const reactCaptions = captions();
      cleanupReact();

      expect(svelteCaptions.length).toBeGreaterThan(0);
      expect(reactCaptions.length).toBeGreaterThan(0);
      expect(svelteCaptions.filter((caption) => caption === "")).toEqual([]);
      expect(reactCaptions.filter((caption) => caption === "")).toEqual([]);
      expect(reactCaptions).toEqual(svelteCaptions);
    });
  }

  it("declares non-empty SpecimenGroup labels across catalogue sources", () => {
    for (const file of readdirSync(SVELTE_SPECIMENS)) {
      if (!file.endsWith("Specimen.svelte")) continue;
      const source = readFileSync(join(SVELTE_SPECIMENS, file), "utf8");
      if (!source.includes("SpecimenGroup")) continue;
      const labels = specimenGroupLabels(source);
      expect(labels.length, file).toBeGreaterThan(0);
      expect(labels.filter((label) => label.trim() === ""), file).toEqual([]);
    }
  });
});
