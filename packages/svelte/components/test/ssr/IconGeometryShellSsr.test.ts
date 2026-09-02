import { render } from "svelte/server";
import { describe, expect, it } from "vitest";

import IconGeometryShell from "../IconGeometryShell.svelte";

describe("IconGeometryShell SSR", () => {
  it("emits the authored initial endpoint on a stable svg root", () => {
    const { body } = render(IconGeometryShell, {
      props: {
        pairId: "chevron-left-to-chevron-right",
        target: "from",
        initial: true,
      },
    });

    expect(body).toContain('data-poodle-icon-geometry=""');
    expect(body).toContain('viewBox="0 0 24 24"');
    expect(body).toContain("<svg");
    expect(body).toContain("<path");
    expect(body).not.toContain("chevron-left-to-chevron-right");
    const svgCount = body.split("<svg").length - 1;
    expect(svgCount).toBe(1);
  });

  it("snaps reduced policy to the semantic endpoint with no pair id", () => {
    const { body } = render(IconGeometryShell, {
      props: {
        pairId: "plus-to-x",
        target: "to",
        policy: "reduced",
        initial: true,
      },
    });
    expect(body).toContain("<path");
    expect(body).not.toContain("plus-to-x");
  });
});
