import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import FileUpload from "../src/FileUpload.svelte";

describe("FileUpload accessibility (svelte)", () => {
  it("links describedBy to the native file control", () => {
    const { container } = render(FileUpload, {
      props: { describedBy: "licence-file-message" },
    });
    const input = container.querySelector('input[type="file"]') as HTMLInputElement;
    expect(input.getAttribute("aria-describedby")).toBe("licence-file-message");
    expect(
      container.querySelector(".poodle-file-upload__dropzone")?.getAttribute("aria-describedby"),
    ).toBe("licence-file-message");
  });
});
