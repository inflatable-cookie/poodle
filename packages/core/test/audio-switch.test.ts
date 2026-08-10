import { describe, expect, test } from "bun:test";
import { audioSwitchTransition, audioSwitchVisualState, createAudioSwitchContext } from "../src/audio/audio-switch";

describe("audio switch machine", () => {
  test("latch changes only on release", () => {
    let context = createAudioSwitchContext({ mode: "latch" });
    let result = audioSwitchTransition(context, { type: "PRESS" });
    expect(result.context).toMatchObject({ state: 0, pressed: true });
    expect(result.effects).toEqual([]);
    result = audioSwitchTransition(result.context, { type: "RELEASE" });
    expect(result.context).toMatchObject({ state: 1, pressed: false });
    expect(result.effects).toEqual([{ type: "emitStateChange", state: 1 }, { type: "emitStateCommit", state: 1 }]);
  });

  test("momentary reports both edges and cancellation releases it", () => {
    let context = createAudioSwitchContext({ mode: "momentary" });
    let result = audioSwitchTransition(context, { type: "PRESS" });
    expect(result.effects).toEqual([{ type: "emitStateChange", state: 1 }]);
    context = result.context;
    result = audioSwitchTransition(context, { type: "CANCEL" });
    expect(result.context).toMatchObject({ state: 0, pressed: false });
    expect(result.effects).toEqual([{ type: "emitStateChange", state: 0 }, { type: "emitStateCommit", state: 0 }]);
  });

  test("multi cycles and normalizes state bounds", () => {
    let context = createAudioSwitchContext({ mode: "multi", state: 2, stateCount: 3 });
    context = audioSwitchTransition(context, { type: "PRESS" }).context;
    expect(audioSwitchTransition(context, { type: "RELEASE" }).context.state).toBe(0);
    expect(createAudioSwitchContext({ state: 9, stateCount: 3 }).state).toBe(2);
  });

  test("lamp can differ from state and VisualState is serializable", () => {
    const context = createAudioSwitchContext({ state: 1, lampOn: false });
    const visual = audioSwitchVisualState(context);
    expect(visual).toMatchObject({ state: 1, lampOn: false });
    expect(JSON.parse(JSON.stringify(visual))).toEqual(visual);
  });

  test("disabled and duplicate press paths are inert", () => {
    const disabled = createAudioSwitchContext({ disabled: true });
    expect(audioSwitchTransition(disabled, { type: "PRESS" })).toEqual({ context: disabled, effects: [] });
    const pressed = audioSwitchTransition(createAudioSwitchContext(), { type: "PRESS" }).context;
    expect(audioSwitchTransition(pressed, { type: "PRESS" })).toEqual({ context: pressed, effects: [] });
  });
});
