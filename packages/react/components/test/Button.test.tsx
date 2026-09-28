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

  it("renders contract-listed form override attributes", () => {
    const { getByRole } = render(
      <Button
        type="submit"
        form="checkout"
        formAction="/submit"
        formEncType="multipart/form-data"
        formMethod="post"
        formNoValidate
        formTarget="_blank"
      >
        Submit
      </Button>,
    );
    const button = getByRole("button");
    expect(button.getAttribute("form")).toBe("checkout");
    expect(button.getAttribute("formaction")).toBe("/submit");
    expect(button.getAttribute("formenctype")).toBe("multipart/form-data");
    expect(button.getAttribute("formmethod")).toBe("post");
    expect(button.hasAttribute("formnovalidate")).toBe(true);
    expect(button.getAttribute("formtarget")).toBe("_blank");
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

  it("passes through native rest attributes the contract does not name as props", () => {
    const { getByRole } = render(
      <Button name="action" value="go" role="menuitem" aria-checked="true">
        Go
      </Button>,
    );
    const button = getByRole("menuitem");
    expect(button.getAttribute("name")).toBe("action");
    expect(button.getAttribute("value")).toBe("go");
    expect(button.getAttribute("role")).toBe("menuitem");
    expect(button.getAttribute("aria-checked")).toBe("true");
  });
});
