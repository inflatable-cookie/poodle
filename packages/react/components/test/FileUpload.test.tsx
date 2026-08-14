import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { FileUpload } from "../src/FileUpload";

describe("FileUpload accessibility (react)", () => {
  it("links describedBy to the native file control", () => {
    const { container } = render(<FileUpload describedBy="licence-file-message" />);
    const input = container.querySelector('input[type="file"]') as HTMLInputElement;
    expect(input.getAttribute("aria-describedby")).toBe("licence-file-message");
  });
});
