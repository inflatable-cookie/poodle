import { cleanup, render } from "@testing-library/svelte";
import { afterEach, describe, expect, it } from "vitest";

import MotionPolicyProvider from "../src/MotionPolicyProvider.svelte";
import MotionPolicyHarness from "./MotionPolicyHarness.svelte";
import MotionPolicyReader from "./MotionPolicyReader.svelte";
import { asSnippet } from "./snippet";

describe("MotionPolicyProvider (svelte)", () => {
  afterEach(() => {
    cleanup();
  });

  it("omitted root policy resolves to full", () => {
    const { container } = render(MotionPolicyProvider, {
      props: { children: asSnippet(() => "content") },
    });
    const root = container.querySelector(".poodle-motion-policy-provider") as HTMLElement;
    expect(root.getAttribute("data-poodle-motion-policy")).toBe("full");
    expect(root.className).toBe("poodle-motion-policy-provider");
    expect(root.getAttribute("role")).toBeNull();
    expect(Array.from(root.attributes).map((a) => a.name).sort()).toEqual([
      "class",
      "data-poodle-motion-policy",
    ]);
    const { getByTestId } = render(MotionPolicyReader, { props: {} });
    expect(getByTestId("effective-motion").textContent).toBe("full");
  });

  it("a child full request cannot re-enable reduced", () => {
    const { getByTestId } = render(MotionPolicyHarness, {
      props: { policy: "reduced", nestedPolicy: "full" },
    });
    expect(getByTestId("effective-motion").textContent).toBe("reduced");
  });

  it("a child reduced request under frozen stays frozen", () => {
    const { getByTestId } = render(MotionPolicyHarness, {
      props: { policy: "frozen", nestedPolicy: "reduced" },
    });
    expect(getByTestId("effective-motion").textContent).toBe("frozen");
  });

  it("a child can tighten full to reduced", () => {
    const { getByTestId } = render(MotionPolicyHarness, {
      props: { policy: "full", nestedPolicy: "reduced" },
    });
    expect(getByTestId("effective-motion").textContent).toBe("reduced");
  });

  it("presentation scopes preserve motion", () => {
    const { getByTestId } = render(MotionPolicyHarness, {
      props: { policy: "reduced", nestedPolicy: "full", wrapPresentation: true },
    });
    expect(getByTestId("effective-motion").textContent).toBe("reduced");
  });
});
