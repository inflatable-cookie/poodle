import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { RangeSlider } from "../src/RangeSlider";
import { rangeSliderDefinition } from "../src/generated/range-slider";

// Card 045 R2: the DOM reads the generated artifact — the data-* attribute
// names, the part class names, and the fill-geometry custom properties come
// from range_slider.rs via `range-slider-ts`, never from hand-written
// literals in this component. A rename in the definition moves the DOM;
// `effigy ir:check` gates drift in the artifact.

function attributeName(id: string): string {
  const attribute = rangeSliderDefinition.attributes.find((entry) => entry.id === id);
  if (!attribute) throw new Error(`definition lacks attribute '${id}'`);
  return attribute.name;
}

function partClass(id: string): string {
  const part = rangeSliderDefinition.parts.find((entry) => entry.id === id);
  if (!part) throw new Error(`definition lacks part '${id}'`);
  return part.className;
}

// Part classes can be space-joined (base + modifier, e.g.
// "poodle-range-slider__fill poodle-range-slider__fill--negative"); the
// modifier token is the unique selector for that part.
function partSelector(id: string): string {
  const classes = partClass(id).split(" ");
  return `.${classes[classes.length - 1]}`;
}

function stylePropName(id: string): string {
  const prop = rangeSliderDefinition.styleProps.find((entry) => entry.id === id);
  if (!prop) throw new Error(`definition lacks style prop '${id}'`);
  return prop.name;
}

describe("RangeSlider (react) — generated definition drives the DOM", () => {
  it("emits the eight data attributes under the definition's names and values", () => {
    const { container } = render(<RangeSlider value={[20, 80]} ariaLabel="Price range" />);
    const el = container.querySelector(partSelector("root")) as HTMLElement;

    expect(el.getAttribute(attributeName("orientation"))).toBe("horizontal");
    expect(el.getAttribute(attributeName("disabled"))).toBe("false");
    expect(el.getAttribute(attributeName("variant"))).toBe("standard");
    expect(el.getAttribute(attributeName("polarity"))).toBe("unipolar");
    expect(el.getAttribute(attributeName("fill-split"))).toBe("false");
    expect(el.getAttribute(attributeName("state"))).toBe("idle");
    expect(el.getAttribute(attributeName("size"))).toBe("md");
    expect(el.getAttribute(attributeName("density"))).toBe("default");
    // The definition's attribute entries are what the DOM carries — the
    // names in this test came from the artifact itself.
    expect(attributeName("orientation")).toBe("data-orientation");
    expect(attributeName("polarity")).toBe("data-polarity");
    expect(attributeName("fill-split")).toBe("data-fill-split");
    expect(attributeName("state")).toBe("data-state");
  });

  it("renders the anatomy under the definition's part classes", () => {
    const { container } = render(<RangeSlider />);
    const root = container.querySelector(partSelector("root")) as HTMLElement;

    expect(root.querySelector(partSelector("track"))).not.toBeNull();
    const track = root.querySelector(partSelector("track")) as HTMLElement;
    expect(track.querySelector(partSelector("fill-negative"))).not.toBeNull();
    expect(track.querySelector(partSelector("fill-positive"))).not.toBeNull();
    expect(track.querySelector(partSelector("center"))).not.toBeNull();
  });

  it("renders two thumb controls under the definition's classes per variant", () => {
    const standard = render(<RangeSlider />);
    const standardRoot = standard.container.querySelector(partSelector("root")) as HTMLElement;
    const lower = standardRoot.querySelector(partSelector("control-lower"));
    const upper = standardRoot.querySelector(partSelector("control-upper"));
    expect(lower).not.toBeNull();
    expect(upper).not.toBeNull();
    expect((lower as HTMLInputElement).type).toBe("range");
    expect((upper as HTMLInputElement).type).toBe("range");
    standard.unmount();

    const embedded = render(<RangeSlider variant="embedded" />);
    const embeddedRoot = embedded.container.querySelector(partSelector("root")) as HTMLElement;
    expect(embeddedRoot.querySelector(partSelector("embedded-lower"))).not.toBeNull();
    expect(embeddedRoot.querySelector(partSelector("embedded-upper"))).not.toBeNull();
    expect(embeddedRoot.querySelector(partSelector("control-lower"))).toBeNull();
    embedded.unmount();
  });

  it("emits the fill-geometry custom properties under the definition's names", () => {
    const { container } = render(<RangeSlider value={[20, 80]} min={0} max={100} />);
    const el = container.querySelector(partSelector("root")) as HTMLElement;
    const style = el.getAttribute("style") ?? "";

    expect(style).toContain(`${stylePropName("range-start")}: 20%`);
    expect(style).toContain(`${stylePropName("range-end")}: 80%`);
    expect(style).toContain(stylePropName("range-center"));
    expect(style).toContain(stylePropName("range-negative-start"));
    expect(style).toContain(stylePropName("range-negative-span"));
    expect(style).toContain(stylePropName("range-positive-start"));
    expect(style).toContain(stylePropName("range-positive-span"));
    expect(stylePropName("range-start")).toBe("--poodle-range-start");
  });

  it("splits the bipolar fill at the center and reports the split state", () => {
    const { container } = render(
      <RangeSlider value={[-0.6, 0.35]} min={-1} max={1} variant="embedded" polarity="bipolar" />,
    );
    const el = container.querySelector(partSelector("root")) as HTMLElement;
    expect(el.getAttribute(attributeName("polarity"))).toBe("bipolar");
    expect(el.getAttribute(attributeName("fill-split"))).toBe("true");
  });
});
