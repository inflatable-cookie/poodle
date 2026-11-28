import { describe, expect, test } from "bun:test";

import { AXIS_TIER_SLUGS, SKIPPED, SWEEP_AXES } from "./config";
import { DEBT, type VisualDebtEntry } from "./debt";
import { debtFor, findStaleDebt, type Failure } from "./run";

describe("visual debt ledger and report classification", () => {
  test("every debt entry has a non-empty reason, valid sweep axes, and no skipped slug", () => {
    const sweepAxisIds = new Set(SWEEP_AXES.map((axis) => axis.id));
    for (const entry of DEBT) {
      expect(entry.reason.trim().length, `missing reason for ${entry.slug}`).toBeGreaterThan(0);
      expect(SKIPPED[entry.slug], `${entry.slug} is skipped and must not be in DEBT`).toBeUndefined();
      expect(entry.axes.length, `${entry.slug} must list at least one axis`).toBeGreaterThan(0);
      for (const axis of entry.axes) {
        expect(sweepAxisIds.has(axis), `${entry.slug} has unknown axis ${axis}`).toBe(true);
      }
      if (entry.kind === "size") {
        expect(entry.reason).not.toContain("untriaged");
      }
    }
    const sizeEntries = DEBT.filter((entry) => entry.kind === "size");
    const pixelEntries = DEBT.filter((entry) => entry.kind === "pixels");
    const countPairs = (entries: readonly VisualDebtEntry[]) =>
      entries.reduce((sum, entry) => sum + entry.axes.length, 0);
    expect(DEBT.length).toBe(12);
    expect(sizeEntries.length).toBe(0);
    expect(pixelEntries.length).toBe(12);
    expect(countPairs(sizeEntries)).toBe(0);
    expect(countPairs(pixelEntries)).toBe(21);
    expect(countPairs(DEBT)).toBe(21);
  });

  test("strict axis tier contains no debt slugs", () => {
    const debtSlugs = new Set(DEBT.map((entry) => entry.slug));
    for (const slug of AXIS_TIER_SLUGS) {
      expect(debtSlugs.has(slug), `${slug} has open debt and must stay out of AXIS_TIER_SLUGS`).toBe(
        false,
      );
    }
  });

  test("a planted stale debt entry fails findStaleDebt naming the slug and axis", () => {
    const debt: VisualDebtEntry[] = [
      {
        slug: "button",
        axes: ["eclipse-compact-md", "iceberg-compact-md"],
        kind: "pixels",
        reason: "planted stale entry",
      },
    ];
    const executed = new Set(["button::eclipse-compact-md", "button::iceberg-compact-md"]);
    const failures: Failure[] = [
      { slug: "button", axis: "eclipse-compact-md", kind: "pixels", detail: "0.05%" },
    ];
    const stale = findStaleDebt(executed, failures, debt);
    expect(stale).toEqual([
      {
        slug: "button",
        axis: "iceberg-compact-md",
        kind: "pixels",
        reason: "planted stale entry",
      },
    ]);
    expect(debtFor("button", "eclipse-compact-md", "pixels", debt)).toBeDefined();
    expect(debtFor("button", "iceberg-compact-md", "size", debt)).toBeUndefined();
  });

  test("partial runs do not flag unexecuted slugs or axes as stale", () => {
    const debt: VisualDebtEntry[] = [
      {
        slug: "pill",
        axes: ["eclipse-compact-md", "iceberg-compact-md"],
        kind: "pixels",
        reason: "pill drift",
      },
      {
        slug: "avatar",
        axes: ["eclipse-compact-md"],
        kind: "pixels",
        reason: "avatar drift",
      },
    ];
    const executed = new Set(["pill::eclipse-compact-md"]);
    const failures: Failure[] = [
      { slug: "pill", axis: "eclipse-compact-md", kind: "pixels", detail: "0.5%" },
    ];
    expect(findStaleDebt(executed, failures, debt)).toEqual([]);
  });

  test("intermittent entries are never flagged as stale even when all executed axes pass", () => {
    const debt: VisualDebtEntry[] = [
      {
        slug: "text-input",
        axes: ["eclipse-compact-md", "iceberg-compact-md"],
        kind: "pixels",
        intermittent: true,
        reason: "flips between axes",
      },
    ];
    const executed = new Set(["text-input::eclipse-compact-md", "text-input::iceberg-compact-md"]);
    const oneFails: Failure[] = [
      { slug: "text-input", axis: "iceberg-compact-md", kind: "pixels", detail: "0.068%" },
    ];
    expect(findStaleDebt(executed, oneFails, debt)).toEqual([]);
    expect(findStaleDebt(executed, [], debt)).toEqual([]);
  });

  test("skipped or absent slugs in debt are flagged on full sweep", () => {
    const debt: VisualDebtEntry[] = [
      {
        slug: "embed-preview",
        axes: ["eclipse-compact-md"],
        kind: "pixels",
        reason: "remote iframe",
      },
    ];
    const stale = findStaleDebt(new Set(), [], debt, {
      skipped: SKIPPED,
      knownSlugs: new Set(["button"]),
    });
    expect(stale).toHaveLength(1);
    expect(stale[0]?.slug).toBe("embed-preview");
    expect(stale[0]?.reason).toContain("skipped");
  });
});
