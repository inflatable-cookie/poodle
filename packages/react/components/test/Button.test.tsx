import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Button } from "../src/Button";

describe("Button (react)", () => {
  it("mounts a button with the root anatomy class", () => {
    const { getByRole } = render(<Button type="button">Go</Button>);
    const el = getByRole("button");
    expect(el.className).toContain("poodle-button");
    expect(el.textContent).toContain("Go");
  });

  it("applies disabled state", () => {
    const { getByRole } = render(<Button disabled>Go</Button>);
    expect((getByRole("button") as HTMLButtonElement).disabled).toBe(true);
  });
});
