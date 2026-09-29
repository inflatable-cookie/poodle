import { afterAll, beforeAll, describe, expect, test } from "bun:test";
import { createServer, type Server } from "node:http";

import { chromium, type Browser, type BrowserContext } from "playwright";

import { PageDeadlineError, captureSession } from "./session";

/**
 * Planted cases for the shared preview harness (`session.ts`).
 *
 * These assert the three failures the papercut batch names, against a real
 * headless Chromium and a local page: a native file chooser and a dialog are
 * absorbed instead of wedging the run, and a navigation that never answers
 * ends at the per-page deadline with a fresh page rather than with the run.
 *
 * Before the helper, a harness had to re-learn each by watching a batch die —
 * the Button comparator lost fixture nine to page degradation, and a
 * catalogue sweep had no chooser handler or deadline at all.
 */

const PAGE = `<!doctype html><html><body>
  <input id="pick" type="file">
  <button id="ask" onclick="window.confirm('proceed?')">ask</button>
  <div id="ready">ready</div>
</body></html>`;

let server: Server;
let port = 0;
let browser: Browser;

beforeAll(async () => {
  server = createServer((req, res) => {
    // Hold the socket open and never respond: the navigation the deadline runs
    // against. Everything else serves the planted page.
    if (req.url === "/hang") return;
    res.setHeader("content-type", "text/html");
    res.end(PAGE);
  });
  await new Promise<void>((resolve, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", resolve);
  });
  const address = server.address();
  if (!address || typeof address === "string") throw new Error("no listener address");
  port = address.port;
  browser = await chromium.launch();
});

afterAll(async () => {
  await browser?.close();
  server?.closeAllConnections();
  await new Promise<void>((resolve) => server?.close(() => resolve()));
});

function base(): string {
  return `http://127.0.0.1:${port}`;
}

async function withContext(run: (context: BrowserContext) => Promise<void>): Promise<void> {
  const context = await browser.newContext();
  try {
    await run(context);
  } finally {
    await context.close();
  }
}

describe("CaptureSession", () => {
  test(
    "absorbs a native file chooser so the page keeps moving",
    async () => {
      await withContext(async (context) => {
        const session = captureSession({ context });
        await session.run("open", (page) => page.goto(base()));
        await session.run("pick", async (page) => {
          await page.click("#pick");
          await page.waitForTimeout(100);
        });
        expect(session.fileChoosers).toBe(1);
        // The absorbed chooser did not poison the page.
        expect(await session.run("ready", (page) => page.textContent("#ready"))).toBe("ready");
        await session.close();
      });
    },
    30_000,
  );

  test(
    "absorbs a dialog and records its type",
    async () => {
      await withContext(async (context) => {
        const session = captureSession({ context });
        await session.run("open", (page) => page.goto(base()));
        await session.run("ask", async (page) => {
          await page.click("#ask");
          await page.waitForTimeout(50);
        });
        expect(session.dialogs).toEqual(["confirm"]);
        expect(await session.run("ready", (page) => page.textContent("#ready"))).toBe("ready");
        await session.close();
      });
    },
    30_000,
  );

  test(
    "a hung navigation fails at the deadline and is replaced, not awaited",
    async () => {
      await withContext(async (context) => {
        const session = captureSession({ context, deadlineMs: 1_000 });
        const opener = await session.run("warm", async (page) => {
          await page.goto(base());
          return page;
        });

        const started = Date.now();
        let thrown: unknown;
        try {
          await session.run("hang", (page) => page.goto(`${base()}/hang`));
        } catch (error) {
          thrown = error;
        }

        expect(thrown).toBeInstanceOf(PageDeadlineError);
        expect((thrown as Error).message).toContain("hang");
        expect(Date.now() - started).toBeLessThan(4_000);
        // The poisoned page was recycled, and the replacement still works.
        expect(session.page).not.toBe(opener);
        const recovered = await session.run("after", async (page) => {
          await page.goto(base());
          return page.textContent("#ready");
        });
        expect(recovered).toBe("ready");
        await session.close();
      });
    },
    30_000,
  );

  test(
    "recycles the page on the shared cadence",
    async () => {
      await withContext(async (context) => {
        const session = captureSession({ context, recycleAfter: 2 });
        await session.run("one", (page) => page.goto(base()));
        const first = session.page;
        await session.run("two", (page) => page.goto(base()));
        expect(session.page).toBe(first);
        await session.run("three", (page) => page.goto(base()));
        expect(session.page).not.toBe(first);
        await session.close();
      });
    },
    30_000,
  );
});
