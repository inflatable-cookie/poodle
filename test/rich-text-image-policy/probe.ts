import { chromium, webkit, type Browser, type BrowserType, type Page } from "playwright";

import { specimenUrl } from "../visual/capture";
import { SERVERS, SMOKE_AXES, type Framework } from "../visual/config";
import { startPreviews } from "../visual/server";

/**
 * Headless Chromium + WebKit proof for the RichTextEditor Image Policy
 * specimen (g18.014):
 *
 *   effigy test:rich-text-image-policy
 *
 * jsdom cannot prove the two things this task exists for: that the seeded and
 * inserted fixtures actually decode and paint (non-zero `naturalWidth` and a
 * non-zero box), and that the host-owned async request inserts exactly one
 * image at the selection retained when Insert image was pressed even though
 * the caret moves before the picker resolves. Both real previews are driven
 * through their public `#components/rich-text-editor` routes, and every
 * network request is recorded so the offline fixture claim is checked rather
 * than assumed.
 */

let failures = 0;

function check(label: string, ok: boolean, detail = ""): void {
  if (ok) {
    console.log(`  ok   ${label}`);
  } else {
    failures += 1;
    console.error(`  FAIL ${label}${detail ? ` — ${detail}` : ""}`);
  }
}

interface Geometry {
  complete: boolean;
  naturalWidth: number;
  naturalHeight: number;
  width: number;
  height: number;
  alt: string | null;
  src: string | null;
}

async function readImages(page: Page): Promise<Geometry[]> {
  return page.evaluate(() => {
    const frame = document.querySelector("[data-part='image-policy-editor']");
    if (!frame) return [];
    return [...frame.querySelectorAll("img")].map((image) => {
      const rect = image.getBoundingClientRect();
      return {
        complete: image.complete,
        naturalWidth: image.naturalWidth,
        naturalHeight: image.naturalHeight,
        width: Math.round(rect.width),
        height: Math.round(rect.height),
        alt: image.getAttribute("alt"),
        src: image.getAttribute("src"),
      };
    });
  });
}

function describe(image: Geometry): string {
  return `alt=${JSON.stringify(image.alt)} natural=${image.naturalWidth}x${image.naturalHeight} box=${image.width}x${image.height}`;
}

function loaded(image: Geometry): boolean {
  return image.complete && image.naturalWidth > 0 && image.naturalHeight > 0 && image.width > 0 && image.height > 0;
}

function metric(page: Page, part: string): Promise<string | null> {
  return page.evaluate(
    (selector) => document.querySelector(selector)?.getAttribute("data-count") ?? null,
    `[data-part='${part}']`,
  );
}

async function hostDocument(page: Page): Promise<{ content?: Array<Record<string, unknown>> }> {
  const text = await page.evaluate(
    () => document.querySelector("[data-part='image-host-document']")?.textContent ?? "",
  );
  return JSON.parse(text) as { content?: Array<Record<string, unknown>> };
}

function isExternal(url: string, base: string): boolean {
  if (url.startsWith(base) || url.startsWith("data:") || url.startsWith("blob:")) return false;
  try {
    const parsed = new URL(url);
    return !["127.0.0.1", "localhost", "[::1]"].includes(parsed.hostname);
  } catch {
    return false;
  }
}

async function runSpecimen(page: Page, base: string, framework: Framework): Promise<void> {
  console.log(`\n${framework}`);
  const requests: string[] = [];
  const imageRequests: string[] = [];
  page.on("request", (request) => {
    requests.push(request.url());
    if (request.resourceType() === "image") imageRequests.push(request.url());
  });

  await page.goto(specimenUrl(base, "rich-text-editor", SMOKE_AXES[0]), { waitUntil: "load" });
  const frame = page.locator("[data-part='image-policy-editor']");
  const toggle = page.locator("[data-part='images-toggle']");
  await toggle.waitFor({ state: "visible", timeout: 60_000 });
  await frame.locator(".ProseMirror").waitFor({ state: "visible", timeout: 60_000 });

  // Images off: no image content, no insertion command, standard content still
  // mounts, and the image-enabled host document is retained rather than bled.
  check(`${framework}: images off starts unpressed`, (await toggle.getAttribute("aria-pressed")) === "false");
  check(`${framework}: images off labels the toggle`, (await toggle.textContent())?.trim() === "Images off");
  check(`${framework}: images off renders no image`, (await readImages(page)).length === 0);
  check(
    `${framework}: images off renders no Insert image command`,
    (await frame.locator("[data-command='insert-image'] button").count()) === 0,
  );
  check(
    `${framework}: images off mounts the ordinary standard document`,
    (await frame.locator("h1").first().textContent())?.trim() === "Release notes",
  );
  check(`${framework}: images off keeps the host image document`, (await metric(page, "image-count")) === "1");
  check(`${framework}: images off makes zero host requests`, (await metric(page, "image-request-count")) === "0");
  check(`${framework}: images off makes zero host changes`, (await metric(page, "image-change-count")) === "0");

  // Images on: the seeded fixture must decode and paint, not merely exist.
  await toggle.click();
  await frame.locator("img").first().waitFor({ state: "visible", timeout: 30_000 });
  const seeded = await readImages(page);
  check(`${framework}: exactly one seeded image is mounted`, seeded.length === 1, `${seeded.length}`);
  check(`${framework}: seeded fixture decodes`, seeded.length === 1 && loaded(seeded[0]!), describe(seeded[0]!));
  check(
    `${framework}: seeded fixture is the 96x48 offline raster`,
    seeded.length === 1 && seeded[0]!.naturalWidth === 96 && seeded[0]!.naturalHeight === 48,
    describe(seeded[0]!),
  );
  check(
    `${framework}: seeded fixture matches the host document`,
    seeded.length === 1 && seeded[0]!.alt === "Revenue chart",
    describe(seeded[0]!),
  );
  const insert = frame.locator("[data-command='insert-image'] button");
  check(`${framework}: images on admits the Insert image command`, (await insert.count()) === 1);

  // Plant a real caret at the end of the last paragraph, press Insert image,
  // observe the pending request, then move the caret away before the host
  // resolves. The insertion must still land where the caret was.
  const paragraphs = frame.locator(".ProseMirror p");
  const paragraphCount = await paragraphs.count();
  await paragraphs.nth(paragraphCount - 1).click();
  await page.keyboard.press("End");
  const planted = await hostDocument(page);
  const targetIndex = planted.content!.length - 1;
  const targetText = ((planted.content![targetIndex]!.content as Array<{ text?: string }> | undefined) ?? [])
    .map((node) => node.text ?? "")
    .join("");

  await insert.click();
  const pending = await page
    .waitForFunction(
      () => document.querySelector("[data-command='insert-image'] button")?.hasAttribute("disabled") === true,
      undefined,
      { timeout: 5_000, polling: 16 },
    )
    .then(() => true)
    .catch(() => false);
  check(`${framework}: an in-flight request disables Insert image`, pending);

  // Move the caret to the first paragraph while the request is still pending.
  await paragraphs.first().click();
  await page
    .waitForFunction(
      () => document.querySelectorAll("[data-part='image-policy-editor'] img").length === 2,
      undefined,
      { timeout: 15_000 },
    )
    .catch(() => {});

  const inserted = await readImages(page);
  check(`${framework}: one click inserts exactly one more image`, inserted.length === 2, `${inserted.length}`);
  const picked = inserted[1]!;
  check(`${framework}: inserted fixture decodes`, loaded(picked), describe(picked));
  check(
    `${framework}: inserted fixture is the host-picked 96x48 raster`,
    picked.naturalWidth === 96 && picked.naturalHeight === 48 && picked.alt === "Revenue chart (host pick)",
    describe(picked),
  );
  check(
    `${framework}: inserted fixture is visibly distinct from the seeded one`,
    picked.src !== seeded[0]?.src,
  );

  const after = await hostDocument(page);
  const nodes = after.content ?? [];
  const imageIndexes = nodes.map((node, index) => (node.type === "image" ? index : -1)).filter((index) => index >= 0);
  check(`${framework}: the host document carries exactly two image nodes`, imageIndexes.length === 2, `${imageIndexes.length}`);
  const insertedIndex = imageIndexes[1] ?? -1;
  check(
    `${framework}: insertion lands after the seeded image, not at the fallback start`,
    insertedIndex > imageIndexes[0]!,
    `indexes=${imageIndexes.join(",")}`,
  );
  const beforeInsert = nodes[insertedIndex - 1];
  check(
    `${framework}: insertion follows the planted selection's paragraph exactly`,
    beforeInsert?.type === "paragraph" && beforeInsert === nodes[targetIndex],
    `before="${beforeInsert?.type}" target=${targetIndex} inserted=${insertedIndex}`,
  );
  check(
    `${framework}: the planted paragraph text is intact`,
    ((nodes[targetIndex]?.content as Array<{ text?: string }> | undefined) ?? [])
      .map((node) => node.text ?? "")
      .join("") === targetText && targetText.length > 0,
    targetText,
  );
  const insertedAttrs = (nodes[insertedIndex]?.attrs ?? {}) as Record<string, unknown>;
  check(
    `${framework}: the host document carries the callback's exact attributes`,
    insertedAttrs.alt === "Revenue chart (host pick)" && String(insertedAttrs.src).startsWith("data:image/png;base64,"),
    JSON.stringify(insertedAttrs.alt),
  );
  check(`${framework}: one click is one host change`, (await metric(page, "image-change-count")) === "1");
  check(`${framework}: one click is one host request`, (await metric(page, "image-request-count")) === "1");
  check(`${framework}: the host feedback reports both images`, (await metric(page, "image-count")) === "2");

  // Toggle off/on: the controlled image document survives an editor remount
  // with no callback echo, no silent stripping, and no further host request.
  const documentAfterInsert = JSON.stringify(after);
  await toggle.click();
  await page
    .waitForFunction(
      () => {
        const current = document.querySelector("[data-part='image-policy-editor']");
        return current !== null && current.querySelector("img") === null;
      },
      undefined,
      { timeout: 15_000 },
    )
    .catch(() => {});
  check(`${framework}: toggling off clears every image`, (await readImages(page)).length === 0);
  check(
    `${framework}: toggling off removes the Insert image command`,
    (await frame.locator("[data-command='insert-image'] button").count()) === 0,
  );
  await toggle.click();
  await page
    .waitForFunction(
      () => document.querySelectorAll("[data-part='image-policy-editor'] img").length === 2,
      undefined,
      { timeout: 15_000 },
    )
    .catch(() => {});
  const remounted = await readImages(page);
  check(`${framework}: remount restores both images`, remounted.length === 2, `${remounted.length}`);
  check(`${framework}: remount restores both decoded fixtures`, remounted.every(loaded), remounted.map(describe).join(" | "));
  check(`${framework}: remount does not echo a host change`, (await metric(page, "image-change-count")) === "1");
  check(`${framework}: remount does not re-request an asset`, (await metric(page, "image-request-count")) === "1");
  check(`${framework}: remount preserves the controlled document`, JSON.stringify(await hostDocument(page)) === documentAfterInsert);

  // Offline proof: the fixture must not resolve through DNS or any host.
  const external = requests.filter((url) => isExternal(url, base));
  check(`${framework}: makes no external request`, external.length === 0, external.slice(0, 3).join(", "));
  check(`${framework}: never requests the retired dead fixture`, requests.every((url) => !url.includes("x.test")));
  check(`${framework}: issues no image request at all for the inline raster`, imageRequests.length === 0, imageRequests.slice(0, 3).join(", "));
}

async function runSection(
  browserType: BrowserType,
  browserName: string,
  framework: Framework,
  base: string,
): Promise<void> {
  const attempts = 2;
  for (let attempt = 1; attempt <= attempts; attempt += 1) {
    const failuresBefore = failures;
    let browser: Browser | null = null;
    try {
      browser = await browserType.launch();
      const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
      const page = await context.newPage();
      await Promise.race([
        runSpecimen(page, base, framework),
        new Promise((_, reject) =>
          setTimeout(() => reject(new Error("section watchdog: no completion within 180s")), 180_000),
        ),
      ]);
      await browser.close();
      return;
    } catch (error) {
      await browser?.close().catch(() => {});
      if (attempt === attempts) throw error;
      failures = failuresBefore;
      console.log(`  retrying ${browserName}/${framework} on a fresh browser after: ${String(error).slice(0, 140)}`);
    }
  }
}

async function main(): Promise<void> {
  await startPreviews();
  const bases: Record<Framework, string> = {
    svelte: `http://127.0.0.1:${SERVERS.svelte.port}`,
    react: `http://127.0.0.1:${SERVERS.react.port}`,
  };
  const only = process.argv.find((arg) => arg.startsWith("--browser="))?.slice("--browser=".length);
  const engines = ([["chromium", chromium], ["webkit", webkit]] as Array<[string, BrowserType]>).filter(
    ([name]) => only === undefined || name === only,
  );
  if (engines.length === 0) throw new Error(`Unknown --browser=${only}`);
  for (const [browserName, browserType] of engines) {
    for (const framework of ["svelte", "react"] as Framework[]) {
      await runSection(browserType, browserName, framework, bases[framework]);
    }
  }
  if (failures > 0) {
    console.error(`\nrich-text image policy probe: ${failures} failing check(s)`);
    process.exit(1);
  }
  console.log("\nrich-text image policy probe: all checks passed");
  process.exit(0);
}

await main();
