/** Read the GPUI proof window's live macOS accessibility tree without activation. */
import { spawnSync } from "node:child_process";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const PREVIEW = new URL("..", import.meta.url).pathname;
const REPO = new URL("../../../..", import.meta.url).pathname;
const MANIFEST = join(PREVIEW, "Cargo.toml");
const BIN = join(PREVIEW, "target", "debug", "poodle-window-capture");
const PROBE_SOURCE = join(REPO, "test/native-visual/gpui-ax-probe.swift");
const PROOF_WINDOW_TITLE = "Poodle GPUI AX proof";
const PROBE_TIMEOUT_MS = 9_000;
const PROBE_ATTEMPT_TIMEOUT_MS = 2_000;
const EXIT_TIMEOUT_MS = 15_000;

type Element = {
  depth: number;
  role: string;
  subrole: string;
  title: string;
  description: string;
  name: string;
  attributes: string[];
  value: string;
  minimum: string;
  maximum: string;
  orientation: string;
  enabled: string;
  expanded: string;
  selected: string;
};

type ProbeResult = { status: string; reason: string; elements: Element[] };

const STRUCTURAL_ROLES = new Set([
  "AXApplication",
  "AXWindow",
  "AXGroup",
  "AXUnknown",
  "AXScrollArea",
]);
const SYSTEM_ROLES = new Set(["AXMenuBar", "AXMenu", "AXMenuItem", "AXMenuBarItem"]);
const PROOF_NAMES = {
  save: "GPUI AX proof: Save",
  details: "GPUI AX proof: Details",
  mixed: "GPUI AX proof: Mixed state",
  level: "GPUI AX proof: Output level",
  selected: "GPUI AX proof: Selected tab",
  disabled: "GPUI AX proof: Disabled button",
};

function check(label: string, ok: boolean, detail = ""): void {
  console.log(`  ${ok ? "PASS" : "FAIL"}  ${label}${detail ? `: ${detail}` : ""}`);
  if (!ok) throw new Error(`${label}${detail ? `: ${detail}` : ""}`);
}

function build(command: string, args: string[], label: string): void {
  const result = spawnSync(command, args, { encoding: "utf8" });
  if (result.status !== 0) {
    throw new Error(`${label} failed (status ${result.status})\n${result.stdout}\n${result.stderr}`);
  }
}

async function probeUntilReady(pid: number, probePath: string): Promise<ProbeResult> {
  const deadline = Date.now() + PROBE_TIMEOUT_MS;
  let last: ProbeResult | undefined;
  let lastProbeError: string | undefined;
  while (Date.now() < deadline) {
    const attemptTimeoutMs = Math.min(PROBE_ATTEMPT_TIMEOUT_MS, Math.max(1, deadline - Date.now()));
    const result = spawnSync(probePath, [String(pid), PROOF_WINDOW_TITLE], {
      encoding: "utf8",
      timeout: attemptTimeoutMs,
    });
    if (result.error?.code === "ETIMEDOUT") {
      lastProbeError = `AXUIElement probe exceeded ${attemptTimeoutMs}ms`;
      await Bun.sleep(250);
      continue;
    }
    if (result.error) {
      throw new Error(`AXUIElement probe could not run: ${result.error.message}`);
    }
    if (result.status === 3) {
      throw new Error(
        `AXUIElement probe is not trusted:\n${result.stdout.trim()}\n${result.stderr.trim()}`,
      );
    }
    if (result.status !== 0) {
      throw new Error(`AXUIElement probe failed (status ${result.status})\n${result.stderr}`);
    }
    try {
      last = JSON.parse(result.stdout) as ProbeResult;
    } catch {
      throw new Error(`AXUIElement probe returned invalid JSON: ${result.stdout}`);
    }
    lastProbeError = undefined;
    if (last.status === "walk_error") {
      throw new Error(`AXUIElement tree walk failed: ${last.reason}`);
    }
    if (last.status === "ready") return last;
    await Bun.sleep(250);
  }
  const lastFailure = [last?.reason, lastProbeError].filter(Boolean).join("; ");
  throw new Error(
    `proof window root and named Poodle content did not become ready in the non-activating platform tree within ${PROBE_TIMEOUT_MS}ms` +
      (lastFailure ? ` (${lastFailure})` : ""),
  );
}

function named(elements: Element[], role: string, name: string): Element | undefined {
  return elements.find((element) => element.role === role && element.name === name);
}

function printTreeEvidence(elements: Element[]): void {
  console.log(`  AX tree evidence (${elements.length} elements):`);
  for (const element of elements) {
    console.log(
      `    depth=${element.depth} role=${JSON.stringify(element.role)} subrole=${JSON.stringify(element.subrole)} title=${JSON.stringify(element.title)} description=${JSON.stringify(element.description)} name=${JSON.stringify(element.name)} attributes=${JSON.stringify(element.attributes)}`,
    );
  }
}

function isTrue(value: string): boolean {
  return value === "1" || value.toLowerCase() === "true" || value.toLowerCase() === "yes";
}

function meaningfulUnnamed(elements: Element[]): Element[] {
  return elements.filter(
    (element) =>
      element.depth >= 2 &&
      !STRUCTURAL_ROLES.has(element.role) &&
      !SYSTEM_ROLES.has(element.role) &&
      element.name.trim() === "",
  );
}

function verifyPoodleContent(elements: Element[]): void {
  // accesskit_macos presents a toggled AccessKit Button as AXCheckBox with
  // the AXToggle subrole. Preserve the source name while checking that native
  // platform mapping instead of assuming the unspecialized AXButton role.
  const save = named(elements, "AXCheckBox", PROOF_NAMES.save);
  check("live pressed Button name and macOS toggle role", save !== undefined && save.subrole === "AXToggle");
  check("Button toggled state reaches AXValue", isTrue(save?.value ?? ""));

  const details = named(elements, "AXButton", PROOF_NAMES.details);
  check("live disclosure Button role and name", details !== undefined);

  const mixed = named(elements, "AXCheckBox", PROOF_NAMES.mixed);
  check("live Poodle Checkbox role and name", mixed !== undefined);
  check("Checkbox state reaches AXValue", mixed?.value !== undefined && mixed.value !== "");

  const level = named(elements, "AXSlider", PROOF_NAMES.level);
  check("live Poodle Slider role and name", level !== undefined);
  check(
    "slider minimum and maximum reach the platform tree",
    level !== undefined && Number(level.minimum) === 0 && Number(level.maximum) === 100,
  );
  check(
    "slider orientation reaches the platform tree",
    level?.orientation === "AXHorizontalOrientation",
    level?.orientation ?? "missing",
  );
  const numericValue = Number(level?.value);
  check(
    "slider numeric value or value text reaches AXValue",
    level?.value === "42.5 percent" || (Number.isFinite(numericValue) && Math.abs(numericValue - 42.5) < 0.01),
    level?.value ?? "missing",
  );

  const selected = elements.find(
    (element) =>
      element.role === "AXRadioButton" &&
      element.subrole === "AXTabButton" &&
      element.name === PROOF_NAMES.selected,
  );
  // The macOS adapter exposes selected Tabs as AXRadioButtons and publishes
  // selection through AXValue rather than AXSelected.
  check("selected Tab state reaches AXValue", selected !== undefined && isTrue(selected.value));
  const disabled = named(elements, "AXButton", PROOF_NAMES.disabled);
  check("disabled state reaches the platform tree", disabled !== undefined && !isTrue(disabled.enabled));
}

async function runWindow(probePath: string, planted: boolean): Promise<void> {
  const label = planted ? "planted unnamed control" : "named Poodle preview";
  console.log(`## ${label}`);
  const args = ["--a11y-proof", ...(planted ? ["--plant-unnamed"] : [])];
  const child = Bun.spawn([BIN, ...args], { stdout: "pipe", stderr: "pipe" });
  const stdout = new Response(child.stdout).text();
  const stderr = new Response(child.stderr).text();

  let probe: ProbeResult | undefined = undefined;
  let auditError: unknown;
  try {
    probe = await probeUntilReady(child.pid, probePath);
    printTreeEvidence(probe.elements);
    verifyPoodleContent(probe.elements);

    const unnamed = meaningfulUnnamed(probe.elements);
    if (planted) {
      check(
        "the name audit rejects a planted unnamed control",
        unnamed.some((element) => element.role === "AXButton"),
        unnamed.map((element) => `${element.role}@${element.depth}`).join(", "),
      );
    } else {
      check(
        "every meaningful element in the clean tree has a name",
        unnamed.length === 0,
        unnamed.map((element) => `${element.role}@${element.depth}`).join(", "),
      );
    }
  } catch (error) {
    auditError = error;
    console.error(`  platform-tree attempt: ${error instanceof Error ? error.message : String(error)}`);
  }

  const exit = await Promise.race([
    child.exited,
    Bun.sleep(EXIT_TIMEOUT_MS).then(() => undefined),
  ]);
  if (exit === undefined) {
    child.kill();
    await child.exited;
    throw new Error(`the non-activating GPUI proof process exceeded ${EXIT_TIMEOUT_MS}ms`);
  }

  const [out, err] = await Promise.all([stdout, stderr]);
  console.log(out.trim());
  if (err.trim()) console.error(err.trim());
  check("GPUI foreground monitor proves no activation", exit === 0 && out.includes("GPUI_AX_COMPLETE foreground=proved"));
  if (probe === undefined) {
    throw new Error(
      "the window stayed non-frontmost, but no platform content was readable; stop here if activation would be required",
    );
  }
  if (auditError !== undefined) throw auditError;
}

async function main(): Promise<void> {
  console.log("## build live GPUI proof window");
  build(
    "cargo",
    ["build", "--quiet", "--manifest-path", MANIFEST, "--bin", "poodle-window-capture", "--features", "window-capture"],
    "GPUI accessibility proof build",
  );

  const work = mkdtempSync(join(tmpdir(), "poodle-gpui-ax-"));
  try {
    const probePath = join(work, "gpui-ax-probe");
    build(
      "swiftc",
      ["-O", "-module-cache-path", join(work, "module-cache"), "-o", probePath, PROBE_SOURCE],
      "macOS AXUIElement probe build",
    );
    await runWindow(probePath, false);
    await runWindow(probePath, true);
  } finally {
    rmSync(work, { recursive: true, force: true });
  }
}

main().catch((error: unknown) => {
  console.error(error instanceof Error ? error.message : String(error));
  process.exitCode = 1;
});
