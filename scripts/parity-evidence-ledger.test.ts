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

  it("rejects missing, duplicate, extra, and unresolved evidence rows", () => {
    const ledger = fs.readFileSync(ledgerPath, "utf8");
    const buttonRow = ledger.split("\n").find((line) => line.startsWith("| Button |"));
    expect(buttonRow).toBeDefined();

    expect(() => validateLedgerText(ledger.replace(`${buttonRow}\n`, ""), root)).toThrow(/missing component rows/);
    expect(() => validateLedgerText(ledger.replace("\n\n## Limitations and measured next gaps", `\n${buttonRow}\n\n## Limitations and measured next gaps`), root)).toThrow(
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
        ledger.replace("a_mounted_button_carries_its_controls_target", "missing_mounted_test_name"),
        root,
      ),
    ).toThrow(/unresolved evidence reference/);
  });

  it("can reproduce the checked-in document from live sources", () => {
    expect(generateLedgerMarkdown(root)).toBe(fs.readFileSync(ledgerPath, "utf8"));
  });
});
