import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { RefSelect } from "../src/RefSelect";
import type { RefOption } from "../src/types";

// Mirrors packages/svelte/components/test/RefSelect.test.ts — filtering, the
// current marker and host-driven search must behave identically.
const refs: RefOption[] = [
  { value: "main", label: "main", kind: "branch", description: "a1b2c3d", group: "Branches" },
  { value: "tree-component", label: "tree-component", kind: "branch", group: "Branches" },
  { value: "v1.4.0", label: "v1.4.0", kind: "tag", group: "Tags" },
];

describe("RefSelect (react)", () => {
  const triggerOf = (container: HTMLElement) =>
    container.querySelector(".poodle-ref-select__trigger") as HTMLButtonElement;
  // The surface is portalled to the theme root; `aria-controls` is the link
  // back, and going through it keeps concurrent instances apart.
  const surfaceOf = (container: HTMLElement) =>
    document.getElementById(
      triggerOf(container).getAttribute("aria-controls") ?? "",
    ) as HTMLElement | null;
  const rowsOf = (container: HTMLElement) =>
    Array.from(
      surfaceOf(container)?.querySelectorAll<HTMLElement>(".poodle-ref-select__option") ?? [],
    );

  it("portals its surface out of the trigger's subtree", () => {
    const { container } = render(<RefSelect refs={refs} value="main" />);
    fireEvent.click(triggerOf(container));

    const surface = surfaceOf(container)!;
    expect(container.querySelector(".poodle-ref-select__surface")).toBeNull();
    expect(surface.closest(".poodle-ref-select")).toBeNull();
    expect(surface.dataset.poodleAnchored).toBe("true");

    // A click inside the portalled surface must not read as "outside".
    fireEvent.mouseDown(surface.querySelector(".poodle-ref-select__list")!);
    expect(surfaceOf(container)).not.toBeNull();
  });

  it("marks the checked-out ref independently of the selected one", () => {
    const { container } = render(<RefSelect refs={refs} value="tree-component" currentRef="main" />);
    expect(triggerOf(container).getAttribute("aria-label")).toBe("Ref: tree-component");

    fireEvent.click(triggerOf(container));
    const rows = rowsOf(container);
    expect(rows[0].dataset.current).toBe("true");
    expect(rows[0].dataset.selected).toBe("false");
    expect(rows[1].dataset.selected).toBe("true");
    expect(rows[1].dataset.current).toBe("false");
    expect(
      surfaceOf(container)?.querySelector(".poodle-ref-select__option-marker")?.textContent?.trim(),
    ).toBe("current");
  });

  it("filters locally across label, value and description", () => {
    const { container } = render(<RefSelect refs={refs} value="main" />);
    fireEvent.click(triggerOf(container));
    const search = surfaceOf(container)!.querySelector("input") as HTMLInputElement;

    fireEvent.input(search, { target: { value: "tree" } });
    expect(rowsOf(container)).toHaveLength(1);

    // A sha typed from a commit line finds its row.
    fireEvent.input(search, { target: { value: "a1b2" } });
    expect(rowsOf(container).map((row) => row.textContent)).toHaveLength(1);

    fireEvent.input(search, { target: { value: "nothing" } });
    expect(rowsOf(container)).toHaveLength(0);
    expect(surfaceOf(container)?.querySelector(".poodle-ref-select__empty")).not.toBeNull();
  });

  it("hands filtering to the host when searchValue is supplied", () => {
    const onSearchChange = vi.fn();
    const { container } = render(
      <RefSelect refs={refs} value="main" searchValue="zzz" onSearchChange={onSearchChange} />,
    );
    fireEvent.click(triggerOf(container));
    // The host already filtered: every passed ref renders, whatever the query.
    expect(rowsOf(container)).toHaveLength(3);

    fireEvent.input(surfaceOf(container)!.querySelector("input") as HTMLInputElement, {
      target: { value: "ma" },
    });
    expect(onSearchChange).toHaveBeenCalledWith("ma");
  });

  it("loading suppresses the empty message", () => {
    const { container } = render(<RefSelect refs={[]} loading />);
    fireEvent.click(triggerOf(container));
    const surface = surfaceOf(container)!;
    expect(surface.querySelector(".poodle-ref-select__empty")).toBeNull();
    const loading = surface.querySelector(".poodle-ref-select__loading") as HTMLElement;
    expect(loading.getAttribute("role")).toBe("status");
  });

  it("choosing a ref closes the popover and reports it", () => {
    const onChange = vi.fn();
    const { container } = render(
      <RefSelect refs={refs} value="main" currentRef="main" onChange={onChange} />,
    );
    fireEvent.click(triggerOf(container));
    fireEvent.click(rowsOf(container)[1]);

    expect(onChange).toHaveBeenCalledWith("tree-component");
    expect(surfaceOf(container)).toBeNull();
  });

  it("emits group headings once per run and hides the search when asked", () => {
    const { container } = render(<RefSelect refs={refs} value="main" searchable={false} />);
    fireEvent.click(triggerOf(container));
    const surface = surfaceOf(container)!;
    expect(surface.querySelector("input")).toBeNull();
    const groups = Array.from(surface.querySelectorAll(".poodle-ref-select__group")).map((g) =>
      g.textContent?.trim(),
    );
    expect(groups).toEqual(["Branches", "Tags"]);
  });
});

describe("RefSelect (react) dismissOnOutsideInteract", () => {
  const triggerOf = (container: HTMLElement) =>
    container.querySelector(".poodle-ref-select__trigger") as HTMLButtonElement;
  const surfaceOf = (container: HTMLElement) =>
    document.getElementById(
      triggerOf(container).getAttribute("aria-controls") ?? "",
    ) as HTMLElement | null;

  it("dismisses the surface on outside mousedown by default", async () => {
    const { container } = render(<RefSelect refs={refs} value="main" />);
    await fireEvent.click(triggerOf(container));
    expect(surfaceOf(container)).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(surfaceOf(container)).toBeNull();
  });

  it("keeps the surface open on outside mousedown when dismissOnOutsideInteract=false", async () => {
    const { container } = render(
      <RefSelect refs={refs} value="main" dismissOnOutsideInteract={false} />,
    );
    await fireEvent.click(triggerOf(container));
    expect(surfaceOf(container)).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(surfaceOf(container)).not.toBeNull();
  });
});
