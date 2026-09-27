import { fireEvent, render, within } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import SidebarNav from "../src/SidebarNav.svelte";
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

describe("SidebarNav (svelte)", () => {
  it("renders a labelled nav region", () => {
    const { container } = render(SidebarNav, {
      props: { groups, ariaLabel: "Components" },
    });
    const nav = container.querySelector(".poodle-sidebar-nav") as HTMLElement;
    expect(nav.tagName).toBe("NAV");
    expect(nav.getAttribute("aria-label")).toBe("Components");
  });

  it("renders href items as anchors and href-less items as buttons", () => {
    const { container } = render(SidebarNav, { props: { groups } });
    const anchor = container.querySelector('a[href="/button"]');
    expect(anchor).not.toBeNull();
    const buttons = [...container.querySelectorAll("button")].map(
      (button) => button.textContent,
    );
    expect(buttons).toContain("Checkbox");
  });

  it("marks the active item with aria-current and the active class", () => {
    const { container } = render(SidebarNav, {
      props: { groups, value: "button" },
    });
    const anchor = container.querySelector('a[href="/button"]') as HTMLElement;
    expect(anchor.getAttribute("aria-current")).toBe("page");
    expect(anchor.classList.contains("poodle-sidebar-nav__item--active")).toBe(true);
  });

  it("renders disabled items inertly and never activates them", async () => {
    const onValueChange = vi.fn();
    const { container } = render(SidebarNav, { props: { groups, onValueChange } });
    const disabled = container.querySelector('button[disabled]') as HTMLButtonElement;
    expect(disabled.textContent).toBe("Switch");
    await fireEvent.click(disabled);
    expect(onValueChange).not.toHaveBeenCalled();
  });

  it("reports the selected value on activation", async () => {
    const onValueChange = vi.fn();
    const { container } = render(SidebarNav, { props: { groups, onValueChange } });
    const checkbox = [...container.querySelectorAll("button")].find(
      (button) => button.textContent === "Checkbox",
    ) as HTMLButtonElement;
    await fireEvent.click(checkbox);
    expect(onValueChange).toHaveBeenCalledWith("checkbox");
  });

  it("filters out empty groups before rendering", () => {
    const withEmpty = [
      ...groups,
      { id: "empty", label: "Empty", items: [] as SidebarNavGroup["items"] },
    ];
    const { container } = render(SidebarNav, { props: { groups: withEmpty } });
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
    const { container } = render(SidebarNav, { props: { groups: multi } });
    const titles = [...container.querySelectorAll(".poodle-sidebar-nav__group-title")];
    expect(titles.length).toBe(2);
    expect((titles[0] as HTMLElement).textContent).toBe("Foundation");
    expect((titles[0] as HTMLElement).getAttribute("title")).toBe("Foundation");
    const groupSections = [...container.querySelectorAll(".poodle-sidebar-nav__group")];
    expect(groupSections[0].getAttribute("data-separated")).toBe("true");
  });

  describe("endLabel", () => {
    const counted: SidebarNavGroup[] = [
      {
        id: "library",
        label: "Library",
        items: [
          { value: "videos", label: "Videos", href: "/videos", endLabel: "198" },
          { value: "audio", label: "Audio", endLabel: "42" },
          { value: "images", label: "Images", endLabel: "7", disabled: true },
          { value: "notes", label: "Notes", endLabel: null },
        ],
      },
    ];

    it("keeps the accessible name as label and describes link and button items with endLabel", () => {
      const { container } = render(SidebarNav, { props: { groups: counted } });
      const view = within(container);
      const link = view.getByRole("link", { name: "Videos", description: "198" });
      expect(link.getAttribute("data-end-label")).toBe("true");
      const button = view.getByRole("button", { name: "Audio", description: "42" });
      const disabled = view.getByRole("button", { name: "Images", description: "7" });
      expect((disabled as HTMLButtonElement).disabled).toBe(true);

      for (const item of [link, button, disabled]) {
        const endLabel = item.querySelector(".poodle-sidebar-nav__end-label") as HTMLElement;
        expect(endLabel.getAttribute("aria-hidden")).toBe("true");
        expect(item.getAttribute("aria-describedby")).toBe(endLabel.id);
        expect(item.querySelector(".poodle-sidebar-nav__label")?.textContent).toBe(
          item === link ? "Videos" : item === button ? "Audio" : "Images",
        );
      }
      expect(link.querySelector(".poodle-sidebar-nav__end-label")?.textContent).toBe("198");
    });

    it("renders no end label or description when endLabel is unset or null", () => {
      const { container } = render(SidebarNav, { props: { groups: counted } });
      const notes = within(container).getByRole("button", { name: "Notes" });
      expect(notes.hasAttribute("aria-describedby")).toBe(false);
      expect(notes.hasAttribute("data-end-label")).toBe(false);
      expect(notes.querySelector("span")).toBeNull();
      expect(notes.textContent).toBe("Notes");
    });

    it("gives every end label a unique id", () => {
      const { container } = render(SidebarNav, { props: { groups: counted } });
      const ids = [...container.querySelectorAll(".poodle-sidebar-nav__end-label")].map((el) => el.id);
      expect(ids.length).toBe(3);
      expect(new Set(ids).size).toBe(3);
    });
  });
});
