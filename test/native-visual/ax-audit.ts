import { existsSync } from "node:fs";
import path from "node:path";

import { repoRoot } from "./config";

/**
 * Read the Jetstream preview's **real macOS accessibility tree**.
 *
 *   bun test/native-visual/ax-audit.ts            # audit, fail on unnamed content
 *   bun test/native-visual/ax-audit.ts --dump     # print the whole tree
 *
 * The Rust tests prove `tree_update` builds the right AccessKit nodes. That is
 * our code agreeing with our code. This goes out through `AXUIElement` — the
 * same API a screen reader uses — so the answer comes from macOS.
 *
 * Two things had to be discovered rather than assumed, and both are why this
 * exists as a script instead of a one-off:
 *
 * 1. **The app must be activated.** An unactivated process exposes *nothing* —
 *    not even `AXApplication`. Probing before activation looks exactly like a
 *    broken adapter, and did for a while. AccessKit builds its tree lazily and
 *    the app becoming frontmost is what asks for it.
 * 2. **The system menu bar dominates the count.** ~86 `AXMenuItem`s belong to
 *    macOS, not to us, and most unnamed elements are its separators. Auditing
 *    raw totals would report a permanent, meaningless failure, so menu roles
 *    are excluded from the naming check.
 *
 * Measured 2026-07-27: **571 elements, 471 of them ours, 467 named**, every
 * unnamed one structural or part of the system menu. That run also found three
 * real defects — an unnamed contrast slider, an unnamed search field, and a
 * decorative search icon being announced — all since fixed.
 *
 * **Caveat on that measurement:** it predates the cycle-detection fix in
 * `ax-probe.swift`, and the display locked before the fixed probe could be run
 * against a real content tree. The numbers above are real; the current probe
 * needs one green run on an unlocked display to confirm the fix did not change
 * them. For contrast, the GPUI preview on the same machine exposes
 * **7 elements, 1 named** — `AXApplication`, `AXWindow`, the three traffic
 * lights and the title. That is AppKit's window chrome; none of GPUI's content
 * is in it, which is what "no accessibility API" looks like from the outside.
 */

const PREVIEW = "packages/jetstream/preview/target/debug/poodle-jetstream-preview";
const PROBE_SRC = "test/native-visual/ax-probe.swift";
const PROBE_BIN = "/tmp/poodle-ax-probe";

/** Roles owned by macOS rather than by us. */
const SYSTEM_ROLES = new Set(["AXMenuBar", "AXMenu", "AXMenuItem", "AXMenuBarItem"]);

/** Roles that are structural and legitimately unnamed. */
const STRUCTURAL_ROLES = new Set(["AXGroup", "AXUnknown", "AXScrollArea", ""]);

const dump = process.argv.includes("--dump");

type Element = { depth: number; role: string; name: string; value: string };

async function buildProbe(): Promise<void> {
  if (existsSync(PROBE_BIN)) return;
  const proc = Bun.spawnSync(["swiftc", "-O", "-o", PROBE_BIN, path.join(repoRoot, PROBE_SRC)], {
    stderr: "pipe",
  });
  if (!proc.success) throw new Error(`probe build failed:\n${proc.stderr.toString()}`);
}

/** Bring `pid` to the front. Without this the tree is empty — see the note above. */
function activate(pid: number): boolean {
  const proc = Bun.spawnSync([
    "osascript",
    "-e",
    `tell application "System Events" to set frontmost of (first process whose unix id is ${pid}) to true`,
  ], { stderr: "pipe" });
  if (!proc.success) {
    // Silent failure here is indistinguishable from a broken adapter: both
    // produce an empty tree. Say which it was.
    console.error(`could not activate pid ${pid}: ${proc.stderr.toString().trim()}`);
  }
  return proc.success;
}

/**
 * Whether the tree is actually built, rather than merely present.
 *
 * `AXApplication` alone is not enough. Probed too early, the application
 * element reports *itself* as its own child, and the probe walks that cycle
 * until its depth guard stops it — 38 nested `AXApplication`s and 94
 * `DEPTH-LIMIT` markers, which passed an "is there an AXApplication?" check
 * perfectly happily. Readiness is a window with real content under it.
 */
function isReady(elements: Element[]): boolean {
  if (!elements.some((el) => el.role === "AXWindow")) return false;
  if (elements.some((el) => el.role === "DEPTH-LIMIT" || el.role === "CYCLE")) return false;
  return elements.some((el) => el.depth >= 3 && el.name !== "");
}

function probe(pid: number): { elements: Element[]; raw: string } {
  const proc = Bun.spawnSync([PROBE_BIN, String(pid)], { stdout: "pipe", stderr: "pipe" });
  const raw = proc.stdout.toString();
  if (raw.startsWith("UNTRUSTED")) {
    throw new Error(
      "This process lacks Accessibility permission, so the probe cannot tell\n" +
        "'the app exposes nothing' from 'we are not allowed to look'.\n" +
        "Grant it in System Settings > Privacy & Security > Accessibility.",
    );
  }
  const elements = raw
    .split("\n")
    .filter((line) => /^\d+\|/.test(line))
    .map((line) => {
      const [depth, role, name, value] = line.split("|");
      return { depth: Number(depth), role, name, value: value ?? "" };
    });
  return { elements, raw };
}

/**
 * Whether the login session is locked.
 *
 * A locked screen never composites the window, so macOS builds no content tree
 * for it and the probe comes back with the system menu bar and nothing else —
 * identical to a broken adapter. This cost an hour of chasing a regression that
 * was not there. `014` learned the same lesson about `screencapture`; this is
 * the same machine state defeating a different API.
 */
function screenIsLocked(): boolean {
  const proc = Bun.spawnSync(["ioreg", "-n", "Root", "-d1", "-r"], { stdout: "pipe" });
  return proc.stdout.toString().includes('CGSSessionScreenIsLocked"=Yes');
}

if (screenIsLocked()) {
  console.error(
    "The screen is locked, so no window is composited and macOS exposes no\n" +
      "accessible content for it — which looks exactly like an app that\n" +
      "implements nothing. Unlock the display and re-run.",
  );
  process.exit(2);
}

const previewPath = path.join(repoRoot, PREVIEW);
if (!existsSync(previewPath)) {
  console.error(`preview binary missing: ${PREVIEW}\nbuild it first (cargo build in that crate)`);
  process.exit(1);
}

await buildProbe();

let failed = false;

console.log("launching the Jetstream preview…");
const preview = Bun.spawn([previewPath], { stdout: "ignore", stderr: "ignore" });
try {
  // Activation is not instant and AccessKit builds its tree lazily, so this
  // retries rather than sleeping on a guess — the same mistake the GPUI
  // screenshot gate spent five rounds on.
  await Bun.sleep(4000);
  let elements: Element[] = [];
  for (let attempt = 1; attempt <= 8; attempt++) {
    if (preview.killed) throw new Error("the preview exited before it could be probed");
    activate(preview.pid);
    await Bun.sleep(1500);
    elements = probe(preview.pid).elements;
    if (isReady(elements)) {
      console.log(`  tree appeared on attempt ${attempt}`);
      break;
    }
  }

  if (dump) {
    for (const el of elements) {
      console.log(`${"  ".repeat(el.depth)}${el.role}${el.name ? ` — ${el.name}` : ""}`);
    }
  }

  // Depth 0 is the application, 1 is the window, and 2 is AppKit's own window
  // chrome — the three traffic lights and the title. The GPUI preview, which
  // has no accessibility implementation at all, exposes exactly those and
  // nothing else, which is how we know they are not ours. AccessKit's content
  // starts at depth 3.
  const ours = elements.filter((el) => !SYSTEM_ROLES.has(el.role) && el.depth >= 3);
  const named = ours.filter((el) => el.name !== "");
  const unnamed = ours.filter((el) => el.name === "" && !STRUCTURAL_ROLES.has(el.role));

  console.log(`\n${elements.length} elements total, ${ours.length} ours, ${named.length} named`);

  const byRole = new Map<string, number>();
  for (const el of ours) byRole.set(el.role, (byRole.get(el.role) ?? 0) + 1);
  console.log("roles: " + [...byRole].sort((a, b) => b[1] - a[1]).map(([r, n]) => `${r}=${n}`).join(" "));

  // An empty tree means the adapter is not surfacing at all, which is a
  // different and much worse failure than a few unnamed nodes.
  if (!isReady(elements)) {
    console.error("\nFAIL: no usable tree — macOS sees no accessible content at all.");
    failed = true;
  } else if (unnamed.length > 0) {
    console.error(`\nFAIL: ${unnamed.length} non-structural element(s) with no accessible name:`);
    for (const el of unnamed.slice(0, 20)) console.error(`  ${el.role} at depth ${el.depth}`);
    failed = true;
  } else {
    console.log("\nevery non-structural element of our own UI has an accessible name.");
  }
} finally {
  preview.kill();
}

// Exiting inside the `try` would skip the `kill` above, and a leaked preview
// poisons the *next* run: the probe finds several instances, activates one and
// walks another. Three orphans turned a green gate into 3469 elements of
// nonsense before this was noticed.
if (failed) process.exit(1);
