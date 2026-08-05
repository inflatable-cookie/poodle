import { describe, expect, test } from "bun:test";

import { isToastSticky, normalizeToast, reconcileToastTimers, resolveToastTone } from "../src/toast.ts";

describe("resolveToastTone / normalizeToast", () => {
  test("tone wins over variant; variant maps; default info", () => {
    expect(resolveToastTone({ id: "1", tone: "success", variant: "error" })).toBe("success");
    expect(resolveToastTone({ id: "1", variant: "error" })).toBe("danger");
    expect(resolveToastTone({ id: "1", variant: "warning" })).toBe("warning");
    expect(resolveToastTone({ id: "1" })).toBe("info");
  });

  test("title falls back to message then placeholder; message only kept with a real title", () => {
    expect(normalizeToast({ id: "1", title: "Saved", message: "Details" })).toMatchObject({
      title: "Saved",
      message: "Details",
    });
    expect(normalizeToast({ id: "1", title: "  ", message: "Only message" })).toMatchObject({
      title: "Only message",
      message: null,
    });
    expect(normalizeToast({ id: "1" }).title).toBe("Notification");
  });
});

describe("isToastSticky / reconcileToastTimers", () => {
  const stickyTones = ["danger"] as const;

  test("sticky flag and sticky tones", () => {
    expect(isToastSticky({ id: "1", sticky: true }, stickyTones)).toBe(true);
    expect(isToastSticky({ id: "1", variant: "error" }, stickyTones)).toBe(true);
    expect(isToastSticky({ id: "1", tone: "info" }, stickyTones)).toBe(false);
  });

  test("plan clears departed toasts, starts new non-sticky ones, keeps running timers", () => {
    const plan = reconcileToastTimers(
      ["gone", "kept"],
      [
        { id: "kept" },
        { id: "fresh" },
        { id: "alarm", tone: "danger" },
        { id: "pinned", sticky: true },
      ],
      { autoDismissMs: 6000, stickyTones },
    );

    expect(plan.clear).toEqual(["gone"]);
    expect(plan.start).toEqual(["fresh"]);
  });

  test("non-positive autoDismissMs starts nothing", () => {
    const plan = reconcileToastTimers([], [{ id: "a" }], { autoDismissMs: 0, stickyTones });
    expect(plan.start).toEqual([]);
  });
});
