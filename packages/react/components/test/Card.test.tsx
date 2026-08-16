import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Card } from "../src/Card";

describe("Card (react)", () => {
  const rootOf = (container: HTMLElement) =>
    container.querySelector(".poodle-card") as HTMLElement;

  it("projects variant, layout, interactive, and selected data attributes", () => {
    const root = rootOf(
      render(<Card variant="elevated" layout="horizontal" interactive selected>body</Card>)
        .container,
    );
    expect(root.dataset.variant).toBe("elevated");
    expect(root.dataset.layout).toBe("horizontal");
    expect(root.dataset.interactive).toBe("true");
    expect(root.dataset.selected).toBe("true");
  });

  it("renders media, header, body, and footer regions from children", () => {
    const { container } = render(
      <Card
        media
        mediaContent={<img src="/hero.png" alt="" />}
        header={<span>Title</span>}
        footer={<span>Actions</span>}
      >
        Body copy
      </Card>,
    );
    expect(container.querySelector(".poodle-card__media")).not.toBeNull();
    expect(container.querySelector(".poodle-card__header")?.textContent).toBe("Title");
    expect(container.querySelector(".poodle-card__body")?.textContent).toBe("Body copy");
    expect(container.querySelector(".poodle-card__footer")?.textContent).toBe("Actions");
  });

  it("omits media, header, and footer regions when their children are absent", () => {
    const { container } = render(<Card>Only body</Card>);
    expect(container.querySelector(".poodle-card__media")).toBeNull();
    expect(container.querySelector(".poodle-card__header")).toBeNull();
    expect(container.querySelector(".poodle-card__footer")).toBeNull();
    expect(container.querySelector(".poodle-card__body")?.textContent).toBe("Only body");
  });
});
