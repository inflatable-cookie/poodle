import { describe, expect, it, vi } from "vitest";

import { circleX, search } from "@inflatable-cookie/poodle-core/icons";
import { resolveIconNodes } from "../src/icon-registry";

describe("icon registry", () => {
  it("resolves application icons before the default Lucide set", () => {
    const override = [["path", { d: "M0 0" }]] as const;
    expect(resolveIconNodes("search", { search: override as never })).toBe(override);
    expect(resolveIconNodes("search")).toBe(search);
  });

  it("reports an unresolved name and returns a visible error glyph", () => {
    const error = vi.spyOn(console, "error").mockImplementation(() => undefined);
    expect(resolveIconNodes("missing-svelte-test-icon")).toBe(circleX);
    expect(error).toHaveBeenCalledWith(expect.stringContaining("missing-svelte-test-icon"));
    error.mockRestore();
  });
});
