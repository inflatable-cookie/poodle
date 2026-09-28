import { readFileSync } from "node:fs";
import { join } from "node:path";
import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import HistoryCenter from "../src/HistoryCenter.svelte";
import MessageCenter from "../src/MessageCenter.svelte";

/**
 * HistoryCenter and MessageCenter content must fit the popover surface at
 * the surface's own minimum and maximum widths. Sizing flows through the
 * surface's min/max custom properties; the content classes carry no fixed
 * width of their own (a `width: clamp(...)` against a padded surface pushed
 * content past its rounded edge), and the HistoryCenter inline rename
 * input carries no width-plus-margin of its own. happy-dom has no layout
 * engine, so the suite pins the mechanism: the surface's bounds in the DOM
 * and the content rules' declarations in the shared stylesheet.
 */

const coreStyles = join(import.meta.dirname, "../../../core/src/styles");

function baseRule(css: string, selector: string): string {
  const start = css.indexOf(`${selector} {`);
  if (start === -1) throw new Error(`missing rule ${selector}`);
  const open = css.indexOf("{", start);
  const close = css.indexOf("}", open);
  return css.slice(open + 1, close).replace(/\/\*[\s\S]*?\*\//g, "");
}

function declarationNames(block: string): string[] {
  return block
    .split(";")
    .map((part) => part.trim())
    .filter(Boolean)
    .map((declaration) => declaration.slice(0, declaration.indexOf(":")).trim());
}

function surfaceOf(): HTMLElement {
  const surface = document.querySelector(".poodle-popover__surface") as HTMLElement;
  if (!surface) throw new Error("popover surface did not mount");
  return surface;
}

describe("composite surface fit (svelte)", () => {
  it("sizes HistoryCenter through the surface bounds with no content width of its own", () => {
    render(HistoryCenter, { props: { defaultOpen: true } });

    const surface = surfaceOf();
    expect(surface.style.getPropertyValue("--poodle-popover-surface-min-width")).toBe(
      "min(28rem, calc(100vw - 2rem))",
    );
    expect(surface.style.getPropertyValue("--poodle-popover-surface-max-width")).toBe(
      "min(38rem, calc(100vw - 2rem))",
    );

    const content = surface.querySelector(".poodle-history-center") as HTMLElement;
    expect(content.style.width).toBe("");

    const css = readFileSync(join(coreStyles, "history-center.css"), "utf8");
    expect(declarationNames(baseRule(css, ".poodle-history-center"))).not.toContain("width");
  });

  it("sizes MessageCenter through the surface bounds with no content width of its own", () => {
    render(MessageCenter, { props: { defaultOpen: true } });

    const surface = surfaceOf();
    expect(surface.style.getPropertyValue("--poodle-popover-surface-min-width")).toBe(
      "min(24rem, calc(100vw - 2rem))",
    );
    expect(surface.style.getPropertyValue("--poodle-popover-surface-max-width")).toBe(
      "min(30rem, calc(100vw - 2rem))",
    );

    const content = surface.querySelector(".poodle-message-center") as HTMLElement;
    expect(content.style.width).toBe("");

    const css = readFileSync(join(coreStyles, "message-center.css"), "utf8");
    expect(declarationNames(baseRule(css, ".poodle-message-center"))).not.toContain("width");
  });

  it("gives the HistoryCenter rename input no width or horizontal margin of its own", () => {
    const css = readFileSync(join(coreStyles, "history-center.css"), "utf8");
    // `width: 100%` in border-box plus a horizontal margin spills past the
    // edge by twice the margin; the input flexes with no width and the
    // controls row zeroes its margin.
    expect(
      declarationNames(baseRule(css, ".poodle-history-center__rename-input")),
    ).not.toContain("width");
    const override = baseRule(
      css,
      ".poodle-history-center__picker-controls .poodle-history-center__rename-input",
    );
    expect(override.replace(/\s+/g, "")).toContain("margin:0");
  });
});
