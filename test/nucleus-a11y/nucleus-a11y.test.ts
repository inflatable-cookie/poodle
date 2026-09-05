// g16.111 — Svelte half of the A1 paired accessibility receipt.
//
// For every shared scenario: mount the Svelte component with the scenario's
// props, replay its actions through DOM events, extract the accessibility
// snapshot, and prove the committed Svelte snapshot is exactly this run's
// output for this scenario hash. When a committed GPUI snapshot exists for
// the row, the two are compared here as well, so a Svelte-side drift bites
// in this project and a GPUI-side drift bites in `effigy regressions:native`.
//
// Regenerate the committed Svelte snapshots with
// `POODLE_NUCLEUS_A11Y_WRITE=1 effigy test:nucleus-a11y`.

import { render } from "@testing-library/svelte";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { createRawSnippet } from "svelte";
import { writable } from "svelte/store";
import { describe, expect, it } from "vitest";

import * as components from "@inflatable-cookie/poodle-svelte";

import {
  A1_SNAPSHOT_SCHEMA,
  diffSnapshotNodes,
  listScenarioRows,
  readScenario,
  serializeSnapshot,
  snapshotPath,
  SVELTE_RUN_RECORD,
  A1_SVELTE_RUNTIME,
  type A1Scenario,
  type SnapshotFile,
} from "./contract";
import { extractSnapshotNodes, replayActions, settle } from "./extract";

const root = path.resolve(import.meta.dirname, "../..");
const write = process.env.POODLE_NUCLEUS_A11Y_WRITE === "1";

/// Slot content the scenario fixes as text. Shared with the Rust proof,
/// which renders the same text as a node.
function fixtureProps(scenario: A1Scenario): Record<string, unknown> {
  if (scenario.component === "ToastHost") {
    const toasts = writable((scenario.props.toasts ?? []) as never[]);
    return { ...scenario.props, store: { toasts, dismiss: () => {} } };
  }
  const panelText = scenario.fixtures?.panel_text;
  const triggerText = scenario.fixtures?.trigger_text;
  const props: Record<string, unknown> = {};
  if (typeof panelText === "string") {
    props.children = panelText.includes("{value}")
      ? createRawSnippet((value: () => string) => ({
          render: () => `<p>${panelText.replace("{value}", value())}</p>`,
        }))
      : createRawSnippet(() => ({ render: () => `<p>${panelText}</p>` }));
  }
  if (typeof triggerText === "string") {
    props.trigger = createRawSnippet(() => ({
      render: () => `<button type="button">${triggerText}</button>`,
    }));
  }
  if (scenario.component === "MessageCenter") {
    props.onItemSelect = () => {};
  }
  return props;
}

describe("g16.111 Nucleus A1 Svelte accessibility snapshots", () => {
  const rows = listScenarioRows(root);

  it("has the foundation, NP-2, and NP-5 scenarios", () => {
    expect(rows).toEqual(["command-palette", "dialog", "editable-label", "menu", "message-center", "popover", "segmented-control", "select", "switch", "tabs", "toast-host"]);
  });

  for (const row of rows) {
    it(`${row}: the executed Svelte projection is the committed snapshot`, async () => {
      const loaded = readScenario(root, row);
      const Component = (components as Record<string, unknown>)[loaded.scenario.component];
      expect(Component, `${loaded.scenario.component} is a public Svelte export`).toBeDefined();

      const props = loaded.scenario.component === "ToastHost"
        ? fixtureProps(loaded.scenario)
        : { ...loaded.scenario.props, ...fixtureProps(loaded.scenario) };
      render(Component as never, { props });
      await settle();
      await replayActions(loaded.scenario.actions);

      const file: SnapshotFile = {
        schema: A1_SNAPSHOT_SCHEMA,
        component: loaded.scenario.component,
        scenario_id: loaded.scenario.scenario_id,
        scenario_path: loaded.path,
        scenario_sha256: loaded.sha256,
        runtime: A1_SVELTE_RUNTIME,
        run: SVELTE_RUN_RECORD,
        nodes: extractSnapshotNodes(loaded.scenario),
      };
      expect(file.nodes.length, "the mounted DOM exposes at least one role").toBeGreaterThan(0);

      const committedPath = path.join(root, snapshotPath(row, "svelte"));
      if (write) {
        writeFileSync(committedPath, serializeSnapshot(file));
      } else {
        expect(existsSync(committedPath), `${snapshotPath(row, "svelte")} is missing; run with POODLE_NUCLEUS_A11Y_WRITE=1`).toBe(true);
        expect(JSON.parse(readFileSync(committedPath, "utf8"))).toEqual(file);
      }

      const gpuiPath = path.join(root, snapshotPath(row, "gpui"));
      if (existsSync(gpuiPath)) {
        const gpui = JSON.parse(readFileSync(gpuiPath, "utf8")) as SnapshotFile;
        expect(gpui.scenario_sha256, `${snapshotPath(row, "gpui")} ran against a different scenario file`).toBe(loaded.sha256);
        expect(diffSnapshotNodes(gpui.nodes, file.nodes)).toEqual([]);
      }
    });
  }
});
