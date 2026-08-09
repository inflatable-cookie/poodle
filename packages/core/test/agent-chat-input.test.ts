import { describe, expect, it } from "bun:test";

import {
  actionIcon,
  actionState,
  canSubmit,
  contextPercentage,
  resolveSubmitIntent,
  type ComposerKeyEvent,
} from "../src/agent-chat-input";

function key(overrides: Partial<ComposerKeyEvent> = {}): ComposerKeyEvent {
  return {
    key: "Enter",
    shiftKey: false,
    metaKey: false,
    ctrlKey: false,
    ...overrides,
  };
}

describe("agent chat input", () => {
  it("maps the contract keyboard gestures to composer intent", () => {
    expect(resolveSubmitIntent(key(), { submitOnEnter: true, isBusy: false })).toBe("submit");
    expect(
      resolveSubmitIntent(key({ shiftKey: true }), { submitOnEnter: true, isBusy: false }),
    ).toBe("newline");
    expect(resolveSubmitIntent(key(), { submitOnEnter: false, isBusy: false })).toBe("newline");
    expect(
      resolveSubmitIntent(key({ ctrlKey: true }), { submitOnEnter: false, isBusy: false }),
    ).toBe("submit");
    expect(
      resolveSubmitIntent(key({ metaKey: true }), { submitOnEnter: false, isBusy: false }),
    ).toBe("submit");
  });

  it("never submits the Enter that confirms IME composition", () => {
    expect(
      resolveSubmitIntent(key({ isComposing: true }), { submitOnEnter: true, isBusy: false }),
    ).toBe("newline");
  });

  it("uses Escape only to stop a busy composer", () => {
    const escape = key({ key: "Escape" });
    expect(resolveSubmitIntent(escape, { submitOnEnter: true, isBusy: true })).toBe("stop");
    expect(resolveSubmitIntent(escape, { submitOnEnter: true, isBusy: false })).toBe("none");
  });

  it("gates submission while always allowing stop", () => {
    expect(
      canSubmit({ disabled: false, isBusy: false, value: "", allowEmptySubmit: false }),
    ).toBe(false);
    expect(
      canSubmit({ disabled: false, isBusy: false, value: "hello", allowEmptySubmit: false }),
    ).toBe(true);
    expect(
      canSubmit({ disabled: false, isBusy: false, value: "", allowEmptySubmit: true }),
    ).toBe(true);
    expect(canSubmit({ disabled: false, isBusy: true, value: "", allowEmptySubmit: false })).toBe(
      true,
    );
    expect(canSubmit({ disabled: true, isBusy: true, value: "hello", allowEmptySubmit: true })).toBe(
      false,
    );
  });

  it("clamps context usage and rejects missing limits", () => {
    expect(contextPercentage(null, null)).toBeNull();
    expect(contextPercentage(20, 0)).toBeNull();
    expect(contextPercentage(null, 200)).toBe(0);
    expect(contextPercentage(-10, 200)).toBe(0);
    expect(contextPercentage(50, 200)).toBe(25);
    expect(contextPercentage(300, 200)).toBe(100);
  });

  it("switches action presentation with busy state", () => {
    expect(actionIcon(false)).toBe("arrow-up");
    expect(actionState(false)).toBe("submit");
    expect(actionIcon(true)).toBe("square");
    expect(actionState(true)).toBe("stop");
  });
});
