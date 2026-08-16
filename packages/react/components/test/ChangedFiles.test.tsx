import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { ChangedFiles } from "../src/ChangedFiles";

const files = [
  { path: "cp-api/crates/latex/src/parser.rs", additions: 320, deletions: 12 },
  { path: "cp-api/src/main.rs", additions: 56, deletions: 4 },
  { path: "docs/README.md", additions: 10, deletions: 0 },
  { path: "docs/guide.md", additions: 1, deletions: 1 },
];

describe("ChangedFiles (react)", () => {
  it("renders the header totals with a worded accessible name", () => {
    const { container } = render(<ChangedFiles id="cf-1" files={files} />);
    const root = container.querySelector(".poodle-changed-files") as HTMLElement;
    expect(root.dataset.fileCount).toBe("4");
    const toggle = container.querySelector(".poodle-changed-files__toggle") as HTMLButtonElement;
    expect(toggle.getAttribute("aria-label")).toBe("4 changed files, 387 added, 17 removed");
    expect(toggle.querySelector(".poodle-changed-files__count")?.textContent).toBe(
      "4 changed files",
    );
    expect(toggle.querySelector(".poodle-changed-files__additions")?.textContent).toBe("+387");
    expect(toggle.querySelector(".poodle-changed-files__deletions")?.textContent).toBe("−17");
  });

  it("renders scopes and chips while collapsed, up to the chip limit", () => {
    const { container } = render(<ChangedFiles id="cf-1" files={files} chipLimit={2} />);
    expect(container.querySelectorAll(".poodle-changed-files__chip").length).toBe(2);
    expect(container.querySelector(".poodle-changed-files__scopes")?.textContent).toContain("cp-api");
    expect(container.querySelector(".poodle-changed-files__more")?.textContent).toBe(
      "Show all 4 files",
    );
    expect(container.querySelector(".poodle-changed-files__tree")).toBeNull();
  });

  it("reports file selection from a chip", () => {
    const onFileSelect = vi.fn();
    const { container } = render(
      <ChangedFiles id="cf-1" files={files} chipLimit={1} onFileSelect={onFileSelect} />,
    );
    const chip = container.querySelector(".poodle-changed-files__chip") as HTMLButtonElement;
    fireEvent.click(chip);
    expect(onFileSelect).toHaveBeenCalledWith("cp-api/crates/latex/src/parser.rs");
  });

  it("expands into the directory tree and reports the toggle", () => {
    const onToggle = vi.fn();
    const { container } = render(<ChangedFiles id="cf-1" files={files} onToggle={onToggle} />);
    fireEvent.click(container.querySelector(".poodle-changed-files__toggle") as HTMLElement);

    expect(onToggle).toHaveBeenCalledWith("cf-1");
    expect(container.querySelector(".poodle-changed-files__tree")).not.toBeNull();
    expect(container.querySelector(".poodle-changed-files__tree [role='tree']")).not.toBeNull();
    expect(container.querySelector(".poodle-changed-files__summary")).toBeNull();
  });

  it("renders the open-diff action when enabled and omits it when disabled", () => {
    const onOpenDiff = vi.fn();
    const { container } = render(<ChangedFiles id="cf-1" files={files} onOpenDiff={onOpenDiff} />);
    const diff = container.querySelector(".poodle-changed-files__actions button") as HTMLElement;
    expect(diff.textContent?.trim()).toBe("Open diff");
    fireEvent.click(diff);
    expect(onOpenDiff).toHaveBeenCalledWith("cf-1");

    const without = render(<ChangedFiles id="cf-1" files={files} showOpenDiff={false} />);
    expect(without.container.querySelector(".poodle-changed-files__actions")).toBeNull();
  });

  it("renders nothing at all for an empty file list", () => {
    const { container } = render(<ChangedFiles id="cf-1" files={[]} />);
    expect(container.querySelector(".poodle-changed-files")).toBeNull();
  });
});