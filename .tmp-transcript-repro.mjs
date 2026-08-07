// Nucleus-like repro: long blocks, scroll to middle, check slice coverage.
import { chromium } from "playwright";

const URL = "http://localhost:5199/repro.html";

const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 900, height: 900 } });
page.on("console", (msg) => {
  if (msg.type() === "error") console.log("[console.error]", msg.text());
});
page.on("pageerror", (err) => console.log("[pageerror]", err.message));
await page.goto(URL, { waitUntil: "networkidle" });
await page.waitForTimeout(1000);

const measure = () =>
  page.evaluate(() => {
    const viewport = document.querySelector(".poodle-agent-transcript__viewport");
    const runway = document.querySelector(".poodle-agent-transcript__runway");
    const slice = document.querySelector(".poodle-agent-transcript__slice");
    const blocks = [...slice.querySelectorAll(".poodle-agent-transcript__block")];
    const vpRect = viewport.getBoundingClientRect();
    const first = blocks[0]?.getBoundingClientRect();
    const last = blocks[blocks.length - 1]?.getBoundingClientRect();
    // Find real painted gaps: walk the blocks, which are contiguous in the slice.
    return {
      scrollTop: Math.round(viewport.scrollTop),
      clientHeight: viewport.clientHeight,
      scrollHeight: Math.round(viewport.scrollHeight),
      runwayHeight: Math.round(runway.getBoundingClientRect().height),
      blockCount: blocks.length,
      gapTop: first ? Math.round(Math.max(0, first.top - vpRect.top)) : -1,
      gapBottom: last ? Math.round(Math.max(0, vpRect.bottom - last.bottom)) : -1,
      pinned: document.querySelector(".poodle-agent-transcript").getAttribute("data-pinned"),
    };
  });

const fmt = (tag, m) =>
  console.log(
    `${tag}: scrollTop=${m.scrollTop} scrollH=${m.scrollHeight} runway=${m.runwayHeight} blocks=${m.blockCount} gapTop=${m.gapTop} gapBottom=${m.gapBottom} pinned=${m.pinned}`,
  );

fmt("initial", await measure());

// Jump straight to the middle of the runway.
await page.evaluate(() => {
  const viewport = document.querySelector(".poodle-agent-transcript__viewport");
  viewport.scrollTop = (viewport.scrollHeight - viewport.clientHeight) / 2;
  viewport.dispatchEvent(new Event("scroll"));
});
for (let frame = 0; frame < 10; frame += 1) {
  await page.waitForTimeout(120);
  fmt(`mid frame ${frame}`, await measure());
}

// Wheel up from the bottom in increments.
await page.evaluate(() => {
  const viewport = document.querySelector(".poodle-agent-transcript__viewport");
  viewport.scrollTop = viewport.scrollHeight;
  viewport.dispatchEvent(new Event("scroll"));
});
await page.waitForTimeout(300);
console.log("--- wheel up from bottom ---");
for (let step = 0; step < 16; step += 1) {
  await page.evaluate(() => {
    const viewport = document.querySelector(".poodle-agent-transcript__viewport");
    viewport.scrollTop = Math.max(0, viewport.scrollTop - 600);
    viewport.dispatchEvent(new Event("scroll"));
  });
  await page.waitForTimeout(150);
  fmt(`up ${step}`, await measure());
}

// Wheel down from the top in increments.
await page.evaluate(() => {
  const viewport = document.querySelector(".poodle-agent-transcript__viewport");
  viewport.scrollTop = 0;
  viewport.dispatchEvent(new Event("scroll"));
});
await page.waitForTimeout(300);
console.log("--- wheel down from top ---");
for (let step = 0; step < 16; step += 1) {
  await page.evaluate(() => {
    const viewport = document.querySelector(".poodle-agent-transcript__viewport");
    viewport.scrollTop = viewport.scrollTop + 600;
    viewport.dispatchEvent(new Event("scroll"));
  });
  await page.waitForTimeout(150);
  fmt(`down ${step}`, await measure());
}

await browser.close();
