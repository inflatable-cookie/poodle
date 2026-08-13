import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import Button from "../src/Button.svelte";

describe("Button (svelte)", () => {
  it("runs a DOM-spelled onclick alongside its own, rather than dropping it", async () => {
    // `{...restProps}` is spread before the component binds its own onclick, so
    // the DOM spelling used to be silently overwritten — no type error, no
    // warning, a dead button. The SettingsShell specimen shipped five of them.
    const onclick = vi.fn();
    const onClick = vi.fn();
    const { getByRole } = render(Button, { props: { onclick, onClick } as never });

    await fireEvent.click(getByRole("button"));

    expect(onClick).toHaveBeenCalledTimes(1);
    expect(onclick).toHaveBeenCalledTimes(1);
  });

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
