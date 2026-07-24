import { render as renderReact, cleanup as cleanupReact } from "@testing-library/react";
import { render as renderSvelte, cleanup as cleanupSvelte } from "@testing-library/svelte";
import { createElement } from "react";
import { describe, expect, it } from "vitest";
import SvRelationPicker from "../../packages/svelte/components/src/RelationPicker.svelte";
import SvMediaPreview from "../../packages/svelte/components/src/MediaPreview.svelte";
import { RelationPicker, MediaPreview } from "../../packages/react/components/src";

const cls = (r: ParentNode) => {
  const s = new Set<string>();
  for (const el of r.querySelectorAll("*")) for (const c of el.classList) if (c.startsWith("poodle-")) s.add(c);
  return s;
};

// The main parity gate renders every component with minimal props, so it only
// sees the EMPTY state. These guard the populated/empty boundary specifically —
// the exact class of divergence fixed in MediaPreview and RelationPicker, where
// one framework passed an always-defined snippet and the other passed null.
describe("populated-state parity", () => {
  it("RelationPicker WITH a selection renders the selection region in both", () => {
    const props = {
      items: [{ id: "a", label: "Alpha" }],
      selectedIds: ["a"],
      selectedItems: [{ id: "a", label: "Alpha" }],
    };
    const sv = cls(renderSvelte(SvRelationPicker as never, { props }).container);
    cleanupSvelte();
    const re = cls(renderReact(createElement(RelationPicker as never, props)).container);
    cleanupReact();
    expect(sv.has("poodle-picker-shell__selection")).toBe(true);
    expect(re.has("poodle-picker-shell__selection")).toBe(true);
  });

  it("MediaPreview WITHOUT media shows the placeholder in both", () => {
    const props = { title: "T" };
    const sv = cls(renderSvelte(SvMediaPreview as never, { props }).container);
    cleanupSvelte();
    const re = cls(renderReact(createElement(MediaPreview as never, props)).container);
    cleanupReact();
    expect(sv.has("poodle-media-thumbnail__placeholder")).toBe(true);
    expect(re.has("poodle-media-thumbnail__placeholder")).toBe(true);
  });
});
