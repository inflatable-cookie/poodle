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

  it("renders aria-controls only when controls is set", () => {
    const { getByRole, rerender } = render(<Button controls="panel-1">Go</Button>);
    expect(getByRole("button").getAttribute("aria-controls")).toBe("panel-1");

    rerender(<Button>Go</Button>);
    expect(getByRole("button").getAttribute("aria-controls")).toBeNull();
  });

  it("renders formEncType and formMethod attributes", () => {
    const { getByRole } = render(
      <Button
        type="submit"
        formEncType="multipart/form-data"
        formMethod="post"
      >
        Submit
      </Button>,
    );
    const button = getByRole("button");
    expect(button.getAttribute("formenctype")).toBe("multipart/form-data");
    expect(button.getAttribute("formmethod")).toBe("post");
  });

  it("renders inline style and combines with maxWidth", () => {
    const { getByRole } = render(
      <Button style={{ color: "red" }} maxWidth="200px">
        Styled
      </Button>,
    );
    const button = getByRole("button");
    expect(button.style.color).toBe("red");
    expect(button.style.maxWidth).toBe("200px");
  });
});
