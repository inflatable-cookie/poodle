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
