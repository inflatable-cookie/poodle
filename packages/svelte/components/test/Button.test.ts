import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Button from "../src/Button.svelte";

describe("Button (svelte)", () => {
  it("mounts a button with the root anatomy class", () => {
    const { getByRole } = render(Button, { props: { type: "button" } });
    const el = getByRole("button");
    expect(el.className).toContain("poodle-button");
  });

  it("applies disabled state", () => {
    const { getByRole } = render(Button, { props: { disabled: true } });
    expect((getByRole("button") as HTMLButtonElement).disabled).toBe(true);
  });
});
