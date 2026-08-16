import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import FormLayout from "../src/FormLayout.svelte";
import { asSnippet } from "./snippet";

describe("FormLayout (svelte)", () => {
  it("renders the description and the field grid", () => {
    const { container } = render(FormLayout, {
      props: { description: "Fill in the details", children: asSnippet(() => "<div>field</div>") },
    });
    expect(container.querySelector(".poodle-form-layout__description")?.textContent).toContain(
      "Fill in the details",
    );
    expect(container.querySelector(".poodle-form-layout__grid")).not.toBeNull();
  });

  it("exposes the column count as a custom property on the grid", () => {
    const { container } = render(FormLayout, {
      props: { columns: 1, children: asSnippet(() => "<div>field</div>") },
    });
    const grid = container.querySelector(".poodle-form-layout__grid") as HTMLElement;
    expect(grid.style.getPropertyValue("--fl-columns")).toBe("1");
  });

  it("renders the error and success callouts with their messages", () => {
    const { container } = render(FormLayout, {
      props: { error: "Unable to save", success: "Saved", children: asSnippet(() => "<div/>") },
    });
    expect(container.textContent).toContain("Unable to save");
    expect(container.textContent).toContain("Saved");
  });

  it("renders the field error summary as a polite alert", () => {
    const { container } = render(FormLayout, {
      props: {
        fieldErrors: { Email: "is invalid", Role: "is required" },
        children: asSnippet(() => "<div/>"),
      },
    });
    const summary = container.querySelector(".poodle-form-layout__field-errors") as HTMLElement;
    expect(summary.getAttribute("role")).toBe("alert");
    expect(summary.getAttribute("aria-live")).toBe("polite");
    expect(summary.textContent).toContain("Please fix the following errors:");
    const items = [...summary.querySelectorAll("li")];
    expect(items.length).toBe(2);
    expect(items[0].textContent).toContain("Email");
    expect(items[0].textContent).toContain("is invalid");
  });

  it("renders the actions region only when an actions snippet is present", () => {
    const withActions = render(FormLayout, {
      props: { children: asSnippet(() => "<div/>"), actions: asSnippet(() => "<button>Save</button>") },
    });
    expect(withActions.container.querySelector(".poodle-form-layout__actions")).not.toBeNull();

    const withoutActions = render(FormLayout, {
      props: { children: asSnippet(() => "<div/>") },
    });
    expect(withoutActions.container.querySelector(".poodle-form-layout__actions")).toBeNull();
  });
});