import assert from "node:assert/strict";
import { resolve } from "node:path";

import { chromium } from "playwright";
import { createServer } from "vite";

const previewRoot = resolve(import.meta.dir, "..");
const watchdog = setTimeout(() => {
  console.error("Svelte specimen capture test exceeded its 120 second bound");
  process.exit(124);
}, 120_000);
watchdog.unref();

const server = await createServer({
  configFile: resolve(previewRoot, "vite.config.ts"),
  root: previewRoot,
  logLevel: "error",
  server: { host: "127.0.0.1", port: 0, strictPort: false },
});
let browser: Awaited<ReturnType<typeof chromium.launch>> | undefined;

try {
  await server.listen();
  const address = server.httpServer?.address();
  assert.ok(address && typeof address !== "string", "Vite did not open a TCP listener");
  const baseUrl = `http://127.0.0.1:${address.port}`;

  browser = await chromium.launch({ headless: true, timeout: 30_000 });
  const page = await browser.newPage({
    viewport: { width: 1280, height: 900 },
    deviceScaleFactor: 1,
  });
  const pageErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error.message));

  for (const slug of ["button", "keyboard"]) {
    await page.goto(`${baseUrl}/?capture=specimen#components/${slug}`, {
      waitUntil: "domcontentloaded",
      timeout: 60_000,
    });
    const frame = page.locator(`[data-specimen-capture="${slug}"][data-capture-ready="${slug}"]`);
    await frame.waitFor({ state: "visible", timeout: 30_000 });

    const dimensions = await frame.evaluate((element) => {
      const bounds = element.getBoundingClientRect();
      return {
        width: bounds.width,
        height: bounds.height,
        scale: element.getAttribute("data-capture-device-scale"),
        referenceScale: element.getAttribute("data-capture-reference-device-scale"),
      };
    });
    assert.equal(dimensions.width, 1280, `${slug} capture frame width`);
    assert.equal(dimensions.height, 900, `${slug} capture frame height`);
    assert.equal(dimensions.scale, "1", `${slug} capture device scale`);
    assert.equal(dimensions.referenceScale, "1", `${slug} reference device scale`);
    assert.equal(
      await page.locator(
        ".poodle-app-shell, .poodle-app-top-bar, .poodle-app-main, .poodle-catalogue-layout, .poodle-component-page__title, .poodle-component-page__description, .poodle-component-page__section-title",
      ).count(),
      0,
      `${slug} specimen capture omits the shell and component documentation`,
    );
    assert.equal(
      await frame.locator(".poodle-specimen-layout > .poodle-tabs").count(),
      0,
      `${slug} specimen capture omits its tab strip`,
    );
    assert.ok(await frame.locator(".poodle-specimen-group").count(), `${slug} examples render`);

    if (slug === "button") {
      assert.ok(await frame.getByRole("button", { name: "Save changes" }).count());
    } else {
      assert.ok(await frame.getByLabel("Playable keyboard").count());
    }
  }

  await page.goto(`${baseUrl}/#components/button`, {
    waitUntil: "domcontentloaded",
    timeout: 60_000,
  });
  await page.locator(".poodle-app-top-bar").waitFor({ state: "visible", timeout: 30_000 });
  assert.ok(await page.locator(".poodle-catalogue-sidebar").count(), "interactive preview keeps its sidebar");
  await page.getByRole("tablist", { name: "Specimen view" }).waitFor({ state: "visible", timeout: 30_000 });
  assert.ok(
    await page.getByRole("tablist", { name: "Specimen view" }).count(),
    "interactive preview keeps the specimen tabs",
  );
  assert.deepEqual(pageErrors, [], "preview emitted no browser errors");
  console.log("Svelte specimen capture passed for button and keyboard; interactive preview shell remains enabled.");
} finally {
  await browser?.close();
  await server.close();
  clearTimeout(watchdog);
}
