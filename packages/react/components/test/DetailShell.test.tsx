import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { DetailShell } from "../src/DetailShell";

describe("DetailShell (react)", () => {
  it("renders the body region when ready and shows the title fallback header", () => {
    const { container } = render(<DetailShell title="Member">content</DetailShell>);
    expect(container.querySelector(".poodle-detail-shell__body")).not.toBeNull();
    const heading = container.querySelector(".poodle-detail-shell__header h2") as HTMLElement;
    expect(heading.textContent).toBe("Member");
  });

  it("replaces the body with a loading state region showing the spinner", () => {
    const { container } = render(<DetailShell title="Member" state="loading" />);
    expect(container.querySelector(".poodle-detail-shell__body")).toBeNull();
    const stateRegion = container.querySelector(".poodle-detail-shell__state") as HTMLElement;
    expect(stateRegion.dataset.state).toBe("loading");
    expect(stateRegion.querySelector(".poodle-detail-shell__spinner")).not.toBeNull();
  });

  it("renders the state title and message in the error state", () => {
    const { container } = render(
      <DetailShell title="Member" state="error" stateTitle="Failed to load" stateMessage="Try again." />,
    );
    const stateRegion = container.querySelector(".poodle-detail-shell__state") as HTMLElement;
    expect(stateRegion.dataset.state).toBe("error");
    expect(stateRegion.textContent).toContain("Failed to load");
    expect(stateRegion.textContent).toContain("Try again.");
  });

  it("uses the stateContent snippet when provided", () => {
    const { container } = render(
      <DetailShell title="Member" state="error" stateContent={<p>custom</p>} />,
    );
    const stateRegion = container.querySelector(".poodle-detail-shell__state") as HTMLElement;
    expect(stateRegion.textContent).toContain("custom");
  });

  it("projects the scroll mode and applies the accessible label", () => {
    const { container } = render(
      <DetailShell scrollMode="shell" ariaLabel="Member detail">
        content
      </DetailShell>,
    );
    const root = container.querySelector(".poodle-detail-shell") as HTMLElement;
    expect(root.dataset.scrollMode).toBe("shell");
    expect(root.getAttribute("aria-label")).toBe("Member detail");
  });
});
