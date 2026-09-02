/**
 * Cross-runtime conformance: runs the shared vectors in
 * packages/contracts/headless/vectors/machines.json against the TS machines.
 * The Rust mirror runs the same vectors (tests/conformance.rs).
 */

import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

import {
  createFaderContext, createKnobContext, faderPointToNorm, faderTransition,
  knobPointToNorm, knobTransition,
} from "../src/audio/value-controls.ts";
import { hitTestCircle, hitTestRect } from "../src/audio/types.ts";
import { createXYPadContext, xyPadPointToNorm, xyPadTransition } from "../src/audio/xy-pad.ts";
import { checkboxTransition } from "../src/checkbox.ts";
import { editLabelTransition } from "../src/edit.ts";
import { disclosureTransition } from "../src/disclosure.ts";
import { dragSessionTransition, resolveDropTarget } from "../src/drag-drop.ts";
import { hoverTransition } from "../src/hover.ts";
import { menuTransition } from "../src/menu.ts";
import { modalTransition } from "../src/modal.ts";
import { selectTransition } from "../src/select.ts";
import { singleSelectTransition } from "../src/single-select.ts";
import {
  rangeSliderTransition,
  sliderTransition,
  snapToStep,
} from "../src/slider.ts";
import { switchTransition } from "../src/switch.ts";
import { tabsTransition } from "../src/tabs.ts";
import { toggleGroupTransition } from "../src/toggle-group.ts";

const vectors = JSON.parse(
  readFileSync(join(import.meta.dir, "..", "..", "contracts", "headless", "vectors", "machines.json"), "utf8"),
);

interface VectorCase {
  name: string;
  state?: string;
  context: Record<string, unknown>;
  event: Record<string, unknown>;
  expect: {
    state?: string;
    context?: Record<string, unknown>;
    order?: string[];
    effects: Record<string, unknown>[];
  };
}

function checkContextSubset(actual: Record<string, unknown>, expected: Record<string, unknown> | undefined): void {
  if (!expected) return;

  for (const [key, value] of Object.entries(expected)) {
    expect(actual[key as keyof typeof actual]).toEqual(value as never);
  }
}

function runMachine(machine: string, vector: VectorCase): void {
  const { context, event } = vector as never as { context: never; event: never };

  switch (machine) {
    case "checkbox": {
      const result = checkboxTransition(context, event);
      checkContextSubset(result.context as never, vector.expect.context);
      expect(result.effects).toEqual(vector.expect.effects as never);
      return;
    }
    case "singleSelect": {
      const result = singleSelectTransition(context, event);
      checkContextSubset(result.context as never, vector.expect.context);
      expect(result.effects).toEqual(vector.expect.effects as never);
      return;
    }
    case "select": {
      const result = selectTransition(context, event);
      checkContextSubset(result.context as never, vector.expect.context);
      expect(result.effects).toEqual(vector.expect.effects as never);
      return;
    }
    case "slider": {
      const result = sliderTransition(context, event);
      checkContextSubset(result.context as never, vector.expect.context);
      expect(result.effects).toEqual(vector.expect.effects as never);
      return;
    }
    case "rangeSlider": {
      const result = rangeSliderTransition(context, event);
      checkContextSubset(result.context as never, vector.expect.context);
      expect(result.effects).toEqual(vector.expect.effects as never);
      return;
    }
    case "sliderSnap": {
      const { min, step } = context as { min: number; step: number };
      const { raw } = event as { raw: number };
      const value = snapToStep(raw, min, step);
      checkContextSubset({ value } as never, vector.expect.context);
      return;
    }
    case "disclosure": {
      const result = disclosureTransition(context, event);
      checkContextSubset(result.context as never, vector.expect.context);
      expect(result.effects).toEqual(vector.expect.effects as never);
      return;
    }
    case "switch": {
      const result = switchTransition(context, event);
      checkContextSubset(result.context as never, vector.expect.context);
      expect(result.effects).toEqual(vector.expect.effects as never);
      return;
    }
    case "toggleGroup": {
      const result = toggleGroupTransition(context, event);
      checkContextSubset(result.context as never, vector.expect.context);
      expect(result.effects).toEqual(vector.expect.effects as never);
      return;
    }
    case "tabs": {
      const result = tabsTransition(context, event);
      checkContextSubset(result.context as never, vector.expect.context);

      if (vector.expect.order) {
        expect(result.context.items.map((item: { value: string }) => item.value)).toEqual(vector.expect.order);
      }

      expect(result.effects).toEqual(vector.expect.effects as never);
      return;
    }
    case "modal": {
      const result = modalTransition(vector.state as never, context, event);
      expect(result.state).toBe(vector.expect.state as never);
      expect(result.effects).toEqual(vector.expect.effects as never);
      return;
    }
    case "hover": {
      const result = hoverTransition(vector.state as never, context, event);
      expect(result.state).toBe(vector.expect.state as never);
      expect(result.effects).toEqual(vector.expect.effects as never);
      return;
    }
    case "menu": {
      const result = menuTransition(vector.state as never, context, event);
      expect(result.state).toBe(vector.expect.state as never);
      expect(result.effects).toEqual(vector.expect.effects as never);
      return;
    }
    case "edit": {
      const result = editLabelTransition(vector.state as never, context, event);
      expect(result.state).toBe(vector.expect.state as never);
      checkContextSubset(result.context as never, vector.expect.context);
      expect(result.effects).toEqual(vector.expect.effects as never);
      return;
    }
    default:
      throw new Error(`no runner for machine: ${machine}`);
  }
}

for (const [machine, cases] of Object.entries(vectors)) {
  if (machine === "description" || machine === "dragDrop" || machine === "audioControls") continue;

  describe(`conformance: ${machine}`, () => {
    for (const vector of cases as VectorCase[]) {
      test(vector.name, () => runMachine(machine, vector));
    }
  });
}

/**
 * The drag session is the one machine whose claims are about ordering across a
 * whole lifecycle rather than a single transition, so its cases are step
 * sequences. Every case starts at `idle` with no session; each step asserts the
 * resulting phase, the effects that step emitted in order, and — where the case
 * pins it — a subset of the resulting session. The Rust mirror runs the same
 * shape (packages/contracts/headless/tests/conformance.rs, drag_drop_conformance).
 */
interface DragStep {
  event: Record<string, unknown>;
  phase: string;
  session?: Record<string, unknown> | null;
  effects: Record<string, unknown>[];
}

interface DragSessionVector {
  name: string;
  steps: DragStep[];
}

interface DragArbitrationVector {
  name: string;
  candidates: Record<string, unknown>[];
  expect: { intent: Record<string, unknown> | null };
}

const dragDrop = vectors.dragDrop as {
  sessions: DragSessionVector[];
  arbitration: DragArbitrationVector[];
};

describe("conformance: dragDrop sessions", () => {
  for (const vector of dragDrop.sessions) {
    test(vector.name, () => {
      let phase = "idle";
      let context: { session: unknown } = { session: null };

      vector.steps.forEach((step, index) => {
        const label = `${vector.name} step ${index} (${String(step.event.type)})`;
        const result = dragSessionTransition(phase as never, context as never, step.event as never);

        expect({ step: label, phase: result.state }).toEqual({ step: label, phase: step.phase });
        expect({ step: label, effects: result.effects }).toEqual({
          step: label,
          effects: step.effects,
        } as never);

        if (step.session !== undefined) {
          if (step.session === null) {
            expect(result.context.session).toBeNull();
          } else {
            checkContextSubset(result.context.session as never, step.session);
          }
        }

        phase = result.state;
        context = result.context;
      });
    });
  }
});

describe("conformance: dragDrop arbitration", () => {
  for (const vector of dragDrop.arbitration) {
    test(vector.name, () => {
      expect(resolveDropTarget(vector.candidates as never)).toEqual(vector.expect.intent as never);
    });
  }
});

/**
 * The continuous-audio controls make lifetime claims — one accepted begin, one
 * terminal, rebase without a jump — so their cases are ordered step sequences
 * over one context rather than single transitions. Every step pins the effects
 * it emitted in order and, where the case claims it, a subset of the resulting
 * context. The Rust mirror runs the same shape
 * (packages/contracts/headless/tests/conformance.rs, audio_controls_conformance).
 */
interface AudioStep {
  event: Record<string, unknown>;
  context?: Record<string, unknown>;
  effects: Record<string, unknown>[];
}

interface AudioVector {
  name: string;
  context: Record<string, unknown>;
  steps: AudioStep[];
}

const audioControls = vectors.audioControls as {
  knob: AudioVector[];
  fader: AudioVector[];
  xyPad: AudioVector[];
  geometry: {
    knob: { name: string; point: never; rect: never; expect: number }[];
    fader: { name: string; point: never; rect: never; orientation: never; expect: number }[];
    xyPad: { name: string; point: never; rect: never; expect: { xNorm: number; yNorm: number } }[];
    hitTest: { name: string; shape: string; point: never; rect: never; expect: boolean }[];
  };
};

function runAudioVectors<Context>(
  control: string,
  cases: AudioVector[],
  create: (input: never) => Context,
  transition: (context: Context, event: never) => { context: Context; effects: unknown[] },
): void {
  describe(`conformance: audioControls ${control}`, () => {
    for (const vector of cases) {
      test(vector.name, () => {
        let context = create(vector.context as never);

        vector.steps.forEach((step, index) => {
          const label = `${vector.name} step ${index} (${String(step.event.type)})`;
          const result = transition(context, step.event as never);

          expect({ step: label, effects: result.effects }).toEqual({
            step: label,
            effects: step.effects,
          } as never);

          checkContextSubset(result.context as never, step.context);
          context = result.context;
        });
      });
    }
  });
}

runAudioVectors("knob", audioControls.knob, createKnobContext, knobTransition);
runAudioVectors("fader", audioControls.fader, createFaderContext, faderTransition);
runAudioVectors("xyPad", audioControls.xyPad, createXYPadContext, xyPadTransition);

describe("conformance: audioControls geometry", () => {
  for (const vector of audioControls.geometry.knob) {
    test(`knob: ${vector.name}`, () => {
      expect(knobPointToNorm(vector.point, vector.rect)).toBe(vector.expect);
    });
  }

  for (const vector of audioControls.geometry.fader) {
    test(`fader: ${vector.name}`, () => {
      expect(faderPointToNorm(vector.point, vector.rect, vector.orientation)).toBe(vector.expect);
    });
  }

  for (const vector of audioControls.geometry.xyPad) {
    test(`xyPad: ${vector.name}`, () => {
      expect(xyPadPointToNorm(vector.point, vector.rect)).toEqual(vector.expect);
    });
  }

  for (const vector of audioControls.geometry.hitTest) {
    test(`hit test: ${vector.name}`, () => {
      const hit = vector.shape === "circle" ? hitTestCircle : hitTestRect;
      expect(hit(vector.point, vector.rect)).toBe(vector.expect);
    });
  }
});
