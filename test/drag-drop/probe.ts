/**
 * Headless Chromium + WebKit evidence for the drag-drop web substrate.
 *
 * Mouse uses Playwright's real input on both engines and asserts the fixture
 * `onDrop` result. Chromium pre-hold scroll uses CDP
 * Input.synthesizeScrollGesture (dispatchTouchEvent does not pan this
 * headless compositor) and asserts the scroller actually moved. Chromium
 * post-hold drag uses dispatchTouchEvent. WebKit Playwright exposes no
 * hold/move touch protocol, so WebKit touch is synthetic PointerEvents at
 * elementFromPoint — controller hold/tolerance only, not native scroll proof.
 *
 *   bun test/drag-drop/probe.ts --browser=chromium
 *   bun test/drag-drop/probe.ts --browser=webkit
 */

import { chromium, webkit, type Browser, type BrowserType, type CDPSession, type Page } from "playwright";
import { fileURLToPath } from "node:url";

const browserFlag = process.argv.find((arg) => arg.startsWith("--browser="))?.slice("--browser=".length);
const engines: Array<[string, BrowserType]> = ([["chromium", chromium], ["webkit", webkit]] as Array<[string, BrowserType]>)
  .filter(([name]) => !browserFlag || browserFlag === name);

if (engines.length === 0) {
  throw new Error(`Unknown --browser=${browserFlag}`);
}

const fixtureRoot = fileURLToPath(new URL(".", import.meta.url));
const repoRoot = fileURLToPath(new URL("../..", import.meta.url));
const viteBin = fileURLToPath(new URL("../../packages/svelte/preview/node_modules/vite/bin/vite.js", import.meta.url));
const port = 4179;
const url = `http://127.0.0.1:${port}/`;

let failures = 0;

function check(label: string, ok: boolean, detail = ""): void {
  if (!ok) failures += 1;
  console.log(`${ok ? "  ok  " : "  FAIL"}  ${label}${detail ? `  — ${detail}` : ""}`);
}

async function waitForServer(timeoutMs = 30_000): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try {
      const res = await fetch(url, { signal: AbortSignal.timeout(1000) });
      if (res.ok) return;
    } catch {
      await new Promise((resolve) => setTimeout(resolve, 150));
    }
  }
  throw new Error(`drag-drop fixture on :${port} did not start`);
}

const child = Bun.spawn(
  ["bun", viteBin, "--config", `${fixtureRoot}/vite.config.ts`, "--port", String(port), "--strictPort", "--host", "127.0.0.1"],
  {
    cwd: repoRoot,
    stdout: "inherit",
    stderr: "inherit",
  },
);
await waitForServer();

async function frames(page: Page, count = 2): Promise<void> {
  await page.evaluate((n) => {
    return new Promise<void>((resolve) => {
      const tick = (left: number) => {
        if (left <= 0) {
          resolve();
          return;
        }
        requestAnimationFrame(() => tick(left - 1));
      };
      tick(n);
    });
  }, count);
}

async function box(page: Page, selector: string): Promise<{ x: number; y: number; width: number; height: number }> {
  const handle = await page.locator(selector).boundingBox();
  if (!handle) throw new Error(`no box for ${selector}`);
  return handle;
}

function center(rect: { x: number; y: number; width: number; height: number }): { x: number; y: number } {
  return { x: rect.x + rect.width / 2, y: rect.y + rect.height / 2 };
}

async function probeAttr(page: Page, name: string): Promise<string> {
  return (await page.locator("#probe").getAttribute(`data-${name}`)) ?? "";
}

async function waitProbe(page: Page, name: string, expected: string, timeout = 2_000): Promise<string> {
  try {
    await page.waitForFunction(
      ({ name, expected }) => document.querySelector("#probe")?.getAttribute(`data-${name}`) === expected,
      { name, expected },
      { timeout },
    );
  } catch {
    // Fall through and report the actual value.
  }
  return probeAttr(page, name);
}

async function dispatchAtPoint(
  page: Page,
  type: "pointerdown" | "pointermove" | "pointerup",
  x: number,
  y: number,
  pointerType: string,
  pointerId: number,
): Promise<void> {
  await page.evaluate(
    ({ type, x, y, pointerType, pointerId }) => {
      const node = document.elementFromPoint(x, y) ?? document.body;
      node.dispatchEvent(
        new PointerEvent(type, {
          bubbles: true,
          cancelable: true,
          composed: true,
          pointerId,
          pointerType,
          isPrimary: true,
          button: 0,
          buttons: type === "pointerup" ? 0 : 1,
          clientX: x,
          clientY: y,
          view: window,
        }),
      );
    },
    { type, x, y, pointerType, pointerId },
  );
}

async function touchAt(
  page: Page,
  cdp: CDPSession | null,
  type: "touchStart" | "touchMove" | "touchEnd",
  x: number,
  y: number,
): Promise<void> {
  if (cdp) {
    await cdp.send("Input.dispatchTouchEvent", {
      type,
      touchPoints: type === "touchEnd" ? [] : [{ x, y }],
    });
    return;
  }
  const pointerType = "touch";
  const pointerId = 7;
  if (type === "touchStart") await dispatchAtPoint(page, "pointerdown", x, y, pointerType, pointerId);
  else if (type === "touchMove") await dispatchAtPoint(page, "pointermove", x, y, pointerType, pointerId);
  else await dispatchAtPoint(page, "pointerup", x, y, pointerType, pointerId);
}

async function run(page: Page, name: string, cdp: CDPSession | null): Promise<void> {
  await page.goto(url, { waitUntil: "load" });
  await page.locator("#source").waitFor();

  const source = center(await box(page, "#source"));
  const target = center(await box(page, "#target"));

  await page.mouse.move(source.x, source.y);
  await page.mouse.down();
  await page.mouse.move(source.x + 28, source.y, { steps: 8 });
  const phase = await waitProbe(page, "phase", "dragging");
  const captured = await waitProbe(page, "captured", "true");
  check(
    `${name}: real mouse capture after activation`,
    phase === "dragging" && captured === "true",
    `captured=${captured} phase=${phase}`,
  );

  await page.mouse.move(target.x, target.y, { steps: 10 });
  await frames(page);
  const preview = await page.locator(".poodle-drag-preview").count();
  const posture = await probeAttr(page, "posture");
  const hoverTarget = await probeAttr(page, "target");
  check(
    `${name}: captured drag routes over the target, not the source`,
    preview === 1 && posture === "accepted" && hoverTarget === "list",
    `preview=${preview} posture=${posture} target=${hoverTarget}`,
  );

  await page.mouse.up();
  const afterPhase = await waitProbe(page, "phase", "idle");
  const afterPreview = await page.locator(".poodle-drag-preview").count();
  const dropped = await probeAttr(page, "drop");
  check(
    `${name}: mouse drop commits onDrop`,
    afterPhase === "idle" && afterPreview === 0 && dropped === "list:inside:move",
    `phase=${afterPhase} preview=${afterPreview} drop=${dropped}`,
  );

  await page.reload({ waitUntil: "load" });
  await page.locator("#source").waitFor();
  const origin = center(await box(page, "#source"));
  const targetBox = await box(page, "#target");
  const miss = { x: targetBox.x + targetBox.width + 48, y: targetBox.y + Math.min(12, targetBox.height / 2) };
  await page.mouse.move(origin.x, origin.y);
  await page.mouse.down();
  await page.mouse.move(origin.x + 28, origin.y, { steps: 6 });
  await waitProbe(page, "phase", "dragging");
  await page.mouse.move(miss.x, miss.y, { steps: 8 });
  await frames(page);
  const missed = await probeAttr(page, "posture");
  await page.evaluate((width) => {
    const node = document.getElementById("target");
    if (node) node.style.width = `${width}px`;
  }, miss.x - targetBox.x + 40);
  await frames(page, 6);
  const afterResize = await probeAttr(page, "posture");
  const targetCenter = center(await box(page, "#target"));
  await page.mouse.move(targetCenter.x, targetCenter.y, { steps: 6 });
  await frames(page);
  await waitProbe(page, "posture", "accepted");
  await page.evaluate(() => {
    const scroller = document.getElementById("scroller");
    if (!scroller) return;
    scroller.scrollTop += 220;
  });
  await frames(page, 6);
  const afterScroll = await probeAttr(page, "posture");
  await page.mouse.up();
  check(
    `${name}: resize/scroll re-hit-test without invalidateLayout`,
    missed !== "accepted" && afterResize === "accepted" && afterScroll !== "accepted",
    `miss=${missed} resized=${afterResize} scrolled=${afterScroll}`,
  );

  await page.reload({ waitUntil: "load" });
  await page.locator("#source").waitFor();
  const start = center(await box(page, "#source"));
  if (cdp) {
    const beforeScroll = await page.locator("#scroller").evaluate((node) => (node as HTMLElement).scrollTop);
    await touchAt(page, cdp, "touchStart", start.x, start.y);
    await cdp.send("Input.synthesizeScrollGesture", {
      x: start.x,
      y: start.y,
      xDistance: 0,
      yDistance: -160,
    });
    await page.waitForTimeout(350);
    const afterScroll = await page.locator("#scroller").evaluate((node) => (node as HTMLElement).scrollTop);
    const touchPhase = await probeAttr(page, "phase");
    const touchCaptured = await probeAttr(page, "captured");
    await touchAt(page, cdp, "touchEnd", start.x, start.y);
    check(
      `${name}: native scroll before hold moves the scroller and does not activate`,
      touchPhase === "idle" && touchCaptured !== "true" && afterScroll > beforeScroll,
      `phase=${touchPhase} captured=${touchCaptured} scroll=${beforeScroll}->${afterScroll}`,
    );
  } else {
    await touchAt(page, cdp, "touchStart", start.x, start.y);
    await touchAt(page, cdp, "touchMove", start.x, start.y + 40);
    await page.waitForTimeout(350);
    const touchPhase = await probeAttr(page, "phase");
    await touchAt(page, cdp, "touchEnd", start.x, start.y + 40);
    check(
      `${name}: synthetic touch movement beyond tolerance stays idle (WebKit has no native hold/move touch protocol; not scroll proof)`,
      touchPhase === "idle",
      `phase=${touchPhase}`,
    );
  }

  await page.reload({ waitUntil: "load" });
  await page.locator("#source").waitFor();
  const hold = center(await box(page, "#source"));
  const drop = center(await box(page, "#target"));
  await touchAt(page, cdp, "touchStart", hold.x, hold.y);
  await page.waitForTimeout(350);
  const heldPhase = await waitProbe(page, "phase", "dragging", 1_000);
  const steps = 8;
  for (let i = 1; i <= steps; i++) {
    await touchAt(
      page,
      cdp,
      "touchMove",
      hold.x + ((drop.x - hold.x) * i) / steps,
      hold.y + ((drop.y - hold.y) * i) / steps,
    );
  }
  await frames(page);
  const heldPosture = await probeAttr(page, "posture");
  const heldTarget = await probeAttr(page, "target");
  await touchAt(page, cdp, "touchEnd", drop.x, drop.y);
  check(
    cdp
      ? `${name}: native touch hold activates, then routes over the target`
      : `${name}: synthetic touch hold activates, then routes over the target (WebKit has no native hold/move touch protocol)`,
    heldPhase === "dragging" && heldPosture === "accepted" && heldTarget === "list",
    `phase=${heldPhase} posture=${heldPosture} target=${heldTarget}`,
  );

  await page.reload({ waitUntil: "load" });
  await page.locator("#source").waitFor();
  await page.focus("#source");
  await page.keyboard.press("Space");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("Enter");
  const focused = await page.evaluate(() => document.activeElement?.id ?? "");
  const idle = await probeAttr(page, "phase");
  const keyboardDrop = await probeAttr(page, "drop");
  check(
    `${name}: keyboard drop commits and restores focus`,
    focused === "source" && idle === "idle" && keyboardDrop === "list:inside:move",
    `focus=${focused} phase=${idle} drop=${keyboardDrop}`,
  );

  await page.reload({ waitUntil: "load" });
  await page.locator("#nested-source").waitFor();
  await page.locator("#inner-scroll").evaluate((node) => {
    (node as HTMLElement).scrollTop = 80;
  });
  await page.locator("#outer-scroll").evaluate((node) => {
    (node as HTMLElement).scrollTop = 40;
  });
  const nestedStart = center(await box(page, "#nested-source"));
  const innerBox = await box(page, "#inner-scroll");
  const readScroll = async (id: string): Promise<number> =>
    page.locator(id).evaluate((node) => (node as HTMLElement).scrollTop);
  const innerBefore = await readScroll("#inner-scroll");
  const outerBefore = await readScroll("#outer-scroll");
  await page.mouse.move(nestedStart.x, nestedStart.y);
  await page.mouse.down();
  await page.mouse.move(nestedStart.x + 20, nestedStart.y, { steps: 6 });
  const activated = await waitProbe(page, "phase", "dragging");
  const edge = { x: innerBox.x + Math.min(80, innerBox.width / 2), y: innerBox.y + 8 };
  await page.mouse.move(edge.x, edge.y, { steps: 6 });
  await frames(page, 3);
  const innerDuring = await readScroll("#inner-scroll");
  const outerDuring = await readScroll("#outer-scroll");
  await page.locator("#inner-scroll").evaluate((node) => {
    const el = node as HTMLElement;
    el.scrollTop = 0;
  });
  await frames(page, 12);
  const outerAfterInnerStop = await readScroll("#outer-scroll");
  await page.keyboard.press("Escape");
  const cancelledPhase = await waitProbe(page, "phase", "idle");
  const outerAfterCancel = await readScroll("#outer-scroll");
  await frames(page, 8);
  const outerHeld = await readScroll("#outer-scroll");
  check(
    `${name}: nested auto-scroll prefers the inner container, then the outer, and stops on cancel`,
    activated === "dragging" &&
      innerDuring < innerBefore &&
      outerDuring === outerBefore &&
      outerAfterInnerStop < outerBefore &&
      cancelledPhase === "idle" &&
      outerHeld === outerAfterCancel,
    `inner=${innerBefore}->${innerDuring} outer=${outerBefore}->${outerDuring}->${outerAfterInnerStop} cancel=${cancelledPhase} held=${outerHeld}`,
  );
}


// ---------------------------------------------------------------------------
// Cross-window host bridge (g16.026).
//
// Two isolated browser contexts, and a host that is neither of them. The probe
// process holds the transaction the way a real shell would: neither page can
// see the other, neither page can reach the other's controller, and every
// hostile case below is expressed by giving one window something the other
// window did not agree to.
// ---------------------------------------------------------------------------

type HostBridge = {
  state(): { prepares: string[]; starts: string[]; stops: string[]; cancels: string[]; outcomes: string[]; commits: string[]; phase: string };
  probe(): { phase: string; posture: string; target: string; position: string; draggable: string };
  arm(token: string | null): void;
  startNativeDrag(): { prevented: boolean; types: string[]; body: string };
  endNativeDrag(dropEffect: string): void;
  terminal(outcome: unknown): void;
  project(projection: Record<string, unknown> & { token: string }): void;
  left(token: string): void;
  cancelled(token: string, reason: string): void;
  refuse(targetId: string | null): void;
  setCommit(answer: unknown): void;
  dropEnvelope(body: string | null, targetId?: string): void;
  dragOverClaimed(body: string | null, targetId?: string): boolean;
};

declare global {
  interface Window {
    __poodleHost: HostBridge;
  }
}

const CROSS_WINDOW_MIME = "application/x-poodle-cross-window-drag+json";

function envelope(token: string, version = 1): string {
  return JSON.stringify({ protocolVersion: version, token });
}

async function runCrossWindow(name: string, browser: Browser): Promise<void> {
  // Two contexts, not two pages: isolated storage, no shared BroadcastChannel,
  // no window handle between them. If anything here worked by accident through
  // a same-page shortcut, it would stop working now.
  const sending = await browser.newContext({ viewport: { width: 640, height: 480 } });
  const receiving = await browser.newContext({ viewport: { width: 640, height: 480 } });
  const a = await sending.newPage();
  const b = await receiving.newPage();

  try {
    await a.goto(`${url}cross-window.html?role=source`, { waitUntil: "load" });
    await b.goto(`${url}cross-window.html?role=target`, { waitUntil: "load" });
    await a.locator("#source").waitFor();
    await b.locator("#target").waitFor();

    check(
      `${name}: a bridged source does not advertise a native drag before it is armed`,
      (await a.evaluate(() => window.__poodleHost.probe().draggable)) === "false",
    );

    // ── preparation runs before activation ────────────────────────────────
    const source = center(await box(a, "#source"));
    await a.mouse.move(source.x, source.y);
    await a.mouse.down();
    const preparing = await waitProbe(a, "phase", "preparing");
    const preparedCount = (await a.evaluate(() => window.__poodleHost.state().prepares.length));
    check(
      `${name}: host preparation starts on the pre-drag gesture`,
      preparing === "preparing" && preparedCount === 1,
      `phase=${preparing} prepares=${preparedCount}`,
    );

    await a.mouse.move(source.x + 40, source.y, { steps: 6 });
    const unarmed = await a.evaluate(() => window.__poodleHost.startNativeDrag());
    check(
      `${name}: an unarmed source cannot start a native cross-window drag`,
      unarmed.prevented && unarmed.types.length === 0,
      `prevented=${unarmed.prevented} types=${unarmed.types.join(",")}`,
    );

    // ── arming, then the one native start ─────────────────────────────────
    await a.evaluate(() => window.__poodleHost.arm("lease-1"));
    // Armed *or* already dragging: the engine may take the advertisement the
    // instant it appears and start its own drag from the pointer that is
    // already down, which is the behaviour this transport exists to use.
    const armed = await waitProbe(a, "phase", "armed");
    const advertised = await a.evaluate(() => window.__poodleHost.probe().draggable);
    const started = await a.evaluate(() => window.__poodleHost.startNativeDrag());
    const afterStart = await a.evaluate(() => window.__poodleHost.state());
    check(
      `${name}: the armed source writes only the receipt and starts one host subscription`,
      (armed === "armed" || armed === "dragging") &&
        advertised === "true" &&
        !started.prevented &&
        started.types.length === 1 &&
        started.types[0] === CROSS_WINDOW_MIME &&
        started.body === envelope("lease-1") &&
        afterStart.starts.length === 1 &&
        afterStart.starts[0] === "lease-1:data-transfer" &&
        afterStart.phase === "dragging",
      `armed=${armed} draggable=${advertised} prevented=${started.prevented} types=${started.types.join(",")} body=${started.body} starts=${afterStart.starts.join(",")}`,
    );

    // ── the receiving window projects, revalidates, and refuses the rest ──
    await b.evaluate(() => window.__poodleHost.project({ token: "lease-1" }));
    const projected = await b.evaluate(() => window.__poodleHost.probe());
    check(
      `${name}: the receiving window projects the host target through its own gates`,
      projected.phase === "dragging" && projected.posture === "accepted" && projected.target === "list",
      `phase=${projected.phase} posture=${projected.posture} target=${projected.target}`,
    );

    const foreignClaim = await b.evaluate(
      (body) => window.__poodleHost.dragOverClaimed(body as string),
      envelope("someone-elses-lease"),
    );
    const ownClaim = await b.evaluate(
      (body) => window.__poodleHost.dragOverClaimed(body as string),
      envelope("lease-1"),
    );
    const noEnvelopeClaim = await b.evaluate(() => window.__poodleHost.dragOverClaimed(null));
    check(
      `${name}: dragover is claimed by the declared envelope plus a live projection`,
      ownClaim && foreignClaim && !noEnvelopeClaim,
      `own=${ownClaim} foreign=${foreignClaim} none=${noEnvelopeClaim}`,
    );

    await b.evaluate((body) => window.__poodleHost.dropEnvelope(body as string), envelope("someone-elses-lease"));
    const afterMismatch = await b.evaluate(() => window.__poodleHost.state());
    const mismatchProbe = await b.evaluate(() => window.__poodleHost.probe());
    check(
      `${name}: a drop envelope that is not the projected receipt cannot reuse hover acceptance`,
      afterMismatch.commits.length === 0 && mismatchProbe.posture === "",
      `commits=${afterMismatch.commits.join(",")} posture=${mismatchProbe.posture}`,
    );

    // ── host geometry moves with no local pointer input ───────────────────
    await b.evaluate(() => window.__poodleHost.project({ token: "lease-1", targetId: "other" }));
    const moved = await b.evaluate(() => window.__poodleHost.probe());
    await b.evaluate(() => window.__poodleHost.refuse("other"));
    await b.evaluate(() => window.__poodleHost.project({ token: "lease-1", targetId: "other" }));
    const refused = await b.evaluate(() => window.__poodleHost.probe());
    check(
      `${name}: the projected target follows host geometry and a refusal clears it`,
      moved.target === "other" && moved.posture === "accepted" && refused.posture === "rejected",
      `moved=${moved.target}/${moved.posture} refused=${refused.posture}`,
    );

    await b.evaluate((body) => window.__poodleHost.dropEnvelope(body as string, "other"), envelope("lease-1"));
    const afterStaleDrop = await b.evaluate(() => window.__poodleHost.state());
    check(
      `${name}: a stale projection cannot commit`,
      afterStaleDrop.commits.length === 0,
      `commits=${afterStaleDrop.commits.join(",")}`,
    );

    // ── the real drop ─────────────────────────────────────────────────────
    await b.evaluate(() => window.__poodleHost.refuse(null));
    await b.evaluate(() => window.__poodleHost.project({ token: "lease-1" }));
    await b.evaluate((body) => window.__poodleHost.dropEnvelope(body as string), envelope("lease-1"));
    await b.waitForFunction(() => window.__poodleHost.state().commits.length > 0, undefined, { timeout: 2_000 }).catch(() => {});
    const committed = await b.evaluate(() => window.__poodleHost.state());
    check(
      `${name}: the host bridge commits once and no local drop callback runs`,
      committed.commits.length === 1 && committed.commits[0] === "lease-1:list:inside",
      `commits=${committed.commits.join(",")}`,
    );

    // ── the native end is not the result ──────────────────────────────────
    await a.evaluate(() => window.__poodleHost.endNativeDrag("move"));
    const afterNativeEnd = await a.evaluate(() => window.__poodleHost.state());
    check(
      `${name}: a native drag end reporting a move is not a commit`,
      afterNativeEnd.outcomes.length === 0 && afterNativeEnd.phase === "dragging",
      `outcomes=${afterNativeEnd.outcomes.join(",")} phase=${afterNativeEnd.phase}`,
    );

    await a.evaluate(() =>
      window.__poodleHost.terminal({ status: "rejected", reason: "lease expired" }),
    );
    await a.evaluate(() =>
      window.__poodleHost.terminal({ status: "committed", intent: { targetId: "list", position: "inside", operation: "move" } }),
    );
    const afterTerminal = await a.evaluate(() => window.__poodleHost.state());
    check(
      `${name}: the host's refusal is the result, once, and a repeat is inert`,
      afterTerminal.outcomes.length === 1 &&
        afterTerminal.outcomes[0] === "rejected:lease expired" &&
        afterTerminal.cancels.length === 0 &&
        afterTerminal.stops.length === 1 &&
        afterTerminal.phase === "idle",
      `outcomes=${afterTerminal.outcomes.join(",")} cancels=${afterTerminal.cancels.join(",")} stops=${afterTerminal.stops.join(",")}`,
    );
    await a.mouse.up();

    // ── window loss ───────────────────────────────────────────────────────
    await a.mouse.move(source.x, source.y);
    await a.mouse.down();
    await waitProbe(a, "phase", "preparing");
    await a.evaluate(() => window.__poodleHost.arm("lease-2"));
    await waitProbe(a, "phase", "armed");
    await a.evaluate(() => window.__poodleHost.startNativeDrag());
    await b.evaluate(() => window.__poodleHost.project({ token: "lease-2" }));

    // The receiving window goes away mid-drag. It is the host's job to say so,
    // and the sending window must release its lease exactly once when it does.
    await receiving.close();
    await a.evaluate(() =>
      window.__poodleHost.terminal({ status: "cancelled", reason: "window-lost" }),
    );
    await a.evaluate(() =>
      window.__poodleHost.terminal({ status: "cancelled", reason: "window-lost" }),
    );
    const afterLoss = await a.evaluate(() => window.__poodleHost.state());
    check(
      `${name}: losing the receiving window ends the sending session once`,
      afterLoss.phase === "idle" &&
        afterLoss.outcomes.length === 2 &&
        afterLoss.outcomes[1] === "cancelled:window-lost" &&
        afterLoss.cancels.length === 0,
      `phase=${afterLoss.phase} outcomes=${afterLoss.outcomes.join(",")} cancels=${afterLoss.cancels.join(",")}`,
    );
    await a.mouse.up();

    // ── the sending window goes away while a lease is live ────────────────
    await a.mouse.move(source.x, source.y);
    await a.mouse.down();
    await waitProbe(a, "phase", "preparing");
    await a.evaluate(() => window.__poodleHost.arm("lease-3"));
    await waitProbe(a, "phase", "armed");
    await a.evaluate(() => window.__poodleHost.startNativeDrag());
    const held = await a.evaluate(() => window.__poodleHost.state());
    await a.evaluate(() => {
      Object.defineProperty(document, "visibilityState", { value: "hidden", configurable: true });
      document.dispatchEvent(new Event("visibilitychange"));
    });
    const afterHidden = await a.evaluate(() => window.__poodleHost.state());
    check(
      `${name}: a lost sending window returns its live lease to the host once`,
      held.starts.length === 3 &&
        afterHidden.phase === "idle" &&
        afterHidden.cancels.length === 1 &&
        afterHidden.cancels[0] === "lease-3:window-lost" &&
        afterHidden.stops.length === 3,
      `phase=${afterHidden.phase} cancels=${afterHidden.cancels.join(",")} stops=${afterHidden.stops.join(",")}`,
    );
  } finally {
    await sending.close().catch(() => {});
    await receiving.close().catch(() => {});
  }
}

// ---------------------------------------------------------------------------
// External files (g16.027).
//
// The engine's own `DataTransfer`, `DataTransferItem`, and `File`: what a drag
// discloses during `dragover` versus at `drop`, whether the page claimed the
// drop, and whether an export refuses the browser's drag so the host can run
// the operating system's instead.
//
// Not proved here, and deliberately not faked: an operating-system-originated
// file drag into the page, and any destination's consumption of an exported
// file. Playwright cannot originate the first, and no browser API reports the
// second. Both stay manual platform evidence.
// ---------------------------------------------------------------------------

type FileHost = {
  state(): {
    prepares: string[];
    starts: string[];
    stops: string[];
    cancels: string[];
    outcomes: string[];
    drops: string[];
    artifacts: string[];
    phase: string;
  };
  probe(): {
    phase: string;
    posture: string;
    reason: string;
    export: string;
    exportName: string;
    draggable: string;
    offered: string;
    names: string;
  };
  arm(receiptId: string | null, fileCount?: number): void;
  startNativeDrag(): { prevented: boolean; types: string[] };
  endNativeDrag(): void;
  reportExport(terminal: { status: string; reason?: string }): void;
  hoverFiles(files: Array<{ name: string; type: string; bytes: number }>): {
    claimed: boolean;
    types: string[];
    kinds: string[];
  };
  dropFiles(files: Array<{ name: string; type: string; bytes: number }>): void;
  leaveOnce(): void;
  heldBatches(): string[];
  hostPath(): string;
};

declare global {
  interface Window {
    __poodleFiles: FileHost;
  }
}

const WAV = { name: "take-01.wav", type: "audio/wav", bytes: 512 };

async function runExternalFiles(name: string, browser: Browser): Promise<void> {
  const context = await browser.newContext({ viewport: { width: 640, height: 480 } });
  const page = await context.newPage();

  try {
    await page.goto(`${url}files.html`, { waitUntil: "load" });
    await page.locator("#zone").waitFor();

    // ── inbound: hover discloses types, the drop discloses everything ────
    const hover = await page.evaluate(
      (file) => window.__poodleFiles.hoverFiles([file]),
      WAV,
    );
    const hovering = await page.evaluate(() => window.__poodleFiles.probe());
    check(
      `${name}: a real file drag is claimed and hovers on declared types alone`,
      hover.claimed &&
        hover.types.includes("Files") &&
        hover.kinds.every((kind) => kind === "file") &&
        hovering.phase === "dragging" &&
        hovering.posture === "accepted" &&
        hovering.offered === "1" &&
        hovering.names === "?",
      `claimed=${hover.claimed} types=${hover.types.join(",")} kinds=${hover.kinds.join(",")} phase=${hovering.phase} posture=${hovering.posture} offered=${hovering.offered} names=${hovering.names}`,
    );

    await page.evaluate((file) => window.__poodleFiles.dropFiles([file]), WAV);
    const dropped = await page.evaluate(() => window.__poodleFiles.state());
    const afterDrop = await page.evaluate(() => window.__poodleFiles.probe());
    const held = await page.evaluate(() => window.__poodleFiles.heldBatches());
    check(
      `${name}: the drop commits through the ordinary target and releases the batch`,
      dropped.drops.length === 1 &&
        dropped.drops[0] === "take-01.wav:512" &&
        afterDrop.phase === "idle" &&
        held.length === 0,
      `drops=${dropped.drops.join(",")} phase=${afterDrop.phase} held=${held.join(",")}`,
    );

    // ── inbound: the target's own limits, on real engine metadata ────────
    await page.reload({ waitUntil: "load" });
    await page.locator("#zone").waitFor();
    const refusedHover = await page.evaluate(() =>
      window.__poodleFiles.hoverFiles([
        { name: "notes.txt", type: "text/plain", bytes: 8 },
      ]),
    );
    const refused = await page.evaluate(() => window.__poodleFiles.probe());
    check(
      `${name}: a declared type the target refuses is rejected at hover`,
      refusedHover.claimed && refused.posture === "rejected" && refused.reason === "unsupported-type",
      `claimed=${refusedHover.claimed} posture=${refused.posture} reason=${refused.reason}`,
    );

    // A size the platform only discloses at drop is caught there.
    await page.reload({ waitUntil: "load" });
    await page.locator("#zone").waitFor();
    await page.evaluate(() =>
      window.__poodleFiles.hoverFiles([{ name: "big.wav", type: "audio/wav", bytes: 8 }]),
    );
    const acceptedHover = await page.evaluate(() => window.__poodleFiles.probe());
    await page.evaluate(() =>
      window.__poodleFiles.dropFiles([{ name: "big.wav", type: "audio/wav", bytes: 9_000 }]),
    );
    const oversized = await page.evaluate(() => window.__poodleFiles.state());
    check(
      `${name}: an oversized file is refused at drop, not on hover acceptance`,
      acceptedHover.posture === "accepted" && oversized.drops.length === 0 && oversized.phase === "idle",
      `hover=${acceptedHover.posture} drops=${oversized.drops.join(",")} phase=${oversized.phase}`,
    );

    // ── inbound: per-element leaves are not the drag leaving ─────────────
    await page.reload({ waitUntil: "load" });
    await page.locator("#zone").waitFor();
    await page.evaluate((file) => window.__poodleFiles.hoverFiles([file]), WAV);
    await page.evaluate((file) => window.__poodleFiles.hoverFiles([file]), WAV);
    await page.evaluate(() => window.__poodleFiles.leaveOnce());
    const stillLive = await page.evaluate(() => window.__poodleFiles.probe());
    await page.evaluate(() => window.__poodleFiles.leaveOnce());
    await page.evaluate(() => window.__poodleFiles.leaveOnce());
    const left = await page.evaluate(() => window.__poodleFiles.probe());
    check(
      `${name}: a per-element leave does not end the drag; leaving the window does`,
      stillLive.phase === "dragging" && left.phase === "idle",
      `after-one=${stillLive.phase} after-all=${left.phase}`,
    );

    // ── drag-out: the host owns the native gesture ───────────────────────
    await page.reload({ waitUntil: "load" });
    await page.locator("#clip").waitFor();
    const clip = center(await box(page, "#clip"));
    await page.mouse.move(clip.x, clip.y);
    await page.mouse.down();
    const preparing = await waitProbe(page, "export", "preparing");
    const unarmed = await page.evaluate(() => window.__poodleFiles.startNativeDrag());
    const beforeArm = await page.evaluate(() => window.__poodleFiles.state());
    check(
      `${name}: an unarmed export cannot start a native drag`,
      preparing === "preparing" && beforeArm.starts.length === 0 && unarmed.prevented,
      `export=${preparing} starts=${beforeArm.starts.join(",")} prevented=${unarmed.prevented}`,
    );

    await page.evaluate(() => window.__poodleFiles.arm("export-1"));
    await waitProbe(page, "export", "armed");
    const armed = await page.evaluate(() => window.__poodleFiles.probe());
    const started = await page.evaluate(() => window.__poodleFiles.startNativeDrag());
    const afterStart = await page.evaluate(() => window.__poodleFiles.state());
    const dragging = await page.evaluate(() => window.__poodleFiles.probe());
    check(
      `${name}: the armed export advertises, refuses the browser's drag, and starts the host's`,
      armed.draggable === "true" &&
        armed.exportName === "take-01.wav" &&
        started.prevented &&
        started.types.length === 0 &&
        afterStart.starts.length === 1 &&
        afterStart.starts[0] === "export-1" &&
        dragging.export === "dragging",
      `draggable=${armed.draggable} name=${armed.exportName} prevented=${started.prevented} types=${started.types.join(",")} starts=${afterStart.starts.join(",")} export=${dragging.export}`,
    );

    await page.evaluate(() => window.__poodleFiles.endNativeDrag());
    const afterNativeEnd = await page.evaluate(() => window.__poodleFiles.state());
    await page.evaluate(() => window.__poodleFiles.reportExport({ status: "ended" }));
    await page.evaluate(() => window.__poodleFiles.reportExport({ status: "ended" }));
    const ended = await page.evaluate(() => window.__poodleFiles.state());
    const endedProbe = await page.evaluate(() => window.__poodleFiles.probe());
    const markup = await page.evaluate(() => document.body.innerHTML);
    const hostPath = await page.evaluate(() => window.__poodleFiles.hostPath());
    check(
      `${name}: a native end is not a result, and an ending deletes nothing`,
      afterNativeEnd.outcomes.length === 0 &&
        ended.outcomes.length === 1 &&
        ended.cancels.length === 0 &&
        ended.stops.length === 1 &&
        ended.artifacts.length === 1 &&
        endedProbe.export === "ended" &&
        endedProbe.phase === "idle" &&
        !markup.includes(hostPath),
      `native-end-outcomes=${afterNativeEnd.outcomes.length} outcomes=${ended.outcomes.join(",")} cancels=${ended.cancels.join(",")} stops=${ended.stops.join(",")} export=${endedProbe.export} leaked=${markup.includes(hostPath)}`,
    );
    await page.mouse.up();

    // ── drag-out: a receipt beyond the adapter's capabilities ────────────
    await page.reload({ waitUntil: "load" });
    await page.locator("#clip").waitFor();
    const again = center(await box(page, "#clip"));
    await page.mouse.move(again.x, again.y);
    await page.mouse.down();
    await waitProbe(page, "export", "preparing");
    await page.evaluate(() => window.__poodleFiles.arm("export-1", 3));
    const refusedExport = await waitProbe(page, "export", "failed");
    const afterRefusal = await page.evaluate(() => window.__poodleFiles.state());
    const refusedProbe = await page.evaluate(() => window.__poodleFiles.probe());
    check(
      `${name}: a receipt beyond the adapter's capabilities is refused and returned`,
      refusedExport === "failed" &&
        afterRefusal.starts.length === 0 &&
        afterRefusal.cancels.length === 1 &&
        afterRefusal.cancels[0] === "export-1:preparation-failed" &&
        afterRefusal.artifacts.length === 1 &&
        refusedProbe.draggable === "false",
      `export=${refusedExport} starts=${afterRefusal.starts.join(",")} cancels=${afterRefusal.cancels.join(",")} draggable=${refusedProbe.draggable}`,
    );
    await page.mouse.up();
  } finally {
    await context.close().catch(() => {});
  }
}

/**
 * Mounted Svelte and React component evidence (g16.028).
 *
 * The fixture next door proves the substrate through a hand-built surface.
 * This one mounts the three migrated composites in both web frameworks —
 * including two ModelCatalogueEditors that deliberately hold the same model
 * ids under one provider — and asserts the authored callback result.
 */
async function runComponents(name: string, browser: Browser): Promise<void> {
  const context = await browser.newContext({ viewport: { width: 1000, height: 900 } });
  const page = await context.newPage();
  const componentsUrl = `${url}components.html`;

  const attr = async (fw: string, key: string): Promise<string> =>
    (await page.locator(`#${fw}-probe`).getAttribute(`data-${key}`)) ?? "";

  const at = async (selector: string): Promise<{ x: number; y: number }> => {
    const locator = page.locator(selector).first();
    await locator.scrollIntoViewIfNeeded();
    const rect = await locator.boundingBox();
    if (!rect) throw new Error(`no box for ${selector}`);
    return center(rect);
  };

  const press = async (selector: string): Promise<void> => {
    const origin = await at(selector);
    await page.mouse.move(origin.x, origin.y);
    await page.mouse.down();
    // One short move arms the gesture; the substrate needs movement, not a
    // click, before it is a drag at all.
    await page.mouse.move(origin.x + 12, origin.y + 2, { steps: 4 });
  };

  const hover = async (selector: string, yFraction = 0.5): Promise<void> => {
    const locator = page.locator(selector).first();
    await locator.scrollIntoViewIfNeeded();
    const rect = await locator.boundingBox();
    if (!rect) throw new Error(`no box for ${selector}`);
    await page.mouse.move(rect.x + rect.width / 2, rect.y + rect.height * yFraction, { steps: 8 });
    await frames(page);
  };

  try {
    for (const fw of ["svelte", "react"]) {
      await page.goto(componentsUrl, { waitUntil: "load" });
      await page.locator(`#${fw}-probe`).waitFor({ state: "attached" });

      // ── Two instances, the same ids, one provider ──
      await press(`#${fw}-mce-a [data-model-catalogue-id="alpha"] [data-reorder-handle]`);
      await hover(`#${fw}-mce-b [data-model-catalogue-id="gamma"]`);
      const foreignPosture = await page
        .locator(`#${fw}-mce-b [data-drop-target="true"]`)
        .count();
      await page.mouse.up();
      await frames(page);
      check(
        `${name}: ${fw} ModelCatalogueEditor instances cannot cross-drop colliding ids`,
        foreignPosture === 0 &&
          (await attr(fw, "order-b")) === "" &&
          (await attr(fw, "order-a-count")) === "0",
        `posture=${foreignPosture} orderB=${await attr(fw, "order-b")} countA=${await attr(fw, "order-a-count")}`,
      );

      // ── One accepted drop, one complete authored order ──
      await press(`#${fw}-mce-a [data-model-catalogue-id="alpha"] [data-reorder-handle]`);
      await hover(`#${fw}-mce-a [data-model-catalogue-id="gamma"]`);
      await page.mouse.up();
      await frames(page);
      check(
        `${name}: ${fw} ModelCatalogueEditor drop emits one complete shown order`,
        (await attr(fw, "order-a")) === "beta,gamma,alpha" &&
          (await attr(fw, "order-a-count")) === "1",
        `order=${await attr(fw, "order-a")} count=${await attr(fw, "order-a-count")}`,
      );

      // ── A row's own controls still work, and never start a drag ──
      await page.locator(`#${fw}-mce-a [data-model-catalogue-id="beta"] [aria-label="Hide Beta"]`).click();
      await frames(page);
      check(
        `${name}: ${fw} ModelCatalogueEditor row controls survive the reorder sensor`,
        (await attr(fw, "hides")) === "beta:false" && (await attr(fw, "order-a-count")) === "1",
        `hides=${await attr(fw, "hides")} count=${await attr(fw, "order-a-count")}`,
      );

      // ── A target that leaves mid-drag cannot commit ──
      await page.reload({ waitUntil: "load" });
      await page.locator(`#${fw}-probe`).waitFor({ state: "attached" });
      await press(`#${fw}-mce-a [data-model-catalogue-id="alpha"] [data-reorder-handle]`);
      await hover(`#${fw}-mce-a [data-model-catalogue-id="gamma"]`);
      await page.evaluate((framework) => {
        const fixture = (window as unknown as Record<string, { removeCatalogueItem(id: string): void }>)[
          `__${framework}Fixture`
        ];
        fixture.removeCatalogueItem("gamma");
      }, fw);
      await frames(page);
      await page.mouse.up();
      await frames(page);
      check(
        `${name}: ${fw} ModelCatalogueEditor rejects a drop whose target left`,
        (await attr(fw, "order-a-count")) === "0" && (await attr(fw, "order-a")) === "",
        `count=${await attr(fw, "order-a-count")} order=${await attr(fw, "order-a")}`,
      );

      // ── An order replaced mid-drag ends the session, it does not commit ──
      //
      // Reordering the catalogue moves the dragged row's element, which the
      // substrate reads as the source leaving. Ending is the honest outcome:
      // the alternative is committing a placement measured against an order
      // that no longer exists.
      await page.reload({ waitUntil: "load" });
      await page.locator(`#${fw}-probe`).waitFor({ state: "attached" });
      await press(`#${fw}-mce-a [data-model-catalogue-id="alpha"] [data-reorder-handle]`);
      await hover(`#${fw}-mce-a [data-model-catalogue-id="gamma"]`);
      await page.evaluate((framework) => {
        const fixture = (window as unknown as Record<string, { replaceCatalogue(ids: string[]): void }>)[
          `__${framework}Fixture`
        ];
        fixture.replaceCatalogue(["gamma", "beta", "alpha"]);
      }, fw);
      await frames(page);
      await hover(`#${fw}-mce-a [data-model-catalogue-id="beta"]`);
      await page.mouse.up();
      await frames(page);
      check(
        `${name}: ${fw} ModelCatalogueEditor never commits against a replaced order`,
        (await attr(fw, "order-a-count")) === "0" && (await attr(fw, "order-a")) === "",
        `order=${await attr(fw, "order-a")} count=${await attr(fw, "order-a-count")}`,
      );

      // ── A catalogue locked mid-drag refuses the drop ──
      //
      // Eligibility is read from live props at commit, not from the posture
      // the row was registered with.
      await page.reload({ waitUntil: "load" });
      await page.locator(`#${fw}-probe`).waitFor({ state: "attached" });
      await press(`#${fw}-mce-a [data-model-catalogue-id="alpha"] [data-reorder-handle]`);
      await hover(`#${fw}-mce-a [data-model-catalogue-id="gamma"]`);
      await page.evaluate((framework) => {
        const fixture = (window as unknown as Record<string, { lockCatalogue(): void }>)[
          `__${framework}Fixture`
        ];
        fixture.lockCatalogue();
      }, fw);
      await frames(page);
      await page.mouse.up();
      await frames(page);
      check(
        `${name}: ${fw} ModelCatalogueEditor refuses a drop into a locked catalogue`,
        (await attr(fw, "order-a-count")) === "0" && (await attr(fw, "order-a")) === "",
        `order=${await attr(fw, "order-a")} count=${await attr(fw, "order-a-count")}`,
      );

      // ── Escape is a terminal: nothing commits, no posture survives ──
      await press(`#${fw}-mce-a [data-model-catalogue-id="alpha"] [data-reorder-handle]`);
      await hover(`#${fw}-mce-a [data-model-catalogue-id="beta"]`);
      await page.keyboard.press("Escape");
      await frames(page);
      const latched = await page.locator(`#${fw}-mce-a [data-drop-target="true"]`).count();
      await page.mouse.up();
      check(
        `${name}: ${fw} ModelCatalogueEditor cancel commits nothing and clears posture`,
        latched === 0 && (await attr(fw, "order-a-count")) === "0",
        `latched=${latched} count=${await attr(fw, "order-a-count")}`,
      );

      // ── BlockEditor: the grip is the handle ──
      await page.reload({ waitUntil: "load" });
      await page.locator(`#${fw}-probe`).waitFor({ state: "attached" });
      await press(`#${fw}-blocks .poodle-block-editor__block:nth-of-type(1) .poodle-block-editor__drag-grip`);
      await hover(`#${fw}-blocks .poodle-block-editor__block:nth-of-type(3)`, 0.9);
      await page.mouse.up();
      await frames(page);
      check(
        `${name}: ${fw} BlockEditor grip drag emits one complete block order`,
        (await attr(fw, "blocks")) === "b2,b3,b1" && (await attr(fw, "blocks-count")) === "1",
        `blocks=${await attr(fw, "blocks")} count=${await attr(fw, "blocks-count")}`,
      );

      // ── BlockEditor's terminal is announced once, in its own region ──
      const blockRegion = (
        await page.locator(`#${fw}-blocks .poodle-drag-live-region`).textContent()
      )?.trim();
      const blockDragging = await page.locator(`#${fw}-blocks .poodle-dragging`).count();
      const blockDragOver = await page.locator(`#${fw}-blocks .poodle-drag-over`).count();
      check(
        `${name}: ${fw} BlockEditor terminal announces once and strands no posture`,
        blockRegion === "Dropped paragraph block on paragraph block" &&
          blockDragging === 0 &&
          blockDragOver === 0,
        `region="${blockRegion}" dragging=${blockDragging} dragOver=${blockDragOver}`,
      );


      // ── BlockEditor: only the grip is a handle ──
      //
      // The press point is the toolbar's own gap — plain, non-interactive
      // chrome the pointer sensor does not skip on its own. Only the
      // registration's handle constraint keeps it from starting a drag.
      await press(`#${fw}-blocks .poodle-block-editor__block:nth-of-type(1) .poodle-block-editor__toolbar`);
      await hover(`#${fw}-blocks .poodle-block-editor__block:nth-of-type(3)`, 0.9);
      const draggingFromChrome = await page.locator(`#${fw}-blocks .poodle-dragging`).count();
      await page.mouse.up();
      await frames(page);
      check(
        `${name}: ${fw} BlockEditor toolbar chrome is not a drag handle`,
        draggingFromChrome === 0 && (await attr(fw, "blocks-count")) === "1",
        `dragging=${draggingFromChrome} count=${await attr(fw, "blocks-count")}`,
      );

      // ── BlockEditor: the content area stays an editing surface ──
      await press(`#${fw}-blocks .poodle-block-editor__block:nth-of-type(1) textarea`);
      await hover(`#${fw}-blocks .poodle-block-editor__block:nth-of-type(3)`, 0.9);
      const draggingFromContent = await page.locator(`#${fw}-blocks .poodle-dragging`).count();
      await page.mouse.up();
      await frames(page);
      check(
        `${name}: ${fw} BlockEditor content press never starts a drag`,
        draggingFromContent === 0 && (await attr(fw, "blocks-count")) === "1",
        `dragging=${draggingFromContent} count=${await attr(fw, "blocks-count")}`,
      );

      // ── A move control commits once, and focus is not stranded ──
      //
      // Focus does not follow the block: the control is re-rendered at its new
      // position and focus falls to the block group that contains it. It stays
      // inside the block that moved, which is the part the terminal rule is
      // about — nothing is left on `body`. Returning focus to the control
      // itself is a BlockEditor focus decision this card did not take.
      await page.locator(`#${fw}-blocks .poodle-block-editor__block:nth-of-type(2) button[aria-label="Move up"]`).click();
      await frames(page);
      const blockFocusInside = await page.evaluate((framework) => {
        const active = document.activeElement;
        if (!active || active === document.body) return "body";
        const moved = document.querySelector(
          `#${framework}-blocks .poodle-block-editor__block:nth-of-type(1)`,
        );
        return moved && moved.contains(active) ? "inside-moved-block" : "elsewhere";
      }, fw);
      check(
        `${name}: ${fw} BlockEditor move control commits once and strands no focus`,
        (await attr(fw, "blocks")) === "b3,b2,b1" &&
          (await attr(fw, "blocks-count")) === "2" &&
          blockFocusInside === "inside-moved-block",
        `blocks=${await attr(fw, "blocks")} count=${await attr(fw, "blocks-count")} focus=${blockFocusInside}`,
      );

      // ── The move is announced once, in the editor's own region ──
      //
      // The editor joined the harness's provider. Both have a live region, and
      // one drop must not be read out twice.
      await page.reload({ waitUntil: "load" });
      await page.locator(`#${fw}-probe`).waitFor({ state: "attached" });
      await press(`#${fw}-mce-a [data-model-catalogue-id="alpha"] [data-reorder-handle]`);
      await hover(`#${fw}-mce-a [data-model-catalogue-id="gamma"]`);
      await page.mouse.up();
      await frames(page);
      const ownRegion = (
        await page.locator(`#${fw}-mce-a .poodle-model-catalogue-editor__live`).textContent()
      )?.trim();
      const providerRegions = await page.evaluate(
        (framework) =>
          [...document.querySelectorAll(".poodle-drag-live-region")]
            .map((node) => node.textContent?.trim() ?? "")
            .filter((text) => text.length > 0)
            .join("|") + `:${framework}`,
        fw,
      );
      check(
        `${name}: ${fw} ModelCatalogueEditor announces its move once, in its own region`,
        ownRegion === "Moved Alpha to position 3 of 3." &&
          providerRegions === `:${fw}`,
        `own="${ownRegion}" provider="${providerRegions}"`,
      );

      // ── Focus follows the moved model to its new position ──
      const focusedLabel = await page.evaluate(
        () => (document.activeElement as HTMLElement | null)?.getAttribute("aria-label") ?? "",
      );
      check(
        `${name}: ${fw} ModelCatalogueEditor returns focus to the moved model's handle`,
        focusedLabel === "Alpha, position 3 of 3",
        `focused="${focusedLabel}"`,
      );

      // ── The dragged model leaving mid-drag is a terminal, not a commit ──
      await page.reload({ waitUntil: "load" });
      await page.locator(`#${fw}-probe`).waitFor({ state: "attached" });
      await press(`#${fw}-mce-a [data-model-catalogue-id="alpha"] [data-reorder-handle]`);
      await hover(`#${fw}-mce-a [data-model-catalogue-id="gamma"]`);
      await page.evaluate((framework) => {
        const fixture = (window as unknown as Record<string, { removeCatalogueItem(id: string): void }>)[
          `__${framework}Fixture`
        ];
        fixture.removeCatalogueItem("alpha");
      }, fw);
      await frames(page);
      await page.mouse.up();
      await frames(page);
      const strandedSource = await page.locator(`#${fw}-mce-a [data-poodle-drag-source]`).count();
      const strandedTarget = await page.locator(`#${fw}-mce-a [data-drop-target="true"]`).count();
      check(
        `${name}: ${fw} ModelCatalogueEditor source unmount ends the session cleanly`,
        (await attr(fw, "order-a-count")) === "0" &&
          strandedSource === 0 &&
          strandedTarget === 0,
        `count=${await attr(fw, "order-a-count")} source=${strandedSource} target=${strandedTarget}`,
      );

      // ── EditableList: a keyboard pickup drops through the same session ──
      //
      // The web keyboard sensor owns Space/arrows here, so this is the
      // "successful drop after keyboard pickup" terminal.
      await page.reload({ waitUntil: "load" });
      await page.locator(`#${fw}-probe`).waitFor({ state: "attached" });
      await page.locator(`#${fw}-list [data-reorder-index="0"]`).focus();
      await page.keyboard.press("Space");
      await page.keyboard.press("ArrowDown");
      await page.keyboard.press("Space");
      await frames(page);
      const listGrabbed = await page.locator(`#${fw}-list .poodle-editable-list__item--grabbed`).count();
      check(
        `${name}: ${fw} EditableList keyboard pickup commits and leaves no posture`,
        (await attr(fw, "rows")) === "r2,r1,r3" &&
          (await attr(fw, "rows-count")) === "1" &&
          listGrabbed === 0,
        `rows=${await attr(fw, "rows")} count=${await attr(fw, "rows-count")} grabbed=${listGrabbed}`,
      );

      // ── A disabled model cannot be picked up, and is still a place to
      //    put one ──
      await page.reload({ waitUntil: "load" });
      await page.locator(`#${fw}-probe`).waitFor({ state: "attached" });
      await press(`#${fw}-mce-c [data-model-catalogue-id="beta"] [data-reorder-handle]`);
      await hover(`#${fw}-mce-c [data-model-catalogue-id="gamma"]`);
      const disabledPickup = await page.locator(`#${fw}-mce-c [data-poodle-drag-source]`).count();
      await page.mouse.up();
      await frames(page);
      const afterDisabledPickup = await attr(fw, "order-c-count");

      await press(`#${fw}-mce-c [data-model-catalogue-id="alpha"] [data-reorder-handle]`);
      await hover(`#${fw}-mce-c [data-model-catalogue-id="beta"]`);
      await page.mouse.up();
      await frames(page);
      check(
        `${name}: ${fw} ModelCatalogueEditor disabled model is unpickable but droppable-onto`,
        disabledPickup === 0 &&
          afterDisabledPickup === "0" &&
          (await attr(fw, "order-c")) === "beta,alpha,gamma" &&
          (await attr(fw, "order-c-count")) === "1",
        `pickup=${disabledPickup} order=${await attr(fw, "order-c")} count=${await attr(fw, "order-c-count")}`,
      );

      // ── `isDragEnabled=false` is pointer drag only ──
      //
      // Nothing is registered, so no gesture arms; the keyboard grab and the
      // move buttons are untouched, which is what the contract promises.
      await page.reload({ waitUntil: "load" });
      await page.locator(`#${fw}-probe`).waitFor({ state: "attached" });
      await page.evaluate((framework) => {
        const fixture = (window as unknown as Record<string, { disableDragC(): void }>)[
          `__${framework}Fixture`
        ];
        fixture.disableDragC();
      }, fw);
      await frames(page);
      await press(`#${fw}-mce-c [data-model-catalogue-id="alpha"] [data-reorder-handle]`);
      await hover(`#${fw}-mce-c [data-model-catalogue-id="gamma"]`);
      const inertSources = await page.locator(`#${fw}-mce-c [data-poodle-drag-source]`).count();
      await page.mouse.up();
      await frames(page);
      const afterInertDrag = await attr(fw, "order-c-count");

      await page.locator(`#${fw}-mce-c [data-model-catalogue-id="alpha"] [data-reorder-handle]`).focus();
      await page.keyboard.press("Space");
      await page.keyboard.press("ArrowDown");
      await frames(page);
      check(
        `${name}: ${fw} ModelCatalogueEditor isDragEnabled=false stops pointer drag only`,
        inertSources === 0 &&
          afterInertDrag === "0" &&
          (await attr(fw, "order-c")) === "beta,alpha,gamma" &&
          (await attr(fw, "order-c-count")) === "1",
        `sources=${inertSources} afterDrag=${afterInertDrag} order=${await attr(fw, "order-c")} count=${await attr(fw, "order-c-count")}`,
      );

      // ── OrderBy: pointer and Alt+Arrow reach one commit path ──
      await page.locator(`#${fw}-order .poodle-order-by__trigger`).click();
      await page.locator(".poodle-order-by__item").first().waitFor();
      await press(".poodle-order-by__item:nth-of-type(1) .poodle-order-by__drag-handle");
      await hover(".poodle-order-by__item:nth-of-type(3)", 0.9);
      await page.mouse.up();
      await frames(page);
      check(
        `${name}: ${fw} OrderBy reorders inside an ambient provider it cannot join`,
        (await attr(fw, "sort")) === "updated,size,title" && (await attr(fw, "sort-count")) === "1",
        `sort=${await attr(fw, "sort")} count=${await attr(fw, "sort-count")}`,
      );

      // ── OrderBy's terminal is announced once, in the panel's own region ──
      const orderRegion = (
        await page.locator(".poodle-order-by__surface .poodle-drag-live-region").textContent()
      )?.trim();
      const orderDragging = await page.locator(".poodle-order-by__item--dragging").count();
      const orderDropTarget = await page.locator(".poodle-order-by__item--drop-target").count();
      check(
        `${name}: ${fw} OrderBy terminal announces once and strands no posture`,
        orderRegion === "Dropped Title on Size" &&
          orderDragging === 0 &&
          orderDropTarget === 0,
        `region="${orderRegion}" dragging=${orderDragging} dropTarget=${orderDropTarget}`,
      );

      await page.locator(".poodle-order-by__item:nth-of-type(2) .poodle-order-by__drag-handle").focus();
      await page.keyboard.press("Alt+ArrowUp");
      await frames(page);
      const orderFocus = await page.evaluate(
        () => (document.activeElement as HTMLElement | null)?.getAttribute("aria-label") ?? "",
      );
      check(
        `${name}: ${fw} OrderBy Alt+Arrow commits through the same session`,
        (await attr(fw, "sort")) === "size,updated,title" && (await attr(fw, "sort-count")) === "2",
        `sort=${await attr(fw, "sort")} count=${await attr(fw, "sort-count")}`,
      );
      check(
        `${name}: ${fw} OrderBy keeps focus on the handle it reordered from`,
        orderFocus === "Reorder Size. Drag or use Alt plus arrow keys.",
        `focused="${orderFocus}"`,
      );
    }
  } finally {
    await context.close().catch(() => {});
  }
}

for (const [name, type] of engines) {
  console.log(`\n=== ${name} ===`);
  const browser = await type.launch();
  const context = await browser.newContext({
    viewport: { width: 800, height: 600 },
    hasTouch: true,
  });
  const page = await context.newPage();
  let cdp: CDPSession | null = null;
  try {
    if (name === "chromium") cdp = await context.newCDPSession(page);
    await run(page, name, cdp);
    await runCrossWindow(name, browser);
    await runExternalFiles(name, browser);
    await runComponents(name, browser);
  } catch (error) {
    failures += 1;
    console.log(`  FAIL  ${name}: ${error instanceof Error ? error.message : String(error)}`);
  }
  await browser.close();
}

child.kill();
await child.exited;

if (failures > 0) {
  console.error(`\n${failures} drag-drop browser check(s) failed`);
  process.exit(1);
}

console.log("\nall drag-drop browser checks passed");
