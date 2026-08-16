import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { NavCard } from "../src/NavCard";

describe("NavCard (react)", () => {
  it("renders an anchor when href is set and a button otherwise", () => {
    const anchor = render(<NavCard title="Docs" href="/docs" />);
    const anchorEl = anchor.container.querySelector(".poodle-nav-card") as HTMLElement;
    expect(anchorEl.tagName).toBe("A");
    expect(anchorEl.getAttribute("href")).toBe("/docs");

    const button = render(<NavCard title="Docs" />);
    expect(button.container.querySelector(".poodle-nav-card")?.tagName).toBe("BUTTON");
  });

  it("defaults the accessible name to the title and renders badge and description", () => {
    const { container } = render(
      <NavCard title="Settings" badge="New" description="Tune preferences" />,
    );
    const root = container.querySelector(".poodle-nav-card") as HTMLElement;
    expect(root.getAttribute("aria-label")).toBe("Settings");
    expect(container.querySelector(".poodle-nav-card__badge")?.textContent).toBe("New");
    expect(container.querySelector(".poodle-nav-card__description")?.textContent).toBe(
      "Tune preferences",
    );
  });

  it("emits onClick and swallows clicks while disabled", () => {
    const onClick = vi.fn();
    const { container } = render(<NavCard title="Docs" onClick={onClick} />);
    fireEvent.click(container.querySelector(".poodle-nav-card") as HTMLElement);
    expect(onClick).toHaveBeenCalledTimes(1);

    const disabled = render(<NavCard title="Docs" disabled onClick={onClick} />);
    const disabledEl = disabled.container.querySelector(".poodle-nav-card") as HTMLButtonElement;
    expect(disabledEl.disabled).toBe(true);
    fireEvent.click(disabledEl);
    expect(onClick).toHaveBeenCalledTimes(1);
  });
});
