import { render, screen } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Dialog from "../src/Dialog.svelte";

describe("Dialog size chrome (svelte)", () => {
  it("applies the resolved dialog size to the built-in close button", () => {
    render(Dialog, { props: { open: true, title: "Settings", showCloseButton: true, size: "xl" } });

    expect(screen.getByRole("button", { name: "Close dialog" }).dataset.size).toBe("xl");
  });
});
