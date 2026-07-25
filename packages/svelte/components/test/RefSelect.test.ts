import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import RefSelect from "../src/RefSelect.svelte";
import type { RefOption } from "../src/types";

// The component's contract is filtering, the current marker, and the fact that
// a host-supplied query hands filtering over entirely.
const refs: RefOption[] = [
  { value: "main", label: "main", kind: "branch", description: "a1b2c3d", group: "Branches" },
  { value: "tree-component", label: "tree-component", kind: "branch", group: "Branches" },
  { value: "v1.4.0", label: "v1.4.0", kind: "tag", group: "Tags" },
];

describe("RefSelect (svelte)", () => {
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

  it("portals its surface out of the trigger's subtree", async () => {
    const { container } = render(RefSelect, { props: { refs, value: "main" } });
    await fireEvent.click(triggerOf(container));

    const surface = surfaceOf(container)!;
    expect(container.querySelector(".poodle-ref-select__surface")).toBeNull();
    expect(surface.closest(".poodle-ref-select")).toBeNull();
    expect(surface.dataset.poodleAnchored).toBe("true");

    // A click inside the portalled surface must not read as "outside".
    await fireEvent.mouseDown(surface.querySelector(".poodle-ref-select__list")!);
    expect(surfaceOf(container)).not.toBeNull();
  });

  it("marks the checked-out ref independently of the selected one", async () => {
    const { container } = render(RefSelect, {
      props: { refs, value: "tree-component", currentRef: "main" },
    });
    expect(triggerOf(container).getAttribute("aria-label")).toBe("Ref: tree-component");

    await fireEvent.click(triggerOf(container));
    const rows = rowsOf(container);
    expect(rows[0].dataset.current).toBe("true");
    expect(rows[0].dataset.selected).toBe("false");
    expect(rows[1].dataset.selected).toBe("true");
    expect(rows[1].dataset.current).toBe("false");
    expect(
      surfaceOf(container)?.querySelector(".poodle-ref-select__option-marker")?.textContent?.trim(),
    ).toBe("current");
  });

  it("filters locally across label, value and description", async () => {
    const { container } = render(RefSelect, { props: { refs, value: "main" } });
    await fireEvent.click(triggerOf(container));
    const search = surfaceOf(container)!.querySelector("input") as HTMLInputElement;

    await fireEvent.input(search, { target: { value: "tree" } });
    expect(rowsOf(container)).toHaveLength(1);

    // A sha typed from a commit line finds its row.
    await fireEvent.input(search, { target: { value: "a1b2" } });
    expect(rowsOf(container).map((row) => row.textContent)).toHaveLength(1);

    await fireEvent.input(search, { target: { value: "nothing" } });
    expect(rowsOf(container)).toHaveLength(0);
    expect(surfaceOf(container)?.querySelector(".poodle-ref-select__empty")).not.toBeNull();
  });

  it("hands filtering to the host when searchValue is supplied", async () => {
    const onSearchChange = vi.fn();
    const { container } = render(RefSelect, {
      props: { refs, value: "main", searchValue: "zzz", onSearchChange },
    });
    await fireEvent.click(triggerOf(container));
    // The host already filtered: every passed ref renders, whatever the query.
    expect(rowsOf(container)).toHaveLength(3);

    await fireEvent.input(surfaceOf(container)!.querySelector("input") as HTMLInputElement, {
      target: { value: "ma" },
    });
    expect(onSearchChange).toHaveBeenCalledWith("ma");
  });

  it("loading suppresses the empty message", async () => {
    const { container } = render(RefSelect, { props: { refs: [], loading: true } });
    await fireEvent.click(triggerOf(container));
    const surface = surfaceOf(container)!;
    expect(surface.querySelector(".poodle-ref-select__empty")).toBeNull();
    const loading = surface.querySelector(".poodle-ref-select__loading") as HTMLElement;
    expect(loading.getAttribute("role")).toBe("status");
  });

  it("choosing a ref closes the popover and reports it", async () => {
    const onChange = vi.fn();
    const { container } = render(RefSelect, {
      props: { refs, value: "main", currentRef: "main", onChange },
    });
    await fireEvent.click(triggerOf(container));
    await fireEvent.click(rowsOf(container)[1]);

    expect(onChange).toHaveBeenCalledWith("tree-component");
    expect(surfaceOf(container)).toBeNull();
  });

  it("emits group headings once per run and hides the search when asked", async () => {
    const { container } = render(RefSelect, {
      props: { refs, value: "main", searchable: false },
    });
    await fireEvent.click(triggerOf(container));
    const surface = surfaceOf(container)!;
    expect(surface.querySelector("input")).toBeNull();
    const groups = Array.from(surface.querySelectorAll(".poodle-ref-select__group")).map((g) =>
      g.textContent?.trim(),
    );
    expect(groups).toEqual(["Branches", "Tags"]);
  });
});
