import { afterEach, describe, expect, it } from "vitest";

import { extractSnapshotNodes } from "./extract";
import type { A1Scenario } from "./contract";

describe("A1 Svelte extractor focus law", () => {
  afterEach(() => {
    document.body.innerHTML = "";
  });

  it("counts a named native radio group as one stop at the checked radio", () => {
    document.body.innerHTML = `
      <div role="radiogroup" aria-label="Plan">
        <input type="radio" name="plan" aria-label="Pro">
        <input type="radio" name="plan" aria-label="Free" checked>
      </div>
    `;

    const scenario = {
      schema: "poodle.g16-nucleus-a11y-scenario.v1",
      component: "Fixture",
      scenario_id: "fixture.radio-group",
      props: {},
      actions: [],
      declared_states: [],
      web_only_exclusions: [],
    } as A1Scenario;

    expect(extractSnapshotNodes(scenario).map((node) => node.focus_order)).toEqual([null, null, 0]);
  });
});
