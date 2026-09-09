import fs from "node:fs";
import path from "node:path";
import { describe, expect, it, setDefaultTimeout } from "bun:test";
import { deriveLiveRoster, generateLedgerMarkdown, validateLedgerText } from "./parity-evidence-ledger";

const root = path.resolve(import.meta.dir, "..");
const ledgerPath = path.join(root, "docs/evidence/nucleus/parity-evidence-ledger.md");
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

  it("keeps an expected mounted test distinct from receipt evidence", () => {
    const ledger = generateLedgerMarkdown(root);
    const calloutRow = ledger.split("\n").find((line) => line.startsWith("| Callout |"));
    expect(calloutRow).toContain(
      "mounted — validated `docs/evidence/nucleus/nucleus-parity-receipts/callout--nucleus-settings-callout.json#proof_level`",
    );
    expect(calloutRow).not.toContain("no validated M1 receipt");
    expect(ledger).toContain(
      "| Callout | `packages/gpui/preview/tests/headless_regressions.rs#callout_dismiss_rebuilds_the_host_spec_through_mounted_input` | expected only |",
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
    ).toThrow(/unresolved evidence reference|unresolved claim|Nucleus receipt ledger row/);
  });

  it("can reproduce the checked-in document from live sources", () => {
    expect(generateLedgerMarkdown(root)).toBe(fs.readFileSync(ledgerPath, "utf8"));
  });
});

describe("g17.001 Nucleus V1 ledger cells", () => {
  it("moves only V1-backed GPUI visual cells to compared with findings open", () => {
    const ledger = generateLedgerMarkdown(root);
    expect(ledger).toContain("| GPUI visual | 0 | 0 | 0 | 29 | 0 | 146 | 1 | 0 |");
    const buttonRow = ledger.split("\n").find((line) => line.startsWith("| Button |"));
    expect(buttonRow).toContain("button--nucleus-shell-button--v1.json#proof_level");
    expect(buttonRow).toContain("retained as open evidence");
    expect(buttonRow).toContain("test/visual/fixtures/button-visual-inventory.json");
    const v1Cells = ledger
      .split("\n")
      .flatMap((line) => line.split("|").map((cell) => cell.trim()))
      .filter((cell) => cell.includes("--v1.json"));
    expect(v1Cells.length).toBeGreaterThan(29);
    for (const cell of v1Cells) {
      expect(cell).toContain("open evidence");
      expect(cell).not.toMatch(/accept/i);
    }
  });

  it("rejects an unbacked compared GPUI visual cell", () => {
    const ledger = fs.readFileSync(ledgerPath, "utf8");
    const forged = ledger.replace(
      "missing — Button-only comparison boundary; no GPUI comparison fixture for Accordion in",
      "compared — fabricated without a validated V1 receipt for Accordion in",
    );
    expect(forged).not.toBe(ledger);
    expect(() => validateLedgerText(forged, root)).toThrow(/differs from live evidence/);
  });

  it("keeps every Nucleus V1 cell compared with retained findings", () => {
    const ledger = generateLedgerMarkdown(root);
    const section = ledger.split("## Nucleus fixed cohort execution ledger")[1]?.split("## Historical")[0] ?? "";
    const rows = section.split("\n").filter((line) => line.startsWith("| ") && !line.startsWith("| Component") && !line.startsWith("| ---"));
    expect(rows).toHaveLength(29);
    for (const row of rows) {
      expect(row).toContain("--v1.json#proof_level");
      expect(row).toContain("compared — validated");
      expect(row).toContain("findings retained as open evidence");
    }
  });
});
