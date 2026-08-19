import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import "@inflatable-cookie/poodle-core/styles/pill.css";
import { Pill, type PillAppearance } from "@inflatable-cookie/poodle-react";

describe("packed @inflatable-cookie/poodle-react Pill", () => {
  it("exposes the corrected appearance axis from the packed root", () => {
    const appearances: PillAppearance[] = ["tint", "solid", "subtle", "badge"];
    for (const appearance of appearances) {
      const view = render(
        <Pill appearance={appearance} tone="success" dot>
          Packed
        </Pill>,
      );
      const root = view.container.querySelector(".poodle-pill") as HTMLElement;
      expect(root.dataset.appearance).toBe(appearance);
      expect(root.hasAttribute("data-fill")).toBe(false);
      expect(view.container.querySelector(".poodle-pill__dot")).not.toBeNull();
      view.unmount();
    }
  });

  it("defaults to the tint appearance with no fill axis", () => {
    const view = render(<Pill>Packed</Pill>);
    const root = view.container.querySelector(".poodle-pill") as HTMLElement;
    expect(root.dataset.appearance).toBe("tint");
    expect(root.hasAttribute("data-fill")).toBe(false);
    expect(root.textContent).toContain("Packed");
  });
});
