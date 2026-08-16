import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { PageHeader } from "../src/PageHeader";

describe("PageHeader (react)", () => {
  it("renders a header with the configurable heading level", () => {
    const { container } = render(<PageHeader title="Components" level={3} />);
    const root = container.querySelector(".poodle-page-header") as HTMLElement;
    expect(root.tagName).toBe("HEADER");
    const heading = root.querySelector("h3") as HTMLElement;
    expect(heading.textContent).toContain("Components");
  });

  it("renders the eyebrow, section, and subtitle in the default posture", () => {
    const { container } = render(
      <PageHeader
        title="Button"
        section="Primitive"
        eyebrow="Foundation"
        subtitle="Primary control."
      />,
    );
    expect(container.querySelector(".poodle-page-header__eyebrow")?.textContent).toBe("Foundation");
    expect(container.querySelector(".poodle-page-header__section")?.textContent).toBe("Primitive");
    expect(container.querySelector(".poodle-page-header__subtitle")?.textContent).toContain(
      "Primary control.",
    );
  });

  it("renders the count badge through the Pill primitive", () => {
    const { container } = render(<PageHeader title="Components" count={42} />);
    const count = container.querySelector(".poodle-page-header__count") as HTMLElement;
    expect(count.querySelector(".poodle-pill")?.textContent).toContain("42");
  });

  it("renders the back link with the resolved display label and context dot", () => {
    const { container } = render(
      <PageHeader
        title="Members"
        backHref="/settings"
        backLabel="Back to settings"
        backIsContextual
      />,
    );
    const back = container.querySelector(".poodle-page-header__back--text") as HTMLAnchorElement;
    expect(back.getAttribute("href")).toBe("/settings");
    expect(back.textContent).toContain("settings");
    expect(back.querySelector(".poodle-page-header__context-dot")).not.toBeNull();

    const iconVariant = container.querySelector(
      ".poodle-page-header__back--icon",
    ) as HTMLAnchorElement;
    expect(iconVariant.getAttribute("aria-label")).toBe("Back to settings");
  });

  it("renders the banner message through a polite Callout", () => {
    const { container } = render(<PageHeader title="Members" bannerMessage="Read-only view" />);
    const banner = container.querySelector(".poodle-page-header__banner") as HTMLElement;
    expect(banner.textContent).toContain("Read-only view");
  });

  it("renders the actions region when an actions snippet is provided", () => {
    const { container } = render(
      <PageHeader title="Members" actions={<button>Edit</button>} />,
    );
    expect(container.querySelector(".poodle-page-header__actions")).not.toBeNull();
  });

  it("swaps section and title in the entity-detail posture", () => {
    const { container } = render(
      <PageHeader title="Member name" section="Members" posture="entity-detail" />,
    );
    const heading = container.querySelector(".poodle-page-header__title") as HTMLElement;
    expect(heading.textContent).toContain("Members");
    expect(container.querySelector(".poodle-page-header__subtitle")?.textContent).toContain(
      "Member name",
    );
  });
});