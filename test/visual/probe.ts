import { chromium } from "playwright";

import { specimenUrl } from "./capture";
import { SMOKE_AXES } from "./config";
import { startPreviews } from "./server";
import { captureSession } from "./session";

/**
 * Ad-hoc triage probe for the visual gate:
 *
 *   bun test/visual/probe.ts <slug> <css-selector> [prop ...]
 *
 * Loads the same pinned specimen page in both previews and prints the element's
 * box plus any requested computed styles side by side.
 */

const [slug, selector, ...props] = process.argv.slice(2);
if (!slug || !selector) {
  console.error("usage: bun test/visual/probe.ts <slug> <selector> [css-prop ...]");
  process.exit(2);
}

const servers = await startPreviews();
const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
const session = captureSession({ context });

for (const framework of ["svelte", "react"] as const) {
  const readings = await session.run(`${framework} ${slug}`, async (page) => {
    await page.goto(specimenUrl(servers.urls[framework], slug, SMOKE_AXES[0]), {
      waitUntil: "load",
    });
    await page.locator(".poodle-component-page__section").first().waitFor();

    return page.evaluate(
      ([sel, wanted, limit]) => {
        return Array.from(document.querySelectorAll(sel as string))
          .slice(0, limit as number)
          .map((el) => {
            const rect = el.getBoundingClientRect();
            const styles = getComputedStyle(el);
            const picked: Record<string, string> = {};
            for (const prop of wanted as string[]) picked[prop] = styles.getPropertyValue(prop);
            return {
              tag: el.tagName.toLowerCase(),
              cls: el.className,
              w: Math.round(rect.width * 100) / 100,
              h: Math.round(rect.height * 100) / 100,
              ...picked,
            };
          });
      },
      [selector, props, Number(process.env.PROBE_LIMIT ?? 5)] as const,
    );
  });

  console.log(`\n${framework}:`);
  for (const reading of readings) console.log(" ", JSON.stringify(reading));
}

await session.close();
await browser.close();
await servers.stop();
