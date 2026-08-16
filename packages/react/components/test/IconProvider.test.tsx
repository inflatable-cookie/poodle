import { render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { Icon } from "../src/Icon";
import { IconProvider } from "../src/IconProvider";
import type { IconSet } from "../src/types";

// Mirrors packages/svelte/components/test/IconProviderHarness.svelte + the
// "makes provider icons available on the initial render" case in
// icon-registry.test.ts — the provider set resolves before the default Lucide
// set, and a registered name renders without a resolution error.
describe("IconProvider + Icon (react)", () => {
  const icons: IconSet = {
    rocket: [["path", { d: "M1 2 3 4" }]],
  };

  it("renders a provider-set icon on the initial render without an error", () => {
    const error = vi.spyOn(console, "error").mockImplementation(() => undefined);
    const { container } = render(
      <IconProvider icons={icons}>
        <Icon name="rocket" ariaLabel="Rocket" />
      </IconProvider>,
    );

    expect(container.querySelector('path[d="M1 2 3 4"]')).not.toBeNull();
    expect(error).not.toHaveBeenCalled();
    error.mockRestore();
  });

  it("prefers the provider set over the default set for a shadowed name", () => {
    const override: IconSet = { search: [["path", { d: "M9 9 5 5" }]] };
    const { container, rerender } = render(
      <IconProvider icons={override}>
        <Icon name="search" ariaLabel="Search" />
      </IconProvider>,
    );

    expect(container.querySelector('path[d="M9 9 5 5"]')).not.toBeNull();

    // Dropping the provider falls back to the default Lucide glyph.
    rerender(<Icon name="search" ariaLabel="Search" />);
    expect(container.querySelector('path[d="M9 9 5 5"]')).toBeNull();
  });
});