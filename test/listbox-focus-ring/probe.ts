import { chromium, type Page } from "playwright";

import { startPreviews, type PreviewServers } from "../visual/server";

type Point = { edge: string; x: number; y: number };
type Pixel = { edge: string; rgba: number[] };

let failures = 0;

function check(label: string, ok: boolean, detail = ""): void {
  if (!ok) failures += 1;
  console.log(`${ok ? "  ok  " : "  FAIL"}  ${label}${detail ? ` — ${detail}` : ""}`);
}

async function screenshotPixels(page: Page, points: Point[]): Promise<Pixel[]> {
  const screenshot = await page.screenshot({ timeout: 15_000 });
  return page.evaluate(
    async ({ base64, samples }) => {
      const image = new Image();
      image.src = `data:image/png;base64,${base64}`;
      await image.decode();
      const canvas = document.createElement("canvas");
      canvas.width = image.naturalWidth;
      canvas.height = image.naturalHeight;
      const context = canvas.getContext("2d");
      if (context === null) throw new Error("could not read the browser screenshot");
      context.drawImage(image, 0, 0);
      const dpr = window.devicePixelRatio;
      return samples.map(({ edge, x, y }) => ({
        edge,
        rgba: [...context.getImageData(Math.round(x * dpr), Math.round(y * dpr), 1, 1).data],
      }));
    },
    { base64: screenshot.toString("base64"), samples: points },
  );
}

function rgbChannels(color: string): number[] {
  const channels = color.match(/[\d.]+/g)?.slice(0, 3).map(Number);
  if (!channels || channels.length !== 3) throw new Error(`could not parse focus-ring color: ${color}`);
  return channels;
}

async function run(servers: PreviewServers): Promise<void> {
  const browser = await chromium.launch({ headless: true, timeout: 30_000 });
  try {
    const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
    await page.goto(
      `${servers.urls.svelte}/?theme=eclipse&density=comfortable&controlSize=md#components/listbox`,
      { waitUntil: "domcontentloaded", timeout: 60_000 },
    );

    const cardList = page.getByRole("listbox", { name: "Library cards" });
    await cardList.waitFor({ state: "visible", timeout: 60_000 });
    const options = cardList.getByRole("option");
    await options.nth(0).focus();
    await page.keyboard.press("ArrowDown");
    await page.waitForFunction(() => {
      const option = document.querySelector<HTMLElement>(
        '[role="listbox"][aria-label="Library cards"] [role="option"][aria-label="Blue Library"]',
      );
      return option?.matches(":focus-visible") === true && option.dataset.focused === "true";
    }, undefined, { timeout: 10_000 });

    const middle = await options.nth(1).evaluate((option) => {
      const rect = option.getBoundingClientRect();
      const points = [
        { edge: "top", x: rect.left + rect.width / 2, y: rect.top + 2 },
        { edge: "right", x: rect.right - 2, y: rect.top + rect.height / 2 },
        { edge: "bottom", x: rect.left + rect.width / 2, y: rect.bottom - 2 },
        { edge: "left", x: rect.left + 2, y: rect.top + rect.height / 2 },
      ];
      return {
        focused: document.activeElement === option && option.matches(":focus-visible"),
        points: points.map(({ edge, x, y }) => {
          const target = document.elementFromPoint(x, y);
          return {
            edge,
            covered: target !== null && (target === option || option.contains(target)),
            target: target?.tagName.toLowerCase() ?? "none",
          };
        }),
      };
    });
    check("ArrowDown focuses the middle card option by keyboard", middle.focused);
    for (const point of middle.points) {
      check(`${point.edge} edge of the focused card ring remains hit-testable`, point.covered, point.target);
    }

    // Add an empty root inside the real specimen frame. This exercises the
    // same component stylesheet and clipping ancestors without changing the
    // public specimen or production markup.
    await page.evaluate(() => {
      const cards = document.querySelector<HTMLElement>(
        '[role="listbox"][aria-label="Library cards"]',
      );
      const frame = cards?.closest<HTMLElement>(".poodle-component-page__section");
      if (frame === null || frame === undefined) throw new Error("Listbox specimen frame is missing");

      const tabTarget = document.createElement("button");
      tabTarget.type = "button";
      tabTarget.setAttribute("aria-label", "Empty listbox focus probe entry");
      tabTarget.style.cssText = "position:fixed;left:-10000px;top:0;width:1px;height:1px";

      const empty = document.createElement("div");
      empty.className = "poodle-listbox";
      empty.setAttribute("data-root-focus-probe", "");
      empty.setAttribute("role", "listbox");
      empty.setAttribute("aria-label", "Empty listbox focus probe");
      empty.tabIndex = 0;
      empty.style.cssText = "width:160px;height:40px;margin:24px 0 24px 12px";
      frame.append(tabTarget, empty);
      tabTarget.focus();
    });
    await page.keyboard.press("Tab");
    await page.waitForFunction(() => {
      const root = document.querySelector<HTMLElement>("[data-root-focus-probe]");
      return root?.matches(":focus-visible") === true && document.activeElement === root;
    }, undefined, { timeout: 10_000 });

    const rootRing = await page.evaluate(() => {
      const root = document.querySelector<HTMLElement>("[data-root-focus-probe]");
      const frame = root?.closest<HTMLElement>(".poodle-component-page__section");
      if (root === null || root === undefined || frame === null || frame === undefined) {
        throw new Error("empty Listbox root or specimen frame is missing");
      }
      const rect = root.getBoundingClientRect();
      const frameRect = frame.getBoundingClientRect();
      const style = getComputedStyle(root);
      const width = Number.parseFloat(style.outlineWidth);
      const offset = Number.parseFloat(style.outlineOffset);
      const points = [
        { edge: "top", x: rect.left + rect.width / 2, y: rect.top - offset - width / 2 },
        { edge: "right", x: rect.right + offset + width / 2, y: rect.top + rect.height / 2 },
        { edge: "bottom", x: rect.left + rect.width / 2, y: rect.bottom + offset + width / 2 },
        { edge: "left", x: rect.left - offset - width / 2, y: rect.top + rect.height / 2 },
      ];
      return {
        focused: document.activeElement === root && root.matches(":focus-visible"),
        color: style.outlineColor,
        frameContainsRing: points.every(
          ({ x, y }) => x >= frameRect.left && x <= frameRect.right && y >= frameRect.top && y <= frameRect.bottom,
        ),
        points,
      };
    });
    check("empty root receives its keyboard focus ring", rootRing.focused);
    check("empty root outset fits inside the Listbox specimen frame", rootRing.frameContainsRing);

    const pixels = await screenshotPixels(page, rootRing.points);
    const expected = rgbChannels(rootRing.color);
    for (const pixel of pixels) {
      const visible = expected.every((channel, index) => Math.abs(pixel.rgba[index]! - channel) <= 28);
      check(`${pixel.edge} edge of the empty root ring is visibly painted`, visible, `pixel ${pixel.rgba.slice(0, 3).join(",")}`);
    }
  } finally {
    await browser.close();
  }
}

let previews: PreviewServers | undefined;
let deadline: ReturnType<typeof setTimeout> | undefined;
try {
  const timedOut = new Promise<never>((_, reject) => {
    deadline = setTimeout(() => reject(new Error("Listbox focus-ring probe exceeded 180 seconds")), 180_000);
  });
  await Promise.race([
    (async () => {
      previews = await startPreviews();
      await run(previews);
    })(),
    timedOut,
  ]);
} catch (error) {
  failures += 1;
  console.error(error);
} finally {
  if (deadline !== undefined) clearTimeout(deadline);
  await previews?.stop();
}

if (failures > 0) {
  console.error(`Listbox focus-ring probe failed ${failures} check(s).`);
  process.exitCode = 1;
} else {
  console.log("Listbox focus-ring browser checks passed.");
}
