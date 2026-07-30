/**
 * Which ancestors can actually clip an anchor.
 *
 * Overflow alone is not enough: an ancestor also has to be one the element is
 * laid out inside. These cases are the ones where those two differ, which is
 * where the bug lived — a fixed pane's DOM ancestors were narrowing the clip
 * rect for a trigger they cannot clip, so the surface was marked hidden and the
 * popover opened invisibly.
 */
import { beforeEach, describe, expect, test } from "vitest";

import { collectClipAncestors, resolveClipRect } from "../../packages/core/src/dom/anchor";

function build(html: string): HTMLElement {
  document.body.innerHTML = html;
  const anchor = document.querySelector<HTMLElement>("[data-anchor]");
  if (!anchor) throw new Error("fixture has no [data-anchor]");
  return anchor;
}

const ids = (elements: HTMLElement[]): string[] => elements.map((el) => el.id);

beforeEach(() => {
  document.body.innerHTML = "";
});

describe("collectClipAncestors", () => {
  test("a static anchor is clipped by every overflow ancestor", () => {
    const anchor = build(`
      <div id="outer" style="overflow: hidden">
        <div id="inner" style="overflow: auto">
          <button data-anchor></button>
        </div>
      </div>
    `);

    expect(ids(collectClipAncestors(anchor))).toEqual(["inner", "outer"]);
  });

  test("a fixed anchor is not clipped by an ordinary overflow ancestor", () => {
    const anchor = build(`
      <div id="clipper" style="overflow: hidden">
        <button data-anchor style="position: fixed"></button>
      </div>
    `);

    expect(ids(collectClipAncestors(anchor))).toEqual([]);
  });

  test("a fixed anchor is clipped by an ancestor that holds fixed descendants", () => {
    const anchor = build(`
      <div id="holder" style="overflow: hidden; transform: translateX(1px)">
        <button data-anchor style="position: fixed"></button>
      </div>
    `);

    expect(ids(collectClipAncestors(anchor))).toEqual(["holder"]);
  });

  /**
   * The reported case. The trigger itself is static; it is the *pane* that is
   * fixed, so reading only the anchor's own position would miss it.
   */
  test("a static anchor inside a fixed pane escapes the pane's own ancestors", () => {
    const anchor = build(`
      <div id="app" style="overflow: hidden">
        <div id="pane" style="position: fixed; inset: 0">
          <button data-anchor></button>
        </div>
      </div>
    `);

    expect(ids(collectClipAncestors(anchor))).toEqual([]);
  });

  test("the fixed pane still clips its own contents when it says so", () => {
    const anchor = build(`
      <div id="app" style="overflow: hidden">
        <div id="pane" style="position: fixed; inset: 0; overflow: hidden">
          <button data-anchor></button>
        </div>
      </div>
    `);

    expect(ids(collectClipAncestors(anchor))).toEqual(["pane"]);
  });

  test("a static overflow ancestor does not clip an absolutely positioned anchor", () => {
    const anchor = build(`
      <div id="positioned" style="position: relative; overflow: hidden">
        <div id="static" style="overflow: hidden">
          <button data-anchor style="position: absolute"></button>
        </div>
      </div>
    `);

    expect(ids(collectClipAncestors(anchor))).toEqual(["positioned"]);
  });

  test("clipping resumes above the containing block", () => {
    const anchor = build(`
      <div id="outer" style="overflow: hidden">
        <div id="pane" style="position: fixed">
          <div id="scroller" style="overflow: auto">
            <button data-anchor></button>
          </div>
        </div>
      </div>
    `);

    // The scroller is inside the pane and does clip; `outer` is above the
    // pane's containing block and cannot.
    expect(ids(collectClipAncestors(anchor))).toEqual(["scroller"]);
  });

  test("a null element has no clip ancestors", () => {
    expect(collectClipAncestors(null)).toEqual([]);
  });
});

describe("resolveClipRect", () => {
  const viewport = { width: 1000, height: 800 };

  /**
   * The failure as the consumer saw it: the surface was positioned correctly
   * but marked hidden, because a rect that cannot clip the anchor was narrowing
   * the clip to a region the anchor sits outside.
   */
  test("a fixed pane's anchor keeps the full viewport as its clip", () => {
    const anchor = build(`
      <div id="app" style="overflow: hidden">
        <div id="pane" style="position: fixed; inset: 0">
          <button data-anchor></button>
        </div>
      </div>
    `);

    const app = document.getElementById("app") as HTMLElement;
    app.getBoundingClientRect = () =>
      ({ top: 0, right: 200, bottom: 100, left: 0 }) as DOMRect;

    expect(resolveClipRect(anchor, viewport)).toEqual({
      top: 0,
      right: 1000,
      bottom: 800,
      left: 0,
    });
  });

  test("a real clipper still narrows the rect", () => {
    const anchor = build(`
      <div id="app" style="overflow: hidden">
        <button data-anchor></button>
      </div>
    `);

    const app = document.getElementById("app") as HTMLElement;
    app.getBoundingClientRect = () =>
      ({ top: 0, right: 200, bottom: 100, left: 0 }) as DOMRect;

    expect(resolveClipRect(anchor, viewport)).toEqual({
      top: 0,
      right: 200,
      bottom: 100,
      left: 0,
    });
  });
});
