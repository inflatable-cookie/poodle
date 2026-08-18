import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Dialog } from "../src/Dialog";

describe("Dialog size chrome (react)", () => {
  it("applies the resolved dialog size to the built-in close button", () => {
    render(<Dialog open title="Settings" showCloseButton size="xl" />);

    expect(screen.getByRole("button", { name: "Close dialog" }).dataset.size).toBe("xl");
  });
});
