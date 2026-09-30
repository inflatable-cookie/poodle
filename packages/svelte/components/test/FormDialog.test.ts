import { render, screen } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import FormDialog from "../src/FormDialog.svelte";

describe("FormDialog (svelte)", () => {
  it("announces the full submission error as an alert", () => {
    const message = "Unable to save your changes. Check the required fields and try again.";
    render(FormDialog, {
      props: { open: true, title: "Edit settings", error: message, initialFocus: "none" },
    });

    const alert = screen.getByRole("alert");
    expect(alert.getAttribute("aria-live")).toBe("assertive");
    expect(alert.textContent).toContain(message);
  });
});
