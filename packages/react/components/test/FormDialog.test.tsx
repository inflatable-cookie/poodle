import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { FormDialog } from "../src/FormDialog";

describe("FormDialog (react)", () => {
  it("announces the full submission error as an alert", () => {
    const message = "Unable to save your changes. Check the required fields and try again.";
    render(<FormDialog open title="Edit settings" error={message} initialFocus="none" />);

    const alert = screen.getByRole("alert");
    expect(alert.getAttribute("aria-live")).toBe("assertive");
    expect(alert.textContent).toContain(message);
  });
});
