import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import ThemeSelect from "../src/ThemeSelect.svelte";
import type { ThemeOption } from "../src/types";

const themes: ThemeOption[] = [
  {
    value: "light",
    label: "Light",
    swatch: { canvas: "#fff", border: "#ddd", surface: "#fff", accent: "#2563eb", text: "#111" },
  },
  {
    value: "dark",
    label: "Dark",
    swatch: { canvas: "#111", border: "#333", surface: "#111", accent: "#60a5fa", text: "#eee" },
  },
];

describe("ThemeSelect (svelte) dismissOnOutsideInteract", () => {
  const triggerOf = (container: HTMLElement) =>
    container.querySelector(".poodle-theme-select__trigger") as HTMLButtonElement;

  // The surface is portalled to the theme root; `aria-controls` is the link
  // back, matching the other anchored overlay tests.
  const surfaceOf = (container: HTMLElement) =>
    document.getElementById(
      triggerOf(container).getAttribute("aria-controls") ?? "",
    ) as HTMLElement;

  it("dismisses the panel on outside mousedown by default", async () => {
    const { container } = render(ThemeSelect, { props: { themes } });
    await fireEvent.click(triggerOf(container));
    expect(surfaceOf(container)).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(surfaceOf(container)).toBeNull();
  });

  it("keeps the panel open on outside mousedown when dismissOnOutsideInteract=false", async () => {
    const { container } = render(ThemeSelect, {
      props: { themes, dismissOnOutsideInteract: false },
    });
    await fireEvent.click(triggerOf(container));
    expect(surfaceOf(container)).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(surfaceOf(container)).not.toBeNull();
  });
});
