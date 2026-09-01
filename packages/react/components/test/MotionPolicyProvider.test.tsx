import { cleanup, render } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import { MotionPolicyProvider, UiPresentationProvider, useMotionPolicy } from "../src";

function Reader() {
  const policy = useMotionPolicy();
  return <span data-testid="effective-motion">{policy}</span>;
}

describe("MotionPolicyProvider (react)", () => {
  afterEach(() => {
    cleanup();
  });

  it("omitted root policy resolves to full", () => {
    const { container, getByTestId } = render(
      <MotionPolicyProvider>
        <Reader />
      </MotionPolicyProvider>,
    );
    const root = container.querySelector(".poodle-motion-policy-provider") as HTMLElement;
    expect(root.getAttribute("data-poodle-motion-policy")).toBe("full");
    expect(root.className).toBe("poodle-motion-policy-provider");
    expect(root.getAttribute("role")).toBeNull();
    expect(Array.from(root.attributes).map((a) => a.name).sort()).toEqual([
      "class",
      "data-poodle-motion-policy",
    ]);
    expect(getByTestId("effective-motion").textContent).toBe("full");
  });

  it("missing provider resolves to full", () => {
    const { getByTestId } = render(<Reader />);
    expect(getByTestId("effective-motion").textContent).toBe("full");
  });

  it("a child full request cannot re-enable reduced", () => {
    const { getByTestId } = render(
      <MotionPolicyProvider policy="reduced">
        <MotionPolicyProvider policy="full">
          <Reader />
        </MotionPolicyProvider>
      </MotionPolicyProvider>,
    );
    expect(getByTestId("effective-motion").textContent).toBe("reduced");
  });

  it("a child reduced request under frozen stays frozen", () => {
    const { getByTestId } = render(
      <MotionPolicyProvider policy="frozen">
        <MotionPolicyProvider policy="reduced">
          <Reader />
        </MotionPolicyProvider>
      </MotionPolicyProvider>,
    );
    expect(getByTestId("effective-motion").textContent).toBe("frozen");
  });

  it("presentation scopes preserve motion", () => {
    const { getByTestId } = render(
      <MotionPolicyProvider policy="reduced">
        <UiPresentationProvider sizeScale="xl" density="comfortable">
          <MotionPolicyProvider policy="full">
            <Reader />
          </MotionPolicyProvider>
        </UiPresentationProvider>
      </MotionPolicyProvider>,
    );
    expect(getByTestId("effective-motion").textContent).toBe("reduced");
  });
});
