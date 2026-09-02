import fs from "node:fs";
import path from "node:path";
import { describe, expect, it, setDefaultTimeout } from "bun:test";
import { deriveLiveRoster, generateLedgerMarkdown, validateLedgerText } from "./parity-evidence-ledger";

const root = path.resolve(import.meta.dir, "..");
const ledgerPath = path.join(root, "docs/roadmaps/g16/parity-evidence-ledger.md");
setDefaultTimeout(30_000);

describe("g16.001 parity evidence ledger", () => {
  it("derives the fixed 176/175 roster", () => {
    const roster = deriveLiveRoster(root);
    expect(roster).toHaveLength(176);
    expect(roster.filter((component) => component.portable)).toHaveLength(175);
    expect(roster.find((component) => component.name === "MeterSurface")?.portable).toBe(false);
  });

  it("accepts the checked-in ledger", () => {
    validateLedgerText(fs.readFileSync(ledgerPath, "utf8"), root);
  });

  it("recognizes numbered Known Deltas headings", () => {
    const buttonRow = generateLedgerMarkdown(root)
      .split("\n")
      .find((line) => line.startsWith("| Button |"));
    expect(buttonRow).toContain("docs/contracts/components/button.md#Known Deltas");
    expect(buttonRow).not.toContain("Known Deltas` | not-applicable");
  });

  it("keeps an expected mounted test missing until a receipt exists", () => {
    const ledger = generateLedgerMarkdown(root);
    const switchRow = ledger.split("\n").find((line) => line.startsWith("| Switch |"));
    expect(switchRow).toContain("expected `docs/roadmaps/g16/nucleus-parity-manifest.json#nucleus.settings.switch`");
    expect(switchRow).toContain("no validated M1 receipt");
    expect(ledger).toContain(
      "| Switch | `packages/gpui/preview/tests/headless_regressions.rs#switch_toggle_readonly_and_disabled_rebuild_the_host_spec` | expected only |",
    );
  });

  it("rejects missing, duplicate, extra, and unresolved evidence rows", () => {
    const ledger = fs.readFileSync(ledgerPath, "utf8");
    const buttonRow = ledger.split("\n").find((line) => line.startsWith("| Button |"));
    expect(buttonRow).toBeDefined();

    expect(() => validateLedgerText(ledger.replace(`${buttonRow}\n`, ""), root)).toThrow(/missing component rows/);
    expect(() => validateLedgerText(ledger.replace(`${buttonRow}\n`, `${buttonRow}\n${buttonRow}\n`), root)).toThrow(
      /duplicate component rows/,
    );
    expect(() => validateLedgerText(ledger.replace("| Button |", "| NotAComponent |"), root)).toThrow(
      /missing component rows|extra component rows/,
    );
    expect(() =>
      validateLedgerText(
        ledger.replace("docs/contracts/components/button.md", "docs/contracts/components/does-not-exist.md"),
        root,
      ),
    ).toThrow(/unresolved claim|unresolved evidence path/);
    expect(() =>
      validateLedgerText(
        ledger.replace("a_pointer_press_reaches_the_backend_listener_once", "missing_mounted_test_name"),
        root,
      ),
    ).toThrow(/unresolved evidence reference|unresolved claim/);
  });

  it("can reproduce the checked-in document from live sources", () => {
    expect(generateLedgerMarkdown(root)).toBe(fs.readFileSync(ledgerPath, "utf8"));
  });
});
