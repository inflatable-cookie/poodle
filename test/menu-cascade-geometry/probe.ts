/** Real rendered submenu acceptance. Run through test:menu-cascade-geometry.
 * --baseline records the original bottom-edge overflow without requiring the fix.
 * Captures and viewport-coordinate receipts go to a fresh run-scratch directory.
 */
import { chromium, type Page, type Locator } from "playwright";
import { fileURLToPath } from "node:url";
import { mkdtemp, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

const root = fileURLToPath(new URL("../..", import.meta.url));
const fixture = fileURLToPath(new URL(".", import.meta.url));
const output = await mkdtemp(join(tmpdir(), "menu-cascade-"));
const head = (await Bun.$`git rev-parse HEAD`.cwd(root).text()).trim();
const baseline = process.argv.includes("--baseline");
const port = 4198;
const origin = `http://127.0.0.1:${port}`;
const child = Bun.spawn(["bun", `${root}/packages/svelte/preview/node_modules/vite/bin/vite.js`, "--config", `${fixture}/vite.config.ts`, "--port", String(port), "--strictPort", "--host", "127.0.0.1"], { cwd: root, stdout: "inherit", stderr: "inherit" });
const receipts: unknown[] = [];
let failures = 0;
function check(label: string, ok: boolean, details?: unknown) {
  console.log(`${ok ? "ok" : "FAIL"} ${label} ${details ? JSON.stringify(details) : ""}`);
  if (!ok) failures++;
}
async function settle(page: Page) { await page.waitForTimeout(160); }
async function capture(page: Page, menu: Locator, name: string) {
  const bounds = await menu.boundingBox();
  const viewport = page.viewportSize()!;
  const extra = await menu.evaluate((node) => ({ scrollTop: node.scrollTop, scrollHeight: node.scrollHeight, clientHeight: node.clientHeight, placement: node.getAttribute("data-placement"), hidden: node.getAttribute("data-anchor-hidden") }));
  const receipt = { head, name, viewport, bounds, ...extra };
  receipts.push(receipt);
  await page.screenshot({ path: join(output, `${name}.png`) });
  console.log(`capture ${JSON.stringify(receipt)}`);
  return bounds;
}
function fits(box: Awaited<ReturnType<Locator["boundingBox"]>>, page: Page) {
  const vp = page.viewportSize()!;
  return !!box && box.x >= 7 && box.y >= 7 && box.x + box.width <= vp.width - 7 && box.y + box.height <= vp.height - 7;
}
async function open(page: Page, fw: string, right = false, tall = false) {
  await page.goto(`${origin}/?framework=${fw}${right ? "&right" : ""}${tall ? "&tall" : ""}`, { waitUntil: "networkidle" });
  await page.getByRole("button", { name: `Options ${fw}`, exact: true }).click();
  const parent = page.getByRole("menuitem", { name: "Go to frame", exact: true });
  await parent.focus();
  await page.keyboard.press("ArrowRight");
  const menu = page.getByRole("menu", { name: "Go to frame", exact: true });
  await menu.waitFor({ timeout: 5000 });
  await settle(page);
  return { menu, parent };
}
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 720, height: 420 } });
page.setDefaultTimeout(5000);
try {
  const deadline = Date.now() + 30000;
  while (true) {
    try { if ((await fetch(origin)).ok) break; } catch {}
    if (Date.now() > deadline) throw new Error("Menu fixture failed to start");
    await Bun.sleep(150);
  }
  if (baseline) {
    const { menu } = await open(page, "svelte");
    const b = await capture(page, menu, "baseline-bottom-overflow");
    check("original flyout exceeds viewport bottom", !!b && b.y + b.height > 420, b);
  } else for (const fw of ["svelte", "react"]) {
    for (const right of [false, true]) {
      const { menu, parent } = await open(page, fw, right);
      const b = await capture(page, menu, `${fw}-${right ? "flipped-right-edge" : "bottom-edge"}`);
      check(`${fw} viewport fit ${right ? "right" : "left"}`, fits(b, page));
      const p = await parent.boundingBox();
      check(`${fw} horizontal row anchor`, !!b && !!p && (right ? Math.abs(b.x + b.width - p.x) < 6 : Math.abs(b.x - p.x - p.width) < 6), { b, p });
      await page.keyboard.press("ArrowRight");
      const deep = page.getByRole("menu", { name: "More frames", exact: true });
      await deep.waitFor();
      await settle(page);
      check(`${fw} depth two fits`, fits(await deep.boundingBox(), page));
      await page.keyboard.press("ArrowLeft");
      check(`${fw} ArrowLeft returns to parent`, await page.getByRole("menuitem", { name: "More frames", exact: true }).evaluate(n => n === document.activeElement));
      await page.keyboard.press("End");
      check(`${fw} End focuses last radio`, await page.getByRole("menuitemradio", { name: "Frame 8", exact: true }).evaluate(n => n === document.activeElement));
      await page.keyboard.press("Home");
      await page.keyboard.press("ArrowUp");
      check(`${fw} arrows wrap enabled items`, await page.getByRole("menuitemradio", { name: "Frame 8", exact: true }).evaluate(n => n === document.activeElement));
      // A real pointer press inside the nested surface must survive the root's
      // outside-dismiss handler, then activate the leaf through the root machine.
      await page.getByRole("menuitemradio", { name: "Frame 8", exact: true }).click();
      await settle(page);
      check(`${fw} nested selection emits leaf and closes root`, await page.locator("body").getAttribute("data-action") === "frame-7" && await page.getByRole("menu").count() === 0);
    }
    const { menu } = await open(page, fw, false, true);
    await page.keyboard.press("End");
    await settle(page);
    const tall = await capture(page, menu, `${fw}-tall-scrolled-list`);
    check(`${fw} tall flyout fits and scrolls`, fits(tall, page) && await menu.evaluate(n => n.scrollTop > 0 && n.scrollHeight > n.clientHeight));
    const last = page.getByRole("menuitemradio", { name: "Frame 40", exact: true });
    const lastBox = await capture(page, last, `${fw}-keyboard-focused-last-item`);
    check(`${fw} focused last item visible`, fits(lastBox, page) && await last.evaluate(n => n === document.activeElement));
    await page.setViewportSize({ width: 620, height: 340 });
    await settle(page);
    check(`${fw} resize refits flyout`, fits(await menu.boundingBox(), page));
    await page.keyboard.press("Home");
    const parent = page.getByRole("menuitem", { name: "Go to frame", exact: true });
    const before = await parent.boundingBox();
    await page.locator(`[data-framework="${fw}"] .scroller`).evaluate(n => n.scrollTop += 12);
    await settle(page);
    const after = await parent.boundingBox();
    const moved = await menu.boundingBox();
    check(`${fw} parent scroll tracks row`, !!before && !!after && before.y !== after.y && fits(moved, page));
    const geometry = await page.evaluate(() => (window as typeof window & { geometry: Array<{ type: string; surface?: { rect: { top: number; left: number }; visible: boolean } }> }).geometry);
    check(`${fw} geometry reports moved surface`, geometry.filter(e => e.type === "upsert" && e.surface?.visible).length >= 3);
    await page.mouse.click(600, 20);
    await settle(page);
    check(`${fw} outside dismisses all surfaces`, await page.getByRole("menu").count() === 0);
    check(`${fw} geometry teardown removes surfaces`, await page.evaluate(() => (window as any).geometry.some((e: any) => e.type === "remove")));
    await open(page, fw);
    await page.keyboard.press("ArrowLeft");
    check(`${fw} ArrowLeft restores root row focus`, await page.getByRole("menuitem", { name: "Go to frame", exact: true }).evaluate(n => n === document.activeElement));
    await page.getByRole("menuitem", { name: "Action", exact: true }).hover();
    await page.getByRole("menuitem", { name: "Go to frame", exact: true }).hover();
    await page.getByRole("menu", { name: "Go to frame", exact: true }).waitFor();
    await page.getByRole("menuitemradio", { name: "Frame 1", exact: true }).hover();
    check(`${fw} pointer path preserves submenu`, await page.getByRole("menu", { name: "Go to frame", exact: true }).isVisible());
    await page.keyboard.press("Escape");
    await settle(page);
    check(`${fw} Escape dismisses root`, await page.getByRole("menu").count() === 0);
    await page.setViewportSize({ width: 720, height: 420 });
  }
} finally {
  await writeFile(join(output, "receipt.json"), JSON.stringify({ head, baseline, receipts, failures }, null, 2));
  console.log(`Menu geometry evidence: ${output}`);
  await browser.close();
  child.kill();
  await child.exited;
}
if (failures) process.exit(1);
