import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { SidebarNav } from "../src/SidebarNav";
import type { SidebarNavGroup } from "../src/types";

const groups: SidebarNavGroup[] = [
  {
    id: "foundation",
    label: "Foundation",
    items: [
      { value: "button", label: "Button", href: "/button" },
      { value: "checkbox", label: "Checkbox" },
      { value: "switch", label: "Switch", disabled: true },
    ],
  },
];

describe("SidebarNav (react)", () => {
  it("renders a labelled nav region", () => {
    const { container } = render(<SidebarNav groups={groups} ariaLabel="Components" />);
    const nav = container.querySelector(".poodle-sidebar-nav") as HTMLElement;
    expect(nav.tagName).toBe("NAV");
    expect(nav.getAttribute("aria-label")).toBe("Components");
  });

  it("renders href items as anchors and href-less items as buttons", () => {
    const { container } = render(<SidebarNav groups={groups} />);
    const anchor = container.querySelector('a[href="/button"]');
    expect(anchor).not.toBeNull();
    const buttons = [...container.querySelectorAll("button")].map((button) => button.textContent);
    expect(buttons).toContain("Checkbox");
  });

  it("marks the active item with aria-current and the active class", () => {
    const { container } = render(<SidebarNav groups={groups} value="button" />);
    const anchor = container.querySelector('a[href="/button"]') as HTMLElement;
    expect(anchor.getAttribute("aria-current")).toBe("page");
    expect(anchor.classList.contains("poodle-sidebar-nav__item--active")).toBe(true);
  });

  it("renders disabled items inertly and never activates them", () => {
    const onValueChange = vi.fn();
    const { container } = render(<SidebarNav groups={groups} onValueChange={onValueChange} />);
    const disabled = container.querySelector("button[disabled]") as HTMLButtonElement;
    expect(disabled.textContent).toBe("Switch");
    fireEvent.click(disabled);
    expect(onValueChange).not.toHaveBeenCalled();
  });

  it("reports the selected value on activation", () => {
    const onValueChange = vi.fn();
    const { container } = render(<SidebarNav groups={groups} onValueChange={onValueChange} />);
    const checkbox = [...container.querySelectorAll("button")].find(
      (button) => button.textContent === "Checkbox",
    ) as HTMLButtonElement;
    fireEvent.click(checkbox);
    expect(onValueChange).toHaveBeenCalledWith("checkbox");
  });

  it("filters out empty groups before rendering", () => {
    const withEmpty = [
      ...groups,
      { id: "empty", label: "Empty", items: [] as SidebarNavGroup["items"] },
    ];
    const { container } = render(<SidebarNav groups={withEmpty} />);
    expect(container.querySelectorAll(".poodle-sidebar-nav__group").length).toBe(1);
  });

  it("renders the group title with the full label and marks multiple groups as separated", () => {
    const multi = [
      ...groups,
      {
        id: "composites",
        label: "Composites",
        items: [{ value: "table", label: "Table" }],
      },
    ];
    const { container } = render(<SidebarNav groups={multi} />);
    const titles = [...container.querySelectorAll(".poodle-sidebar-nav__group-title")];
    expect(titles.length).toBe(2);
    expect((titles[0] as HTMLElement).textContent).toBe("Foundation");
    expect((titles[0] as HTMLElement).getAttribute("title")).toBe("Foundation");
    const groupSections = [...container.querySelectorAll(".poodle-sidebar-nav__group")];
    expect(groupSections[0].getAttribute("data-separated")).toBe("true");
  });
});
