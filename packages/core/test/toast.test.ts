import { describe, expect, test } from "bun:test";

import {
  isToastSticky,
  normalizeToast,
  reconcileToastTimers,
  resolveToastTone,
  uniqueToastInputs,
} from "../src/toast.ts";

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

describe("isToastSticky / uniqueToastInputs / reconcileToastTimers", () => {
  const stickyTones = ["danger"] as const;
  const defaults = { autoDismissMs: 6000, stickyTones };

  test("sticky flag and sticky tones", () => {
    expect(isToastSticky({ id: "1", sticky: true }, stickyTones)).toBe(true);
    expect(isToastSticky({ id: "1", variant: "error" }, stickyTones)).toBe(true);
    expect(isToastSticky({ id: "1", tone: "info" }, stickyTones)).toBe(false);
  });

  test("one live row per id keeps first position and last fields", () => {
    expect(
      uniqueToastInputs([
        { id: "job", title: "First", sticky: true },
        { id: "other", title: "Keep" },
        { id: "job", title: "Last", tone: "success" as const },
      ]),
    ).toEqual([
      { id: "job", title: "Last", tone: "success" },
      { id: "other", title: "Keep" },
    ]);
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
      defaults,
    );

    expect(plan.clear).toEqual(["gone"]);
    expect(plan.start).toEqual(["fresh"]);
    expect(plan.delayMs).toBe(6000);
  });

  test("sticky pending schedules no start", () => {
    const plan = reconcileToastTimers([], [{ id: "job", title: "Publishing", sticky: true }], defaults);
    expect(plan).toEqual({ clear: [], start: [], delayMs: 0 });
  });

  test("become-sticky clears the running clock before it can dismiss", () => {
    const plan = reconcileToastTimers(
      ["job"],
      [{ id: "job", title: "Failed", tone: "danger" }],
      defaults,
    );
    expect(plan.clear).toEqual(["job"]);
    expect(plan.start).toEqual([]);
    expect(plan.delayMs).toBe(0);
  });

  test("default settlement starts exactly one 6000 ms clock", () => {
    const plan = reconcileToastTimers(
      [],
      [{ id: "job", title: "Published", tone: "success" }],
      defaults,
    );
    expect(plan.clear).toEqual([]);
    expect(plan.start).toEqual(["job"]);
    expect(plan.delayMs).toBe(6000);
  });

  test("custom delay is authoritative on become-non-sticky", () => {
    const plan = reconcileToastTimers(
      [],
      [{ id: "job", title: "Published", tone: "success" }],
      { autoDismissMs: 2500, stickyTones },
    );
    expect(plan.start).toEqual(["job"]);
    expect(plan.delayMs).toBe(2500);
    expect(plan.delayMs).not.toBe(6000);
  });

  test("custom sticky tones settle danger with a clock and warning with none", () => {
    const plan = reconcileToastTimers(
      [],
      [
        { id: "fail", title: "Failed", tone: "danger" },
        { id: "slow", title: "Slow", tone: "warning" },
      ],
      { autoDismissMs: 2500, stickyTones: ["warning"] },
    );
    expect(plan.clear).toEqual([]);
    expect(plan.start).toEqual(["fail"]);
    expect(plan.delayMs).toBe(2500);
  });

  test("disabled expiry starts nothing on non-sticky settlement", () => {
    const plan = reconcileToastTimers(
      [],
      [{ id: "job", title: "Published", tone: "success" }],
      { autoDismissMs: 0, stickyTones },
    );
    expect(plan).toEqual({ clear: [], start: [], delayMs: 0 });
  });

  test("negative autoDismissMs starts nothing", () => {
    const plan = reconcileToastTimers([], [{ id: "a" }], { autoDismissMs: -1, stickyTones });
    expect(plan.start).toEqual([]);
    expect(plan.delayMs).toBe(0);
  });

  test("copy tone and action churn do not reset a running clock", () => {
    const plan = reconcileToastTimers(
      ["job"],
      [{ id: "job", title: "Still saving", tone: "info", actionLabel: "Cancel" }],
      defaults,
    );
    expect(plan.clear).toEqual([]);
    expect(plan.start).toEqual([]);
    expect(plan.delayMs).toBe(0);
  });

  test("changing autoDismissMs preserves an existing non-sticky clock", () => {
    const plan = reconcileToastTimers(
      ["job"],
      [{ id: "job", title: "Saved", tone: "success" }],
      { autoDismissMs: 2500, stickyTones },
    );
    expect(plan.clear).toEqual([]);
    expect(plan.start).toEqual([]);
  });

  test("changing stickyTones clears a running row that became sticky", () => {
    const plan = reconcileToastTimers(
      ["job"],
      [{ id: "job", tone: "warning" }],
      { autoDismissMs: 6000, stickyTones: ["warning"] },
    );
    expect(plan.clear).toEqual(["job"]);
    expect(plan.start).toEqual([]);
  });

  test("changing stickyTones starts the current delay when a sticky row becomes non-sticky", () => {
    const plan = reconcileToastTimers(
      [],
      [{ id: "job", tone: "warning" }],
      { autoDismissMs: 2500, stickyTones: [] },
    );
    expect(plan.start).toEqual(["job"]);
    expect(plan.delayMs).toBe(2500);
  });

  test("duplicate ids reconcile against the last fields", () => {
    const plan = reconcileToastTimers(
      ["job"],
      [
        { id: "job", tone: "info" },
        { id: "job", tone: "danger" },
      ],
      defaults,
    );
    expect(plan.clear).toEqual(["job"]);
    expect(plan.start).toEqual([]);
  });

  test("public surface stays API-zero", async () => {
    const source = await Bun.file(new URL("../src/toast.ts", import.meta.url)).text();
    expect(source).not.toMatch(/\bpending\s*\|/);
    expect(source).not.toMatch(/createToastPromise|unwrapPromise|progressSlot/);
    expect(source).not.toMatch(/lifecycle:\s*["']pending["']/);
  });
});
