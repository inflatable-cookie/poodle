import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { EmptyState } from "../src/EmptyState";

describe("EmptyState (react)", () => {
  it("labels the section from ariaLabel, falling back to the title", () => {
    const { container } = render(<EmptyState title="No projects yet" />);
    const root = container.querySelector(".poodle-empty-state") as HTMLElement;
    expect(root.tagName).toBe("SECTION");
    expect(root.getAttribute("aria-label")).toBe("No projects yet");

    const labelled = render(<EmptyState title="No projects" ariaLabel="Empty projects" />);
    expect(labelled.container.querySelector(".poodle-empty-state")?.getAttribute("aria-label")).toBe(
      "Empty projects",
    );
  });

  it("renders the title as an h3 and the message when given", () => {
    const { container } = render(<EmptyState title="Nothing here" message="Create something." />);
    const heading = container.querySelector(".poodle-empty-state__copy h3") as HTMLElement;
    expect(heading.textContent).toBe("Nothing here");
    expect(container.querySelector(".poodle-empty-state__copy p")?.textContent).toContain(
      "Create something.",
    );
  });

  it("projects the variant and size data attributes", () => {
    const { container } = render(
      <EmptyState title="No results" variant="search" size="compact" />,
    );
    const root = container.querySelector(".poodle-empty-state") as HTMLElement;
    expect(root.dataset.variant).toBe("search");
    expect(root.dataset.size).toBe("compact");
  });

  it("keeps the visual region decorative", () => {
    const { container } = render(<EmptyState title="No results" variant="search" />);
    const visual = container.querySelector(".poodle-empty-state__visual") as HTMLElement;
    expect(visual.getAttribute("aria-hidden")).toBe("true");
    expect(visual.querySelector(".poodle-icon")).not.toBeNull();
  });

  it("renders the actions region only when actions content is present", () => {
    const withActions = render(
      <EmptyState title="No projects" actions={<button>Create</button>} />,
    );
    expect(withActions.container.querySelector(".poodle-empty-state__actions")).not.toBeNull();

    const withoutActions = render(<EmptyState title="No projects" />);
    expect(withoutActions.container.querySelector(".poodle-empty-state__actions")).toBeNull();
  });
});