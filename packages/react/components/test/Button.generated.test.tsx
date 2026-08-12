import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Button } from "../src/Button";
import { buttonDefinition } from "../../preview/src/generated/button";

// Card 041 R2: the DOM reads the generated artifact — the data-* attribute
// names and part class names come from button.rs via `button-ts`, never
// from hand-written literals in this component. A rename in the definition
// moves the DOM; `effigy ir:check` gates drift in the artifact.

function attributeName(id: string): string {
  const attribute = buttonDefinition.attributes.find((entry) => entry.id === id);
  if (!attribute) throw new Error(`definition lacks attribute '${id}'`);
  return attribute.name;
}

function partClass(id: string): string {
  const part = buttonDefinition.parts.find((entry) => entry.id === id);
  if (!part) throw new Error(`definition lacks part '${id}'`);
  return part.className;
}

describe("Button (react) — generated definition drives the DOM", () => {
  it("emits data attributes under the definition's names and values", () => {
    const { getByRole } = render(<Button variant="primary" tone="danger" truncate />);
    const el = getByRole("button") as HTMLButtonElement;

    expect(el.getAttribute(attributeName("variant"))).toBe("primary");
    expect(el.getAttribute(attributeName("tone"))).toBe("danger");
    expect(el.hasAttribute(attributeName("truncate"))).toBe(true);
    // The definition's attribute entries are what the DOM carries — the
    // names in this test came from the artifact itself.
    expect(attributeName("tone")).toBe("data-tone");
    expect(attributeName("fit")).toBe("data-fit");
    expect(attributeName("pressed")).toBe("data-pressed");
  });

  it("omits data-tone when the tone is default and omits data-fit at the default fit", () => {
    const { getByRole } = render(<Button>Go</Button>);
    const el = getByRole("button") as HTMLButtonElement;
    expect(el.hasAttribute(attributeName("tone"))).toBe(false);
    expect(el.hasAttribute(attributeName("fit"))).toBe(false);
  });

  it("always emits data-loading, even as false (BTN-08)", () => {
    const { getByRole } = render(<Button>Go</Button>);
    const el = getByRole("button") as HTMLButtonElement;
    expect(el.getAttribute(attributeName("loading"))).toBe("false");
  });

  it("renders the anatomy under the definition's part classes", () => {
    const { getByRole } = render(
      <Button loading leadingIcon="plus" trailingIcon="check">
        Go
      </Button>,
    );
    const el = getByRole("button") as HTMLButtonElement;
    expect(el.className).toContain(partClass("root"));
    expect(el.querySelector(`.${partClass("spinner")}`)).not.toBeNull();
    expect(el.querySelectorAll(`.${partClass("leading-icon")}`).length).toBeGreaterThan(0);
    // Leading and trailing icons share one DOM class (B §2).
    expect(el.querySelectorAll(`.${partClass("trailing-icon")}`).length).toBeGreaterThan(0);
  });
});
