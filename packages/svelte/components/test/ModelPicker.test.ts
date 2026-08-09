import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import ModelPicker from "../src/ModelPicker.svelte";
import type { ModelCapabilityAxis, ModelOption } from "../src/types.ts";

// The picker's contract is per-model axis resolution: axes are declared once by
// key, each model references the keys it exposes, and a binding may override the
// level set for that model. Cross-provider lists depend on both halves.
const models: ModelOption[] = [
  { value: "pro", label: "Pro", group: "Frontier", axes: ["effort", "context"] },
  { value: "mini", label: "Mini", group: "Frontier", axes: ["effort"] },
  {
    value: "other",
    label: "Other",
    group: "Frontier",
    axes: [
      {
        key: "effort",
        options: [
          { value: "minimal", label: "Minimal" },
          { value: "deep", label: "Deep" },
        ],
        defaultValue: "minimal",
      },
    ],
  },
];

const axes: ModelCapabilityAxis[] = [
  {
    key: "effort",
    label: "Effort",
    kind: "select",
    options: [
      { value: "low", label: "Low" },
      { value: "high", label: "High" },
    ],
    defaultValue: "low",
  },
  {
    key: "context",
    label: "Context window",
    kind: "select",
    options: [
      { value: "200k", label: "200K" },
      { value: "1m", label: "1M" },
    ],
    defaultValue: "1m",
  },
];

describe("ModelPicker (svelte)", () => {
  const triggerOf = (container: HTMLElement) =>
    container.querySelector(".poodle-model-picker__trigger") as HTMLButtonElement;

  // The surface is portalled to the theme root, so it is not reachable from the
  // render container. `aria-controls` is the link back — and going through it
  // keeps concurrently-rendered pickers apart.
  const surfaceOf = (container: HTMLElement) =>
    document.getElementById(
      triggerOf(container).getAttribute("aria-controls") ?? "",
    ) as HTMLElement;

  it("portals its surface out of the trigger's subtree", async () => {
    const { container } = render(ModelPicker, {
      props: { models, axes, value: { model: "pro", axes: {} } },
    });
    await fireEvent.click(triggerOf(container));

    const surface = surfaceOf(container);
    expect(surface).not.toBeNull();
    // Escaping the subtree is the whole point: no ancestor of the trigger can
    // clip it or trap it in a stacking context.
    expect(container.querySelector(".poodle-model-picker__surface")).toBeNull();
    expect(surface.closest(".poodle-model-picker")).toBeNull();
    expect(surface.dataset.poodleAnchored).toBe("true");
  });

  it("keeps the portalled surface inside its dismiss layer", async () => {
    const { container } = render(ModelPicker, {
      props: { models, axes, value: { model: "pro", axes: {} } },
    });
    await fireEvent.click(triggerOf(container));

    // A click in the surface is no longer a DOM descendant of the trigger, so
    // without `layerContains` it would read as an outside interaction and close.
    await fireEvent.mouseDown(
      surfaceOf(container).querySelector(".poodle-model-picker__option")!,
    );
    expect(surfaceOf(container)).not.toBeNull();
  });

  it("summarises the axes the selected model exposes", () => {
    const { container } = render(ModelPicker, {
      props: { models, axes, value: { model: "pro", axes: { effort: "high", context: "1m" } } },
    });
    const summary = container.querySelector(".poodle-model-picker__summary") as HTMLElement;
    expect(summary.textContent?.trim()).toBe("High · 1M");
    expect(triggerOf(container).getAttribute("aria-label")).toBe("Model: Pro, High · 1M");
  });

  it("drops an axis the model does not reference", () => {
    const { container } = render(ModelPicker, {
      props: { models, axes, value: { model: "mini", axes: { effort: "high", context: "1m" } } },
    });
    expect(
      (container.querySelector(".poodle-model-picker__summary") as HTMLElement).textContent?.trim(),
    ).toBe("High");
  });

  it("applies a per-model binding: same key, different levels", async () => {
    const { container } = render(ModelPicker, {
      props: { models, axes, value: { model: "other", axes: {} } },
    });
    // The binding's option set replaced the shared one; the label is inherited.
    expect(
      (container.querySelector(".poodle-model-picker__summary") as HTMLElement).textContent?.trim(),
    ).toBe("Minimal");

    await fireEvent.click(triggerOf(container));
    const surface = surfaceOf(container);
    expect(
      (surface.querySelector(".poodle-model-picker__axis-label") as HTMLElement).textContent,
    ).toBe("Effort");
    const labels = Array.from(
      surface.querySelectorAll(
        ".poodle-segmented-control__label, .poodle-segmented-control__option-label",
      ),
    ).map((node) => node.textContent?.trim());
    expect(labels.join("|")).toContain("Minimal");
  });

  it("normalises the emitted selection: another provider's level never leaks", async () => {
    const onChange = vi.fn();
    const { container } = render(ModelPicker, {
      props: {
        models,
        axes,
        // `high` belongs to the shared scale, not to `other`'s binding.
        value: { model: "pro", axes: { effort: "high", context: "1m" } },
        onChange,
      },
    });

    await fireEvent.click(triggerOf(container));
    const options = surfaceOf(container).querySelectorAll<HTMLButtonElement>(
      ".poodle-model-picker__option",
    );
    await fireEvent.click(options[2]);

    expect(onChange).toHaveBeenCalledTimes(1);
    // `context` is not referenced by `other`, and `high` is not one of its levels.
    expect(onChange.mock.calls[0][0]).toEqual({ model: "other", axes: { effort: "minimal" } });
  });

  it("a model with an empty axis list exposes none", async () => {
    const bare: ModelOption[] = [{ value: "bare", label: "Bare", axes: [] }];
    const { container } = render(ModelPicker, {
      props: { models: bare, axes, value: { model: "bare", axes: {} } },
    });
    expect(container.querySelector(".poodle-model-picker__summary")).toBeNull();
    await fireEvent.click(triggerOf(container));
    const surface = surfaceOf(container);
    expect(surface.querySelector(".poodle-model-picker__axes")).toBeNull();
    expect(surface.dataset.layout).toBe("single");
  });

  it("a model with no axis declaration inherits every axis", async () => {
    const inherits: ModelOption[] = [{ value: "all", label: "All" }];
    const { container } = render(ModelPicker, {
      props: { models: inherits, axes, value: { model: "all", axes: {} } },
    });
    await fireEvent.click(triggerOf(container));
    expect(surfaceOf(container).querySelectorAll(".poodle-model-picker__axis")).toHaveLength(2);
  });

  it("renders an arbitrary image in place of a default Lucide icon", async () => {
    const withImage: ModelOption[] = [
      { value: "logo", label: "Logo", icon: "star", image: { src: "/m.svg", alt: "Mark" } },
      { value: "icon-only", label: "Icon only", icon: "star" },
    ];
    const { container } = render(ModelPicker, {
      props: { models: withImage, axes, value: { model: "logo", axes: {} } },
    });

    // Trigger: image wins over the icon name when a model sets both.
    const triggerImage = container.querySelector(
      ".poodle-model-picker__image",
    ) as HTMLImageElement;
    expect(triggerImage.getAttribute("src")).toBe("/m.svg");
    expect(triggerImage.getAttribute("alt")).toBe("Mark");
    expect(container.querySelector(".poodle-model-picker__icon .poodle-icon")).toBeNull();

    await fireEvent.click(triggerOf(container));
    const rows = surfaceOf(container).querySelectorAll(".poodle-model-picker__option");
    expect(rows[0].querySelector(".poodle-model-picker__option-image")).not.toBeNull();
    // The icon-only model still renders the default Lucide icon.
    expect(rows[1].querySelector(".poodle-model-picker__option-image")).toBeNull();
    expect(rows[1].querySelector(".poodle-model-picker__option-icon")).not.toBeNull();
  });

  it("defaults image alt to empty (the label sits beside it)", () => {
    const decorative: ModelOption[] = [
      { value: "logo", label: "Logo", image: { src: "/m.svg" } },
    ];
    const { container } = render(ModelPicker, {
      props: { models: decorative, axes, value: { model: "logo", axes: {} } },
    });
    expect(
      (container.querySelector(".poodle-model-picker__image") as HTMLImageElement).getAttribute(
        "alt",
      ),
    ).toBe("");
  });

  it("carries the emphasis axis for embedding beside a louder control", () => {
    const plain = render(ModelPicker, {
      props: { models, axes, value: { model: "pro", axes: {} } },
    });
    expect(
      (plain.container.querySelector(".poodle-model-picker") as HTMLElement).dataset.emphasis,
    ).toBe("default");

    const quiet = render(ModelPicker, {
      props: { models, axes, value: { model: "pro", axes: {} }, emphasis: "subdued" as const },
    });
    expect(
      (quiet.container.querySelector(".poodle-model-picker") as HTMLElement).dataset.emphasis,
    ).toBe("subdued");
    // Emphasis is presentation only — the trigger still reads the same.
    expect(
      quiet.container.querySelector(".poodle-model-picker__trigger")?.getAttribute("aria-label"),
    ).toBe(
      plain.container.querySelector(".poodle-model-picker__trigger")?.getAttribute("aria-label"),
    );
  });

  it("shows the placeholder and group headings", async () => {
    const { container } = render(ModelPicker, {
      props: { models, axes, value: { model: "", axes: {} } },
    });
    const label = container.querySelector(".poodle-model-picker__label") as HTMLElement;
    expect(label.dataset.placeholder).toBe("true");
    expect(label.textContent?.trim()).toBe("Select model");

    await fireEvent.click(triggerOf(container));
    const groups = surfaceOf(container).querySelectorAll(".poodle-model-picker__group");
    // All three models share one group, so the heading is emitted once.
    expect(groups).toHaveLength(1);
    expect(groups[0].textContent?.trim()).toBe("Frontier");
  });

  it("renders a many-level select axis as a list, and honours the control hint", async () => {
    const levels = ["a", "b", "c", "d", "e", "f", "g"].map((value) => ({ value, label: value }));
    const many = render(ModelPicker, {
      props: {
        models,
        axes: [{ key: "effort", label: "Effort", kind: "select", options: levels }],
        value: { model: "mini", axes: { effort: "c" } },
      },
    });
    await fireEvent.click(triggerOf(many.container));
    const manySurface = surfaceOf(many.container);
    const axis = manySurface.querySelector(".poodle-model-picker__axis") as HTMLElement;
    expect(axis.dataset.control).toBe("list");
    expect(manySurface.querySelectorAll(".poodle-model-picker__axis-option")).toHaveLength(7);
    expect(manySurface.querySelector(".poodle-segmented-control")).toBeNull();

    // Two options stay segmented…
    const short = render(ModelPicker, {
      props: { models, axes, value: { model: "mini", axes: {} } },
    });
    await fireEvent.click(triggerOf(short.container));
    expect(
      (surfaceOf(short.container).querySelector(".poodle-model-picker__axis") as HTMLElement).dataset
        .control,
    ).toBe("segmented");

    // …unless the host says otherwise.
    const forced = render(ModelPicker, {
      props: {
        models,
        axes: [{ ...axes[0], control: "list" as const }],
        value: { model: "mini", axes: {} },
      },
    });
    await fireEvent.click(triggerOf(forced.container));
    expect(
      (surfaceOf(forced.container).querySelector(".poodle-model-picker__axis") as HTMLElement)
        .dataset.control,
    ).toBe("list");
  });

  it("splits into models | axes columns only when axes apply", async () => {
    const { container } = render(ModelPicker, {
      props: { models, axes, value: { model: "pro", axes: {} } },
    });
    await fireEvent.click(triggerOf(container));
    const surface = surfaceOf(container);
    expect(surface.dataset.layout).toBe("split");
    expect(surface.querySelectorAll(".poodle-model-picker__axes")).toHaveLength(1);

    // No axes declared → single column, no axes rail.
    const plain = render(ModelPicker, { props: { models, value: { model: "pro", axes: {} } } });
    await fireEvent.click(triggerOf(plain.container));
    const plainSurface = surfaceOf(plain.container);
    expect(plainSurface.dataset.layout).toBe("single");
    expect(plainSurface.querySelector(".poodle-model-picker__axes")).toBeNull();
  });

  it("stays open after selecting a model so the axes can still be edited", async () => {
    const { container } = render(ModelPicker, {
      props: { models, axes, value: { model: "pro", axes: {} } },
    });
    await fireEvent.click(triggerOf(container));
    expect(surfaceOf(container)).not.toBeNull();

    await fireEvent.click(
      surfaceOf(container).querySelectorAll<HTMLButtonElement>(".poodle-model-picker__option")[1],
    );
    expect(surfaceOf(container)).not.toBeNull();
    expect(surfaceOf(container).querySelector(".poodle-model-picker__axes")).not.toBeNull();
  });
});
